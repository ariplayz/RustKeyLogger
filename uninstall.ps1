﻿$ErrorActionPreference = "SilentlyContinue"

$installDir = Join-Path $env:LOCALAPPDATA "WindowsSystemUtility"
$registryPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
$registryValue = "WinSysUtils"

Write-Host "Starting KeyLogger (WinSysUtils) Uninstallation..." -ForegroundColor Cyan

Write-Host "Terminating processes..." -ForegroundColor Yellow

# Kill main WinSysUtils processes
$mainProcesses = Get-Process -Name "WinSysUtils"
foreach ($p in $mainProcesses) {
    try {
        if ($p.Path -like "$installDir*") {
            Stop-Process -Id $p.Id -Force
            Write-Host "Killed main process: $($p.Id)"
        }
    } catch {}
}

# Kill watchdog processes (12-digit numeric names)
$allProcesses = Get-Process
foreach ($p in $allProcesses) {
    try {
        if ($p.ProcessName -match "^\d{12}$") {
            if ($p.Path -like "$installDir*") {
                Stop-Process -Id $p.Id -Force
                Write-Host "Killed watchdog process: $($p.ProcessName) ($($p.Id))"
            }
        }
    } catch {}
}

# Also kill any RustKeyLogger processes (original name)
$rustProcesses = Get-Process -Name "RustKeyLogger"
foreach ($p in $rustProcesses) {
    try {
        Stop-Process -Id $p.Id -Force
        Write-Host "Killed RustKeyLogger process: $($p.Id)"
    } catch {}
}

# Wait for processes to terminate
Start-Sleep -Seconds 1

Write-Host "Removing registry startup entry..." -ForegroundColor Yellow
if (Get-ItemProperty -Path $registryPath -Name $registryValue -ErrorAction SilentlyContinue) {
    Remove-ItemProperty -Path $registryPath -Name $registryValue
    Write-Host "Removed registry value: $registryValue"
} else {
    Write-Host "Registry entry not found (already removed or never installed)"
}

Write-Host "Removing installation files..." -ForegroundColor Yellow
if (Test-Path $installDir) {
    Remove-Item -Path "$installDir\*" -Force -Recurse
    Remove-Item -Path $installDir -Force -Recurse
    Write-Host "Deleted directory: $installDir"
} else {
    Write-Host "Installation directory not found (already removed or never installed)"
}

Write-Host "`nUninstallation complete!" -ForegroundColor Green
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

