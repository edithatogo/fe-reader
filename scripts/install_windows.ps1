param(
  [ValidateSet("CurrentUser", "AllUsers")]
  [string]$Scope = "CurrentUser",
  [ValidateSet("Debug", "Release")]
  [string]$Configuration = "Release",
  [string]$InstallDir,
  [switch]$RegisterFileAssociation,
  [switch]$RegisterCom,
  [switch]$NoBuild
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Resolve-Path (Join-Path $ScriptDir "..")

if (-not $InstallDir) {
  if ($Scope -eq "AllUsers") {
    $InstallDir = Join-Path $env:ProgramFiles "Fe Reader\bin"
  } else {
    $LocalAppData = [Environment]::GetFolderPath("LocalApplicationData")
    $InstallDir = Join-Path $LocalAppData "Programs\Fe Reader\bin"
  }
}

if ($Scope -eq "AllUsers") {
  $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
  if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "AllUsers install requires an elevated PowerShell session."
  }
}

if (-not $NoBuild) {
  $cargoArgs = @("build", "--manifest-path", (Join-Path $RepoRoot "Cargo.toml"), "-p", "fe_reader_cli")
  if ($Configuration -eq "Release") {
    $cargoArgs += "--release"
  }
  & cargo @cargoArgs
}

$ProfileDir = if ($Configuration -eq "Release") { "release" } else { "debug" }
$Binary = Join-Path $RepoRoot "target\$ProfileDir\fe-reader.exe"
if (-not (Test-Path $Binary)) {
  throw "Built binary not found: $Binary"
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item -Force $Binary (Join-Path $InstallDir "fe-reader.exe")

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($Scope -eq "CurrentUser" -and ($UserPath -split ";" | Where-Object { $_ -eq $InstallDir }).Count -eq 0) {
  $NewUserPath = (($UserPath, $InstallDir) | Where-Object { $_ }) -join ";"
  [Environment]::SetEnvironmentVariable("Path", $NewUserPath, "User")
}

if ($RegisterFileAssociation) {
  $AssociationRoot = if ($Scope -eq "AllUsers") { "HKLM:\Software\Classes" } else { "HKCU:\Software\Classes" }
  $PdfKey = Join-Path $AssociationRoot ".pdf"
  $CommandKey = Join-Path $AssociationRoot "FeReader.PDF\shell\open\command"
  New-Item -Force -Path $PdfKey | Out-Null
  New-Item -Force -Path $CommandKey | Out-Null
  New-ItemProperty -Force -Path $PdfKey -Name "OpenWithProgids" -PropertyType MultiString -Value "FeReader.PDF" | Out-Null
  Set-Item -Path $CommandKey -Value ('"{0}" "%1"' -f (Join-Path $InstallDir "fe-reader.exe"))
}

if ($RegisterCom) {
  Write-Warning "COM registration is not performed by this local helper. Use signed MSI/MSIX packaging once COM automation leaves the contract-only lane."
}

Write-Host "Installed Fe Reader CLI to $(Join-Path $InstallDir 'fe-reader.exe')"
Write-Host "Open a new shell, then verify with: fe-reader doctor"
