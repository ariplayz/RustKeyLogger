# RustKeyLogger Uninstaller
# This script removes all traces of the keylogger installation
$installDir = Join-Path $env:LOCALAPPDATA "WindowsSystemUtility"
$registryPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
$registryValue = "WinSysUtils"
# Check if running with elevated privileges (helpful but not required)
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "[!] Not running as Administrator - some processes may not be killable" -ForegroundColor Yellow
    Write-Host ""
}
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "   RustKeyLogger (WinSysUtils) Uninstallation" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "Terminating processes..." -ForegroundColor Yellow
$killedCount = 0
# Kill main WinSysUtils processes
try {
    $mainProcesses = Get-Process -Name "WinSysUtils" -ErrorAction SilentlyContinue
    foreach ($p in $mainProcesses) {
        try {
            $processPath = $p.Path
            if ($processPath -and ($processPath -like "$installDir*")) {
                Stop-Process -Id $p.Id -Force -ErrorAction Stop
                Write-Host "  [✓] Killed main process: $($p.Name) (PID: $($p.Id))"
                $killedCount++
            }
        } catch {
            Write-Host "  [!] Could not kill process $($p.Id): $($_.Exception.Message)" -ForegroundColor Yellow
        }
    }
} catch {}
# Kill watchdog processes (12-digit numeric names)
try {
    $allProcesses = Get-Process -ErrorAction SilentlyContinue
    foreach ($p in $allProcesses) {
        try {
            if ($p.ProcessName -match "^\d{12}$") {
                $processPath = $p.Path
                if ($processPath -and ($processPath -like "$installDir*")) {
                    Stop-Process -Id $p.Id -Force -ErrorAction Stop
                    Write-Host "  [✓] Killed watchdog process: $($p.ProcessName) (PID: $($p.Id))"
                    $killedCount++
                }
            }
        } catch {}
    }
} catch {}
# Also kill any RustKeyLogger processes (original name)
try {
    $rustProcesses = Get-Process -Name "RustKeyLogger" -ErrorAction SilentlyContinue
    foreach ($p in $rustProcesses) {
        try {
            Stop-Process -Id $p.Id -Force -ErrorAction Stop
            Write-Host "  [✓] Killed RustKeyLogger process (PID: $($p.Id))"
            $killedCount++
        } catch {
            Write-Host "  [!] Could not kill RustKeyLogger process: $($_.Exception.Message)" -ForegroundColor Yellow
        }
    }
} catch {}
if ($killedCount -eq 0) {
    Write-Host "  No running processes found" -ForegroundColor Gray
} else {
    Write-Host "  Killed $killedCount process(es)" -ForegroundColor Green
}
# Wait for processes to terminate
Start-Sleep -Seconds 2
Write-Host ""
Write-Host "Removing registry startup entry..." -ForegroundColor Yellow
try {
    $regEntry = Get-ItemProperty -Path $registryPath -Name $registryValue -ErrorAction SilentlyContinue
    if ($regEntry) {
        Remove-ItemProperty -Path $registryPath -Name $registryValue -ErrorAction Stop
        Write-Host "  [✓] Removed registry value: $registryValue" -ForegroundColor Green
    } else {
        Write-Host "  [-] Registry entry not found (already removed or never installed)" -ForegroundColor Gray
    }
} catch {
    Write-Host "  [!] Error removing registry entry: $($_.Exception.Message)" -ForegroundColor Red
}
Write-Host ""
Write-Host "Removing installation files..." -ForegroundColor Yellow
if (Test-Path $installDir) {
    try {
        Get-ChildItem -Path $installDir -File -Force | ForEach-Object {
            try {
                Remove-Item -Path $_.FullName -Force -ErrorAction Stop
                Write-Host "  [✓] Deleted: $($_.Name)"
            } catch {
                Write-Host "  [!] Could not delete: $($_.Name)" -ForegroundColor Yellow
            }
        }
        Remove-Item -Path $installDir -Force -Recurse -ErrorAction Stop
        Write-Host "  [✓] Deleted directory: $installDir" -ForegroundColor Green
    } catch {
        Write-Host "  [!] Error removing directory: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "  [!] You may need to manually delete: $installDir" -ForegroundColor Yellow
    }
} else {
    Write-Host "  [-] Installation directory not found (already removed or never installed)" -ForegroundColor Gray
}
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "Uninstallation complete!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
