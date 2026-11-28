//! The guest agent of `winterbaume-lambda-vm`.
//!
//! Inside the microVM, programs talk TCP: the runtime interface client to
//! `AWS_LAMBDA_RUNTIME_API`, the AWS SDKs to `AWS_ENDPOINT_URL`.  The host
//! serves both on Unix sockets that libkrun connects to guest vsock ports.
//! This agent joins the two: for every `TCP:VSOCK` argument it listens on
//! guest `127.0.0.1:TCP` and relays each connection to host vsock port
//! `VSOCK`.
//!
//! Started by the launch script as `ready=$(agent 9001:1000 ... &)`: once
//! every listener is bound it prints `ready` and closes its stdout, which
//! ends the command substitution, and keeps relaying in the background for
//! the life of the VM.
//!
//! No libc: system calls go through rustix's raw Linux backend and origin
//! provides the program start-up.  No allocator either; connections live in
//! a fixed table.

#![no_std]
#![no_main]

use core::ffi::CStr;

use rustix::event::epoll;
use rustix::fd::{AsFd, OwnedFd};
use rustix::io::Errno;
use rustix::net::addr::{SocketAddrArg, SocketAddrLen, SocketAddrOpaque};
use rustix::net::{
    AddressFamily, Ipv4Addr, RecvFlags, SendFlags, Shutdown, SocketAddrV4, SocketFlags, SocketType,
};

/// The host's context id.
const VMADDR_CID_HOST: u32 = 2;
const MAX_LISTENERS: usize = 8;
const MAX_PAIRS: usize = 64;
const BUF: usize = 16 * 1024;
/// Event data of listener `i` is `LISTENER | i`; of a pair side, `2 * pair + side`.
const LISTENER: u64 = 1 << 40;

/// `struct sockaddr_vm`.
#[repr(C)]
struct VsockAddr {
    family: u16,
    reserved1: u16,
    port: u32,
    cid: u32,
    flags: u8,
    zero: [u8; 3],
}

// SAFETY: `VsockAddr` has the layout of `struct sockaddr_vm`, and the pointer
// and length passed to `f` describe it exactly.
unsafe impl SocketAddrArg for VsockAddr {
    unsafe fn with_sockaddr<R>(
        &self,
        f: impl FnOnce(*const SocketAddrOpaque, SocketAddrLen) -> R,
    ) -> R {
        f(
            (self as *const Self).cast(),
            core::mem::size_of::<Self>() as SocketAddrLen,
        )
    }
}

struct Buf {
    data: [u8; BUF],
    start: usize,
    end: usize,
}

impl Buf {
    const EMPTY: Self = Self {
        data: [0; BUF],
        start: 0,
        end: 0,
    };

    fn is_empty(&self) -> bool {
        self.start == self.end
    }

    fn has_space(&self) -> bool {
        self.end < BUF
    }

    fn reset(&mut self) {
        self.start = 0;
        self.end = 0;
    }
}

/// One relayed connection: guest TCP on one side, host vsock on the other.
struct Pair {
    tcp: Option<OwnedFd>,
    vsock: Option<OwnedFd>,
    connecting: bool,
    /// TCP to vsock.
    up: Buf,
    /// Vsock to TCP.
    down: Buf,
    tcp_eof: bool,
    vsock_eof: bool,
    up_shut: bool,
    down_shut: bool,
}

impl Pair {
    const FREE: Self = Self {
        tcp: None,
        vsock: None,
        connecting: false,
        up: Buf::EMPTY,
        down: Buf::EMPTY,
        tcp_eof: false,
        vsock_eof: false,
        up_shut: false,
        down_shut: false,
    };
}

struct Listener {
    fd: OwnedFd,
    vsock_port: u32,
}

/// The connection table, in static memory rather than on the stack.
static mut PAIRS: [Pair; MAX_PAIRS] = [Pair::FREE; MAX_PAIRS];

