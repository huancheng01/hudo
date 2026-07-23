# hudo installer
# Usage: irm hudo.zexa.cc/install.ps1 | iex
# With profile restore (one command to rebuild a full dev environment):
#   $env:HUDO_PROFILE = "https://example.com/hudo-profile.toml"; irm hudo.zexa.cc/install.ps1 | iex
# or: & ([scriptblock]::Create((irm hudo.zexa.cc/install.ps1))) -Profile <url-or-path>

param(
    [string]$Profile = ""
)

$ErrorActionPreference = "Stop"

if (-not $Profile -and $env:HUDO_PROFILE) {
    $Profile = $env:HUDO_PROFILE
}

$repo      = "zexadev/hudo"
$installDir = "$env:USERPROFILE\.hudo\bin"

Write-Host ""
Write-Host "  ==========================================" -ForegroundColor DarkGray
Write-Host "    hudo - Dev Environment Bootstrap Tool" -ForegroundColor Cyan
Write-Host "  ==========================================" -ForegroundColor DarkGray
Write-Host ""

# ── 1. Fetch latest release ──────────────────────────────────────────────────
Write-Host "  > Fetching latest version..." -ForegroundColor Cyan
try {
    $headers = @{ "User-Agent" = "hudo-installer" }
    $release = Invoke-RestMethod `
        -Uri "https://api.github.com/repos/$repo/releases/latest" `
        -Headers $headers `
        -ErrorAction Stop
} catch {
    Write-Host "  x Failed to reach GitHub API, check your network" -ForegroundColor Red
    Write-Host "    $_" -ForegroundColor DarkGray
    exit 1
}

$version = $release.tag_name.TrimStart('v')
$asset   = $release.assets | Where-Object { $_.name -eq "hudo.exe" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "  x hudo.exe not found in release v$version" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
Write-Host "  + Latest version: v$version" -ForegroundColor Green

# ── 2. Check existing installation ───────────────────────────────────────────
$exePath = "$installDir\hudo.exe"
$skipInstall = $false
if (Test-Path $exePath) {
    try {
        $currentVer = (& $exePath --version 2>$null) -replace '^hudo\s+', ''
        if ($currentVer -eq $version) {
            Write-Host "  + Already up to date (v$version)" -ForegroundColor Green
            $skipInstall = $true
        } else {
            Write-Host "  > Upgrading: v$currentVer -> v$version" -ForegroundColor Cyan
        }
    } catch {
        Write-Host "  > Reinstalling v$version" -ForegroundColor Cyan
    }
} else {
    Write-Host "  > Installing v$version to $installDir" -ForegroundColor Cyan
}

if (-not $skipInstall) {
    # ── 3. Download hudo.exe ─────────────────────────────────────────────────
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
    $tmpPath = "$env:TEMP\hudo-install.exe"

    Write-Host "  > Downloading..." -ForegroundColor Cyan
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tmpPath -UseBasicParsing -ErrorAction Stop
    } catch {
        Write-Host "  x Download failed: $_" -ForegroundColor Red
        exit 1
    }

    # ── 4. Install (atomic replace) ──────────────────────────────────────────
    Unblock-File -Path $tmpPath
    Move-Item -Force $tmpPath $exePath

    # ── 5. Add to user PATH ──────────────────────────────────────────────────
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
        Write-Host "  > Added $installDir to user PATH" -ForegroundColor Cyan
    }

    Write-Host ""
    Write-Host "  + hudo v$version installed successfully!" -ForegroundColor Green
}

# ── 6. Optional: restore environment from profile ────────────────────────────
if ($Profile) {
    Write-Host ""
    Write-Host "  > Restoring environment from profile..." -ForegroundColor Cyan
    $profilePath = $Profile
    if ($Profile -match '^https?://') {
        $profilePath = "$env:TEMP\hudo-profile.toml"
        try {
            Invoke-WebRequest -Uri $Profile -OutFile $profilePath -UseBasicParsing -ErrorAction Stop
        } catch {
            Write-Host "  x Profile download failed: $_" -ForegroundColor Red
            exit 1
        }
    }
    if (-not (Test-Path $profilePath)) {
        Write-Host "  x Profile file not found: $profilePath" -ForegroundColor Red
        exit 1
    }
    & $exePath import $profilePath -y
    exit $LASTEXITCODE
}

# ── 7. Done ──────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "  Restart your terminal and run 'hudo' to get started" -ForegroundColor DarkGray
Write-Host ""
