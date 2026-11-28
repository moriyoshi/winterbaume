//! The VM helper: the process that becomes a microVM.
//!
//! libkrun's `krun_start_enter` never returns on success; it turns the
//! calling process into the VM, whose exit ends the process with the guest's
//! exit status.  The executor therefore starts a helper process per VM.  Any
//! program can be the helper by calling [`helper_main`]; the crate ships one
//! as `winterbaume-lambda-vm-helper`, and `winterbaume-server` acts as one
//! when started with a hidden argument.
//!
//! libkrun is loaded at run time, so that nothing links against it and a
//! missing library is a reported condition rather than a loader failure.

use std::ffi::{CString, c_char};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::proto::{AGENT_PATH, HelperStatus, LAUNCH_SCRIPT, VmSpec, launch_script};

/// libkrun's tag for the root virtio-fs device.
const KRUN_FS_ROOT_TAG: &str = "/dev/root";
/// `KRUN_TSI_HIJACK_INET`: proxy the guest's IP sockets through the host.
const KRUN_TSI_HIJACK_INET: u32 = 1 << 0;
/// Exit status of a helper that could not start the VM.
pub const HELPER_FAILED_EXIT: i32 = 70;

/// Read a [`VmSpec`] from stdin, report on stderr, and boot the VM.  Never
/// returns.
pub fn helper_main() -> ! {
    die_with_parent();
    let mut input = String::new();
    let spec = std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())
        .and_then(|_| serde_json::from_str::<VmSpec>(&input).map_err(|e| e.to_string()));
    let spec = match spec {
        Ok(spec) => spec,
        Err(message) => fail("vm-unavailable", &format!("invalid VM spec: {message}")),
    };
    match run(&spec) {
        Ok(()) => {
            report(&HelperStatus::Booting);
            std::process::exit(0)
        }
        Err((reason, message)) => fail(reason, &message),
    }
}