fn write_all(fd: impl AsFd, mut bytes: &[u8]) {
    while !bytes.is_empty() {
        match rustix::io::write(&fd, bytes) {
            Ok(0) | Err(_) => return,
            Ok(n) => bytes = &bytes[n..],
        }
    }
}

fn fail(message: &[u8]) -> ! {
    // SAFETY: fd 2 is the console, left open by the launch script.
    let stderr = unsafe { rustix::stdio::stderr() };
    write_all(stderr, b"winterbaume-lambda-vm-agent: ");
    write_all(stderr, message);
    write_all(stderr, b"\n");
    origin::program::exit(1)
}

fn parse_u32(s: &[u8]) -> Option<u32> {
    if s.is_empty() || s.len() > 10 {
        return None;
    }
    let mut n: u64 = 0;
    for b in s {
        if !b.is_ascii_digit() {
            return None;
        }
        n = n * 10 + u64::from(b - b'0');
    }
    u32::try_from(n).ok()
}

/// `TCP:VSOCK`.
fn parse_forward(arg: &[u8]) -> Option<(u16, u32)> {
    let colon = arg.iter().position(|b| *b == b':')?;
    let tcp = u16::try_from(parse_u32(&arg[..colon])?).ok()?;
    let vsock = parse_u32(&arg[colon + 1..])?;
    (tcp != 0).then_some((tcp, vsock))
}

/// # Safety
///
/// Called once by origin with the process arguments.
#[unsafe(no_mangle)]
unsafe fn origin_main(argc: usize, argv: *mut *mut u8, _envp: *mut *mut u8) -> i32 {
    let epoll = epoll::create(epoll::CreateFlags::CLOEXEC)
        .unwrap_or_else(|_| fail(b"cannot create an epoll instance"));
    let mut listeners: [Option<Listener>; MAX_LISTENERS] = [const { None }; MAX_LISTENERS];
    for index in 1..argc {
        // SAFETY: argv holds argc NUL-terminated strings.
        let arg = unsafe { CStr::from_ptr((*argv.add(index)).cast()) }.to_bytes();
        let Some((tcp_port, vsock_port)) = parse_forward(arg) else {
            fail(b"arguments must be TCP:VSOCK port pairs");
        };
        let slot = index - 1;
        if slot >= MAX_LISTENERS {
            fail(b"too many forwards");
        }
        let fd = listen_on(tcp_port).unwrap_or_else(|_| fail(b"cannot listen on a forwarded port"));
        epoll::add(
            &epoll,
            &fd,
            epoll::EventData::new_u64(LISTENER | slot as u64),
            epoll::EventFlags::IN,
        )
        .unwrap_or_else(|_| fail(b"cannot watch a listener"));
        listeners[slot] = Some(Listener { fd, vsock_port });
    }

    // Every port accepts connections now: end the launch script's wait.
    // SAFETY: fd 1 is this process's stdout and nothing else uses it.
    write_all(unsafe { rustix::stdio::stdout() }, b"ready\n");
    drop(unsafe { rustix::stdio::take_stdout() });

    // SAFETY: the table is only ever touched from this single thread.
    let table = &raw mut PAIRS;
    let pairs = unsafe { &mut *table };
    let mut events = [epoll::Event {
        flags: epoll::EventFlags::empty(),
        data: epoll::EventData::new_u64(0),
    }; 64];
    loop {
        let ready = match epoll::wait(&epoll, &mut events[..], None) {
            Ok(ready) => ready,
            Err(Errno::INTR) => continue,
            Err(_) => fail(b"epoll_wait failed"),
        };
        for event in &events[..ready] {
            // `epoll_event` is packed on x86_64: copy the fields out.
            let flags = event.flags;
            let data = { event.data }.u64();
            if data & LISTENER != 0 {
                if let Some(listener) = &listeners[(data & !LISTENER) as usize] {
                    accept_all(&epoll, listener, pairs);
                }
            } else {
                let index = (data / 2) as usize;
                if index < MAX_PAIRS {
                    let pair = &mut pairs[index];
                    pump(&epoll, pair, index);
                    // A side that hung up for good keeps reporting it; once
                    // what it sent has been passed on, the pair is done.
                    if flags.intersects(epoll::EventFlags::HUP | epoll::EventFlags::ERR) {
                        let drained = if data % 2 == 0 {
                            pair.up.is_empty()
                        } else {
                            pair.down.is_empty()
                        };
                        if drained && !pair.connecting {
                            close(pair);
                        }
                    }
                }
            }
        }
    }
}

