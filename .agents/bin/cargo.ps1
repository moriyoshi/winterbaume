#!/usr/bin/env pwsh
# Agent-only cargo wrapper. Mints a per-session CARGO_TARGET_DIR and ensures
# RUSTC_WRAPPER points at a fresh sccache-wrapper, so concurrent coding agents
# do not contend on the shared default `target/` lock.
#
# Use this script in place of bare `cargo` from any agent shell:
#   .agents/bin/cargo.ps1 build -p winterbaume-foo
#
# Humans: keep using mise + bare `cargo` as before.

$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent ([System.IO.Path]::GetFullPath($MyInvocation.MyCommand.Path))
$WS = (Resolve-Path (Join-Path $ScriptDir '..' '..')).Path

# Resolve the main worktree. The sccache-wrapper binary, the rustc cache, and
# the scoreboard all live there so sibling worktrees share one binary and one
# cache instead of each rebuilding their own copy. WB_WORKSPACE_ROOT itself
# stays tied to the current worktree — the wrapper substitutes that string out
# of rustc args, so it has to match cargo's absolute paths.
$MainWs = $WS
$worktreeOutput = & git -C $WS worktree list --porcelain 2>$null
if ($LASTEXITCODE -eq 0 -and $worktreeOutput) {
    foreach ($line in $worktreeOutput) {
        if ($line -match '^worktree (.+)$') {
            $candidate = $Matches[1]
            if (Test-Path -LiteralPath $candidate -PathType Container) {
                $MainWs = $candidate
            }
            break
        }
    }
}
$global:LASTEXITCODE = 0

function Get-SessionId {
    if ($env:CLAUDE_CODE_SSE_PORT) {
        return "claude-$($env:CLAUDE_CODE_SSE_PORT)"
    }
    $cur = (Get-CimInstance Win32_Process -Filter "ProcessId=$PID").ParentProcessId
    while ($cur -and $cur -ne 0) {
        $proc = Get-CimInstance Win32_Process -Filter "ProcessId=$cur" -ErrorAction SilentlyContinue
        if (-not $proc) { break }
        $name = [System.IO.Path]::GetFileNameWithoutExtension($proc.Name)
        switch ($name) {
            'claude' { return "claude-$cur" }
            'codex'  { return "codex-$cur"  }
        }
        $cur = $proc.ParentProcessId
    }
    return "shell-$PID"
}

$Session = Get-SessionId

$WrapperBin      = Join-Path $MainWs '.agents-workspace/tmp/sccache-wrapper.exe'
$WrapperSrc      = Join-Path $MainWs 'tools/sccache-wrapper/src/main.rs'
$WrapperToml     = Join-Path $MainWs 'tools/sccache-wrapper/Cargo.toml'
$WrapperBuildDir = Join-Path $MainWs '.agents-workspace/tmp/sccache-wrapper-build'

function Test-NeedsRebuild {
    if (-not (Test-Path $WrapperBin)) { return $true }
    $binTime = (Get-Item $WrapperBin).LastWriteTime
    if ((Get-Item $WrapperSrc).LastWriteTime  -gt $binTime) { return $true }
    if ((Get-Item $WrapperToml).LastWriteTime -gt $binTime) { return $true }
    return $false
}

if (Test-NeedsRebuild) {
    [Console]::Error.WriteLine('.agents/bin/cargo: rebuilding sccache-wrapper')
    New-Item -ItemType Directory -Force -Path (Join-Path $MainWs '.agents-workspace/tmp') | Out-Null
    Push-Location $MainWs
    try {
        $env:RUSTC_WRAPPER     = $null
        $env:CARGO_TARGET_DIR  = $WrapperBuildDir
        cargo build --release -p sccache-wrapper
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    } finally {
        Pop-Location
    }
    # Atomic install: stage to a unique tmp path then rename, so a concurrent
    # rebuilder from a sibling worktree never observes a partial binary.
    $built = Join-Path $WrapperBuildDir 'release/sccache-wrapper.exe'
    $tmpBin = "$WrapperBin.$PID.tmp"
    if (Test-Path $tmpBin) { Remove-Item -Force $tmpBin }
    Copy-Item -Force $built $tmpBin
    Move-Item -Force $tmpBin $WrapperBin
}

$env:CARGO_INCREMENTAL              = '0'
$env:RUSTC_WRAPPER                  = $WrapperBin
$env:WB_WORKSPACE_ROOT              = $WS
$env:WB_RUSTC_CACHE_DIR             = Join-Path $MainWs '.agents-workspace/tmp/winterbaume-rustc-cache'
$env:WB_SCCACHE_WRAPPER_SCOREBOARD  = Join-Path $MainWs '.agents-workspace/tmp/sccache-wrapper-scoreboard'
$env:CARGO_TARGET_DIR               = Join-Path $WS ".agents-workspace/tmp/target-$Session"

# Cap build parallelism so a single crate build does not saturate every core
# and spike load averages. Honour an explicit override if the caller set one.
if (-not $env:CARGO_BUILD_JOBS) {
    $ncpu = [Environment]::ProcessorCount
    $jobs = [Math]::Max(1, [Math]::Floor($ncpu / 2))
    $env:CARGO_BUILD_JOBS = "$jobs"
}

& cargo @args
exit $LASTEXITCODE
