#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v dotnet >/dev/null 2>&1; then
  echo "dotnet not installed; NuGet wrapper check advisory skip during bootstrap" >&2
  exit 0
fi

dotnet build packaging/nuget/FeReader.Native.sln --configuration Release --nologo
DOTNET_ROLL_FORWARD=Major dotnet run --project packaging/nuget/FeReader.Native.Smoke/FeReader.Native.Smoke.csproj --configuration Release --no-build
dotnet pack packaging/nuget/FeReader.Native/FeReader.Native.csproj --configuration Release --no-build --output target/nuget --nologo

package="target/nuget/FeReader.Native.0.1.0-preview.1.nupkg"
test -f "$package"

python3 - "$package" <<'PY'
import sys
import zipfile

package = sys.argv[1]
required = {
    "README.md",
    "contracts/fe_reader_c_abi.facade.json",
    "build/native/include/fe_reader_c_abi.h",
    "lib/net8.0/FeReader.Native.dll",
}
with zipfile.ZipFile(package) as archive:
    names = set(archive.namelist())
missing = sorted(required - names)
if missing:
    raise SystemExit(f"NuGet package missing expected files: {missing}")
runtime_assets = sorted(name for name in names if name.startswith("runtimes/"))
if runtime_assets:
    raise SystemExit(f"C3 package must not ship native runtime assets yet: {runtime_assets}")
PY

echo "NuGet wrapper check passed"