fn listen_on(port: u16) -> Result<OwnedFd, Errno> {
    let fd = rustix::net::socket_with(
        AddressFamily::INET,
        SocketType::STREAM,
        SocketFlags::NONBLOCK | SocketFlags::CLOEXEC,
        None,
    )?;
    rustix::net::sockopt::set_socket_reuseaddr(&fd, true)?;
    rustix::net::bind(&fd, &SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
    rustix::net::listen(&fd, 128)?;
    Ok(fd)
}

fn accept_all(epoll: &OwnedFd, listener: &Listener, pairs: &mut [Pair; MAX_PAIRS]) {
    loop {
        let tcp = match rustix::net::accept_with(
            &listener.fd,
            SocketFlags::NONBLOCK | SocketFlags::CLOEXEC,
        ) {
            Ok(fd) => fd,
            Err(Errno::INTR) => continue,
            // AGAIN: nothing more to accept.  Anything else: try again later.
            Err(_) => return,
        };
        let Some(index) = pairs
            .iter()
            .position(|p| p.tcp.is_none() && p.vsock.is_none())
        else {
            // Table full: refuse this connection; clients retry.
            drop(tcp);
            continue;
        };
        let Ok(vsock) = connect_vsock(listener.vsock_port) else {
            drop(tcp);
            continue;
        };
        let pair = &mut pairs[index];
        *pair = Pair::FREE;
        pair.connecting = true;
        let registered = epoll::add(
            epoll,
            &tcp,
            epoll::EventData::new_u64(2 * index as u64),
            epoll::EventFlags::empty(),
        )
        .and_then(|()| {
            epoll::add(
                epoll,
                &vsock,
                epoll::EventData::new_u64(2 * index as u64 + 1),
                epoll::EventFlags::OUT,
            )
        });
        pair.tcp = Some(tcp);
        pair.vsock = Some(vsock);
        if registered.is_err() {
            close(pair);
            continue;
        }
        pump(epoll, pair, index);
    }
}

fn connect_vsock(port: u32) -> Result<OwnedFd, Errno> {
    let fd = rustix::net::socket_with(
        AddressFamily::VSOCK,
        SocketType::STREAM,
        SocketFlags::NONBLOCK | SocketFlags::CLOEXEC,
        None,
    )?;
    let addr = VsockAddr {
        family: AddressFamily::VSOCK.as_raw(),
        reserved1: 0,
        port,
        cid: VMADDR_CID_HOST,
        flags: 0,
        zero: [0; 3],
    };
    match rustix::net::connect(&fd, &addr) {
        Ok(()) | Err(Errno::INPROGRESS) | Err(Errno::INTR) => Ok(fd),
        Err(e) => Err(e),
    }
}

fn close(pair: &mut Pair) {
    // Closing a descriptor removes it from the epoll set.
    pair.tcp = None;
    pair.vsock = None;
    pair.up.reset();
    pair.down.reset();
}

enum Io {
    Progress,
    Blocked,
    Eof,
    Failed,
}

fn read_into(fd: &OwnedFd, buf: &mut Buf) -> Io {
    if !buf.has_space() {
        return Io::Blocked;
    }
    match rustix::net::recv(fd, &mut buf.data[buf.end..], RecvFlags::empty()) {
        Ok((0, _)) => Io::Eof,
        Ok((n, _)) => {
            buf.end += n;
            Io::Progress
        }
        Err(Errno::AGAIN) | Err(Errno::INTR) => Io::Blocked,
        Err(_) => Io::Failed,
    }
}

fn write_from(fd: &OwnedFd, buf: &mut Buf) -> Io {
    if buf.is_empty() {
        return Io::Blocked;
    }
    // NOSIGNAL: a peer that went away is an error here, not a SIGPIPE.
    match rustix::net::send(fd, &buf.data[buf.start..buf.end], SendFlags::NOSIGNAL) {
        Ok(n) => {
            buf.start += n;
            if buf.is_empty() {
                buf.reset();
            }
            Io::Progress
        }
        Err(Errno::AGAIN) | Err(Errno::INTR) => Io::Blocked,
        Err(_) => Io::Failed,
    }
}

/// Move whatever can be moved without blocking, then re-arm the events
/// that can make more progress.
fn pump(epoll: &OwnedFd, pair: &mut Pair, index: usize) {
    let (Some(tcp), Some(vsock)) = (pair.tcp.as_ref(), pair.vsock.as_ref()) else {
        return;
    };
    if pair.connecting {
        match rustix::net::sockopt::socket_error(vsock) {
            Ok(Ok(())) => {
                // Connected once the socket has a peer.
                if rustix::net::getpeername(vsock).is_ok() {
                    pair.connecting = false;
                }
            }
            _ => return close(pair),
        }
    }
    loop {
        let mut progress = false;
        if !pair.tcp_eof {
            match read_into(tcp, &mut pair.up) {
                Io::Progress => progress = true,
                Io::Eof => pair.tcp_eof = true,
                Io::Failed => return close(pair),
                Io::Blocked => {}
            }
        }
        if !pair.connecting {
            match write_from(vsock, &mut pair.up) {
                Io::Progress => progress = true,
                Io::Failed => return close(pair),
                _ => {}
            }
            if pair.tcp_eof && pair.up.is_empty() && !pair.up_shut {
                let _ = rustix::net::shutdown(vsock, Shutdown::Write);
                pair.up_shut = true;
            }
            if !pair.vsock_eof {
                match read_into(vsock, &mut pair.down) {
                    Io::Progress => progress = true,
                    Io::Eof => pair.vsock_eof = true,
                    Io::Failed => return close(pair),
                    Io::Blocked => {}
                }
            }
        }
        match write_from(tcp, &mut pair.down) {
            Io::Progress => progress = true,
            Io::Failed => return close(pair),
            _ => {}
        }
        if pair.vsock_eof && pair.down.is_empty() && !pair.down_shut {
            let _ = rustix::net::shutdown(tcp, Shutdown::Write);
            pair.down_shut = true;
        }
        if pair.up_shut && pair.down_shut {
            return close(pair);
        }
        if !progress {
            break;
        }
    }

    let mut tcp_interest = epoll::EventFlags::empty();
    if !pair.tcp_eof && pair.up.has_space() {
        tcp_interest |= epoll::EventFlags::IN;
    }
    if !pair.down.is_empty() {
        tcp_interest |= epoll::EventFlags::OUT;
    }
    let mut vsock_interest = epoll::EventFlags::empty();
    if pair.connecting || !pair.up.is_empty() {
        vsock_interest |= epoll::EventFlags::OUT;
    }
    if !pair.connecting && !pair.vsock_eof && pair.down.has_space() {
        vsock_interest |= epoll::EventFlags::IN;
    }
    let rearmed = epoll::modify(
        epoll,
        tcp,
        epoll::EventData::new_u64(2 * index as u64),
        tcp_interest,
    )
    .and_then(|()| {
        epoll::modify(
            epoll,
            vsock,
            epoll::EventData::new_u64(2 * index as u64 + 1),
            vsock_interest,
        )
    });
    if rearmed.is_err() {
        close(pair);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    fail(b"panic")
}
