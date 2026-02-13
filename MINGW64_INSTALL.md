# MinGW-w64 Installation Guide (64-bit)

## The Problem
You installed MinGW 32-bit, but Rust needs MinGW-w64 (64-bit version) for x86_64 compilation.

Error: `sorry, unimplemented: 64-bit mode not compiled in`

## Quick Solution: Install MinGW-w64 via MSYS2

### Step 1: Install MSYS2 (Recommended - Easiest)

1. **Download MSYS2**:
   - Go to: https://www.msys2.org/
   - Download the installer: `msys2-x86_64-latest.exe`
   - Run the installer (default location: `C:\msys64`)

2. **Install MinGW-w64 toolchain**:
   Open MSYS2 MINGW64 terminal and run:
   ```bash
   pacman -S mingw-w64-x86_64-gcc
   pacman -S mingw-w64-x86_64-tools-git
   ```

3. **Add to PATH**:
   ```powershell
   [System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\msys64\mingw64\bin", [System.EnvironmentVariableTarget]::User)
   ```

4. **Restart PowerShell** and verify:
   ```powershell
   gcc --version
   # Should show: gcc.exe (Rev...) 13.x.x or similar
   ```

### Step 2: Build Your Project

```powershell
# Make sure GNU toolchain is active
rustup default stable-x86_64-pc-windows-gnu

# Clean and build
cd 'C:\Users\Ari Cummings\RustroverProjects\RustKeyLogger'
cargo clean
cargo build --release
```

## Alternative: Manual MinGW-w64 Installation

If you don't want MSYS2:

1. **Download Pre-built MinGW-w64**:
   - Go to: https://github.com/niXman/mingw-builds-binaries/releases
   - Download: `x86_64-13.2.0-release-posix-seh-msvcrt-rt_v11-rev1.7z`
   - Size: ~50MB

2. **Extract**:
   - Use 7-Zip (download from https://www.7-zip.org/ if needed)
   - Extract to `C:\mingw64\`
   - Final structure: `C:\mingw64\bin\gcc.exe`

3. **Add to PATH**:
   ```powershell
   [System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\mingw64\bin", [System.EnvironmentVariableTarget]::User)
   ```

4. **Restart PowerShell** and build

## Alternative: Use Chocolatey

If you have Chocolatey package manager:

```powershell
choco install mingw --version=11.2.0
```

## Verification

After installation, verify with:

```powershell
gcc --version
# Should show 64-bit version: x86_64-w64-mingw32

dlltool --version
# Should be present

rustup show
# Should show: stable-x86_64-pc-windows-gnu (active)
```

## Then Build

```powershell
cd 'C:\Users\Ari Cummings\RustroverProjects\RustKeyLogger'
cargo clean
cargo build --release
```

Should create: `target\release\RustKeyLogger.exe` ✅

## Current Status

- ❌ C:\MinGW - 32-bit version (won't work for 64-bit Rust)
- ✅ Need: MinGW-w64 64-bit version

## Quick Commands After Installing MinGW-w64

```powershell
# Set toolchain
rustup default stable-x86_64-pc-windows-gnu

# Add to PATH (adjust path if different)
$env:Path = "C:\msys64\mingw64\bin;" + $env:Path

# Build
cd 'C:\Users\Ari Cummings\RustroverProjects\RustKeyLogger'
cargo build --release
```

The exe will be: `target\release\RustKeyLogger.exe`

