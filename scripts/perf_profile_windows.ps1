param(
  [string]$Scenario = "startup"
)

$OutDir = "artifacts/perf/platform/windows"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

switch ($Scenario) {
  "startup" { Write-Host "Capture startup traces with WPR/WPA and ETW." }
  "file.open" { Write-Host "Capture file-open traces and bridge traffic with WPR/WPA." }
  "tile.render" { Write-Host "Capture tile render and WebView/native bridge traces with WPR/WPA." }
  default { throw "Unknown scenario: $Scenario" }
}

@"
# Windows Profiling

- Scenario: $Scenario
- Status: advisory
- Primary tools: Windows Performance Recorder, Windows Performance Analyzer, ETW, Visual Studio Profiler.
"@ | Set-Content -Encoding UTF8 (Join-Path $OutDir "summary.md")
