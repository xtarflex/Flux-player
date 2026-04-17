# Flux Player - Interactive Uninstaller (Beta)
# --------------------------------------------------

$ErrorActionPreference = "SilentlyContinue"
[console]::OutputEncoding = [System.Text.Encoding]::UTF8

# Flux Aesthetic Colors
$Cyan = "`e[96m"
$Magenta = "`e[95m"
$Red = "`e[91m"
$Reset = "`e[0m"
$Bold = "`e[1m"

Clear-Host

# 1. Branding Header
Write-Host "${Cyan}${Bold}"
Write-Host "  ____ _    _  _ _  _ "
Write-Host "  |___ |    |  |  \/  "
Write-Host "  |    |___ |__| _/\_ (BETA 1.1)"
Write-Host "  --------------------------${Reset}"
Write-Host ""

# 2. Preparation
Write-Host "${Cyan}[!] Preparing to UNINSTALL Flux Player...${Reset}"
Stop-Process -Name "flux-player" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

# 3. Decision Point
Write-Host ""
Write-Host "${Magenta}${Bold}[?] DATA DISCOVERY${Reset}"
Write-Host "Would you like to perform a ${Bold}Clean Wipe${Reset}?"
Write-Host "This will delete your library database, posters, and settings."
Write-Host ""
$choice = Read-Host "[y] Clean Wipe (Delete all data) / [n] Standard Uninstall (Keep data)"

# 4. Cleanup Logic
Write-Host ""
Write-Host "${Cyan}[+] Removing application binaries...${Reset}"

# Remove files in the current directory (assuming script is run from install root)
$CurrentDir = Get-Location
Get-ChildItem -Path $CurrentDir -Exclude "uninstall.ps1" | Remove-Item -Recurse -Force

if ($choice -eq 'y') {
    Write-Host "${Red}[!] Purging AppData (Database & Image Cache)...${Reset}"
    $AppData = "$env:APPDATA\com.sunny.flux-player"
    if (Test-Path $AppData) {
        Remove-Item -Path $AppData -Recurse -Force
    }
    Write-Host "${Red}[+] System is now pristine.${Reset}"
} else {
    Write-Host "${Magenta}[i] Keeping your library data for future installs.${Reset}"
}

# 5. Finalize
Write-Host ""
Write-Host "${Cyan}${Bold}Flux has been successfully uninstalled.${Reset}"
Write-Host "Hope to see you again soon."
Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