/// End this process ( and so the VM ) when the executor's process goes
/// away, however it goes: a VM must not outlive the emulator.
fn die_with_parent() {
    // Not PR_SET_PDEATHSIG: it fires when the parent *thread* that spawned
    // this process exits, which an async runtime's threads may do while the
    // executor lives on.  A reparented process has a new parent id.
    #[cfg(unix)]
    {
        // SAFETY: getppid has no preconditions.
        let parent = unsafe { libc::getppid() };
        // The thread keeps running after `krun_start_enter` takes over the
        // main thread.
        let _ = std::thread::Builder::new()
            .name("parent-watch".to_string())
            .spawn(move || {
                loop {
                    // SAFETY: as above.
                    if unsafe { libc::getppid() } != parent {
                        // SAFETY: _exit ends the process at once.
                        unsafe { libc::_exit(137) };
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });
    }
}

fn fail(reason: &str, message: &str) -> ! {
    report(&HelperStatus::Failed {
        reason: reason.to_string(),
        message: message.to_string(),
    });
    std::process::exit(HELPER_FAILED_EXIT)
}

fn report(status: &HelperStatus) {
    let mut stderr = std::io::stderr().lock();
    let _ = stderr.write_all(status.to_line().as_bytes());
    let _ = stderr.flush();
}

/// Configure and boot.  Returns `Ok` only for a probe; a successful boot
/// does not return.
fn run(spec: &VmSpec) -> Result<(), (&'static str, String)> {
    let krun =
        Krun::load(spec.libkrun.as_deref()).map_err(|message| ("libkrun-unavailable", message))?;
    check_hypervisor().map_err(|message| ("vm-unavailable", message))?;
    if spec.probe {
        return Ok(());
    }
    krun.boot(spec)
        .map_err(|message| ("vm-unavailable", message))
}

#[cfg(target_os = "linux")]
fn check_hypervisor() -> Result<(), String> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .map(|_| ())
        .map_err(|e| format!("cannot open /dev/kvm: {e}"))
}

#[cfg(not(target_os = "linux"))]
fn check_hypervisor() -> Result<(), String> {
    // Hypervisor.framework has no cheap check; a missing entitlement makes
    // `krun_start_enter` fail, which is reported the same way.
    Ok(())
}

#[cfg(target_os = "macos")]
const LIBKRUN_CANDIDATES: &[&str] = &[
    "libkrun.1.dylib",
    "/opt/homebrew/lib/libkrun.1.dylib",
    "/usr/local/lib/libkrun.1.dylib",
];
#[cfg(not(target_os = "macos"))]
const LIBKRUN_CANDIDATES: &[&str] = &["libkrun.so.1", "libkrun.so"];

#[cfg(target_os = "macos")]
const LIBKRUNFW_NAMES: &[&str] = &["libkrunfw.5.dylib", "libkrunfw.dylib"];
#[cfg(not(target_os = "macos"))]
const LIBKRUNFW_NAMES: &[&str] = &["libkrunfw.so.5", "libkrunfw.so"];

type SetLogLevel = unsafe extern "C" fn(u32) -> i32;
type CreateCtx = unsafe extern "C" fn() -> i32;
type SetVmConfig = unsafe extern "C" fn(u32, u8, u32) -> i32;
type AddVirtiofs3 = unsafe extern "C" fn(u32, *const c_char, *const c_char, u64, bool) -> i32;
type SetRoot = unsafe extern "C" fn(u32, *const c_char) -> i32;
type AddOverlayFile =
    unsafe extern "C" fn(u32, *const c_char, *const c_char, *const u8, usize, u32, bool) -> i32;
type SetPortMap = unsafe extern "C" fn(u32, *const *const c_char) -> i32;
type SetWorkdir = unsafe extern "C" fn(u32, *const c_char) -> i32;
type SetExec =
    unsafe extern "C" fn(u32, *const c_char, *const *const c_char, *const *const c_char) -> i32;
type StartEnter = unsafe extern "C" fn(u32) -> i32;
type DisableImplicitVsock = unsafe extern "C" fn(u32) -> i32;
type AddVsock = unsafe extern "C" fn(u32, u32) -> i32;
type AddVsockPort = unsafe extern "C" fn(u32, u32, *const c_char) -> i32;

/// The libkrun 1.x C API, as far as the helper uses it.
///
/// Kept in one place because libkrun's development branch is replacing this
/// API with a builder; a port touches only this type.
struct Krun {
    _library: libloading::Library,
    /// Keeps a pre-loaded libkrunfw resident.
    _firmware: Option<libloading::Library>,
    set_log_level: SetLogLevel,
    create_ctx: CreateCtx,
    set_vm_config: SetVmConfig,
    add_virtiofs3: Option<AddVirtiofs3>,
    set_root: SetRoot,
    add_overlay_file: Option<AddOverlayFile>,
    set_port_map: SetPortMap,
    set_workdir: SetWorkdir,
    set_exec: SetExec,
    start_enter: StartEnter,
    disable_implicit_vsock: Option<DisableImplicitVsock>,
    add_vsock: Option<AddVsock>,
    add_vsock_port: AddVsockPort,
}

impl Krun {
    fn load(explicit: Option<&Path>) -> Result<Self, String> {
        let mut errors = Vec::new();
        let candidates: Vec<PathBuf> = match explicit {
            Some(path) => vec![path.to_path_buf()],
            None => LIBKRUN_CANDIDATES.iter().map(PathBuf::from).collect(),
        };
        // libkrun loads libkrunfw by its soname; one next to an explicitly
        // given libkrun is loaded first, globally, so that it is found.
        let firmware = explicit
            .and_then(Path::parent)
            .and_then(|dir| {
                LIBKRUNFW_NAMES
                    .iter()
                    .map(|name| dir.join(name))
                    .find(|p| p.is_file())
            })
            .and_then(|path| load_global(&path).ok());
        for candidate in candidates {
            // SAFETY: loading libkrun runs its initialisers, which have no
            // preconditions.
            match unsafe { libloading::Library::new(&candidate) } {
                Ok(library) => return Self::bind(library, firmware),
                Err(e) => errors.push(format!("{}: {e}", candidate.display())),
            }
        }
        Err(format!("cannot load libkrun ( {} )", errors.join("; ")))
    }

    fn bind(
        library: libloading::Library,
        firmware: Option<libloading::Library>,
    ) -> Result<Self, String> {
        fn required<T: Copy>(library: &libloading::Library, name: &str) -> Result<T, String> {
            // SAFETY: the symbol types above match libkrun.h of libkrun 1.x.
            unsafe {
                library
                    .get::<T>(name.as_bytes())
                    .map(|symbol| *symbol)
                    .map_err(|e| format!("libkrun lacks {name}: {e}"))
            }
        }
        fn optional<T: Copy>(library: &libloading::Library, name: &str) -> Option<T> {
            required(library, name).ok()
        }
        Ok(Self {
            set_log_level: required(&library, "krun_set_log_level")?,
            create_ctx: required(&library, "krun_create_ctx")?,
            set_vm_config: required(&library, "krun_set_vm_config")?,
            add_virtiofs3: optional(&library, "krun_add_virtiofs3"),
            set_root: required(&library, "krun_set_root")?,
            add_overlay_file: optional(&library, "krun_fs_add_overlay_file"),
            set_port_map: required(&library, "krun_set_port_map")?,
            set_workdir: required(&library, "krun_set_workdir")?,
            set_exec: required(&library, "krun_set_exec")?,
            start_enter: required(&library, "krun_start_enter")?,
            disable_implicit_vsock: optional(&library, "krun_disable_implicit_vsock"),
            add_vsock: optional(&library, "krun_add_vsock"),
            add_vsock_port: required(&library, "krun_add_vsock_port")?,
            _library: library,
            _firmware: firmware,
        })
    }

    /// Configure a VM from `spec` and enter it.  Returns only on failure.
    fn boot(&self, spec: &VmSpec) -> Result<(), String> {
        let c = |s: &str| CString::new(s).map_err(|e| format!("invalid string {s:?}: {e}"));
        let check = |what: &str, status: i32| {
            if status < 0 {
                Err(format!(
                    "{what} failed: {}",
                    std::io::Error::from_raw_os_error(-status)
                ))
            } else {
                Ok(())
            }
        };
        let root = c(&spec.root.to_string_lossy())?;
        let root_tag = c(KRUN_FS_ROOT_TAG)?;
        // The script must outlive the VM; the process ends with it.
        let script: &'static [u8] = Box::leak(launch_script(spec).into_bytes().into_boxed_slice());
        let script_name = c(LAUNCH_SCRIPT.trim_start_matches('/'))?;
        let shell = c("/bin/sh")?;
        let workdir = c(&spec.workdir)?;
        let no_ports: [*const c_char; 1] = [std::ptr::null()];
        // A non-NULL, nearly empty environment: with NULL libkrun would pass
        // this process's environment to the guest.
        let guest_env_entry = c("WINTERBAUME_LAMBDA_VM=1")?;
        // The agent must outlive the VM too.
        let agent: Option<&'static [u8]> = match &spec.agent {
            Some(path) => Some(Box::leak(
                std::fs::read(path)
                    .map_err(|e| format!("cannot read the guest agent {}: {e}", path.display()))?
                    .into_boxed_slice(),
            )),
            None => None,
        };
        let agent_name = c(AGENT_PATH.trim_start_matches('/'))?;
        let sockets = spec
            .forwards
            .iter()
            .map(|f| Ok((f.vsock_port, c(&f.host_socket.to_string_lossy())?)))
            .collect::<Result<Vec<_>, String>>()?;
        let guest_env: [*const c_char; 2] = [guest_env_entry.as_ptr(), std::ptr::null()];

        // SAFETY: every pointer passed below refers to a NUL-terminated
        // string or a NULL-terminated array that lives until
        // `krun_start_enter`, which does not return on success; the overlay
        // data is leaked, so it outlives the VM.
        unsafe {
            (self.set_log_level)(spec.log_level);
            let ctx = (self.create_ctx)();
            check("krun_create_ctx", ctx)?;
            let ctx = ctx as u32;
            check(
                "krun_set_vm_config",
                (self.set_vm_config)(ctx, spec.vcpus.max(1), spec.ram_mib.max(128)),
            )?;
            match self.add_virtiofs3 {
                Some(add) => check(
                    "krun_add_virtiofs3",
                    add(ctx, root_tag.as_ptr(), root.as_ptr(), 0, true),
                )?,
                // Older libkrun cannot mount the root read-only.
                None => check("krun_set_root", (self.set_root)(ctx, root.as_ptr()))?,
            }
            // Never expose guest ports on the host.
            check(
                "krun_set_port_map",
                (self.set_port_map)(ctx, no_ports.as_ptr()),
            )?;

            // The vsock device: with the host's network ( TSI ) or without
            // any network at all, and the agent's ports to the host.
            match (self.disable_implicit_vsock, self.add_vsock) {
                (Some(disable), Some(add)) => {
                    check("krun_disable_implicit_vsock", disable(ctx))?;
                    let features = if spec.network {
                        KRUN_TSI_HIJACK_INET
                    } else {
                        0
                    };
                    check("krun_add_vsock", add(ctx, features))?;
                }
                // Older libkrun always proxies the guest's network.
                _ if !spec.network => {
                    return Err(
                        "this libkrun cannot run a guest without network access".to_string()
                    );
                }
                _ => {}
            }
            for (port, path) in &sockets {
                check(
                    "krun_add_vsock_port",
                    (self.add_vsock_port)(ctx, *port, path.as_ptr()),
                )?;
            }
            check(
                "krun_set_workdir",
                (self.set_workdir)(ctx, workdir.as_ptr()),
            )?;

            let script_path = match self.add_overlay_file {
                Some(add) => {
                    check(
                        "krun_fs_add_overlay_file",
                        add(
                            ctx,
                            root_tag.as_ptr(),
                            script_name.as_ptr(),
                            script.as_ptr(),
                            script.len(),
                            0o100755,
                            false,
                        ),
                    )?;
                    c(LAUNCH_SCRIPT)?
                }
                None => {
                    // Without in-memory overlays the script is a file of the
                    // root; the executor removes these with the root.
                    let name = format!(".winterbaume-launch.{}.sh", std::process::id());
                    std::fs::write(spec.root.join(&name), script)
                        .map_err(|e| format!("cannot write the launch script: {e}"))?;
                    c(&format!("/{name}"))?
                }
            };
            if let Some(agent) = agent {
                match self.add_overlay_file {
                    Some(add) => check(
                        "krun_fs_add_overlay_file",
                        add(
                            ctx,
                            root_tag.as_ptr(),
                            agent_name.as_ptr(),
                            agent.as_ptr(),
                            agent.len(),
                            0o100755,
                            false,
                        ),
                    )?,
                    None => {
                        use std::os::unix::fs::PermissionsExt;
                        let path = spec.root.join(AGENT_PATH.trim_start_matches('/'));
                        if !path.exists() {
                            std::fs::write(&path, agent)
                                .and_then(|()| {
                                    std::fs::set_permissions(
                                        &path,
                                        std::fs::Permissions::from_mode(0o755),
                                    )
                                })
                                .map_err(|e| format!("cannot write the guest agent: {e}"))?;
                        }
                    }
                }
            }
            let argv: [*const c_char; 2] = [script_path.as_ptr(), std::ptr::null()];
            check(
                "krun_set_exec",
                (self.set_exec)(ctx, shell.as_ptr(), argv.as_ptr(), guest_env.as_ptr()),
            )?;

            report(&HelperStatus::Booting);
            let status = (self.start_enter)(ctx);
            check("krun_start_enter", status)?;
            Err(format!("krun_start_enter returned {status}"))
        }
    }
}

#[cfg(unix)]
fn load_global(path: &Path) -> Result<libloading::Library, libloading::Error> {
    use libloading::os::unix::{Library, RTLD_GLOBAL, RTLD_NOW};
    // SAFETY: loading libkrunfw runs no code with preconditions.
    unsafe { Library::open(Some(path), RTLD_NOW | RTLD_GLOBAL).map(Into::into) }
}

#[cfg(not(unix))]
fn load_global(path: &Path) -> Result<libloading::Library, libloading::Error> {
    // SAFETY: as above.
    unsafe { libloading::Library::new(path) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_explicit_libkrun_is_reported() {
        let error = match Krun::load(Some(Path::new("/nonexistent/libkrun.so.1"))) {
            Ok(_) => panic!("a missing library must not load"),
            Err(e) => e,
        };
        assert!(error.contains("/nonexistent/libkrun.so.1"), "{error}");
    }
}
