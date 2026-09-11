@echo off
rem ============================================================================
rem Runvoid Windows Installer Launcher (Batch wrapper for PowerShell)
rem ============================================================================

setlocal
cd /d "%~dp0"

echo [Runvoid Installer] Launching PowerShell installer...
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Installation failed with exit code %ERRORLEVEL%.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo Press any key to exit installer.
pause >nul
endlocal
