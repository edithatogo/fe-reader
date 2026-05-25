using System.Reflection;
using System.Runtime.InteropServices;
using FeReader.Native;

var contract = FeReaderNative.ManagedContract;

Require(contract.PackageVersion == "0.1.0-preview.1", "package version must match C3 preview");
Require(contract.ExpectedAbiVersion == new FeReaderNativeVersion(0, 1, 0), "ABI version must match C ABI snapshot");
Require(!contract.ExposesApply, "NuGet wrapper must not expose apply in Wave 0");
Require(contract.ExposesPlanOnly, "NuGet wrapper must expose plan-only contract metadata");
Require(contract.MutationPolicy == "read_only_or_plan_only", "mutation policy must remain plan-only");
Require(Marshal.SizeOf<FeReaderPlanContract>() == 24, "plan contract layout must stay C-compatible");

var nativeMethods = typeof(FeReaderNative).Assembly.GetType("FeReader.Native.NativeMethods", throwOnError: true)!;
var entryPoints = nativeMethods.GetMethods(BindingFlags.Static | BindingFlags.NonPublic)
    .Select(method => method.GetCustomAttribute<DllImportAttribute>())
    .Where(attribute => attribute is not null)
    .Select(attribute => attribute!.EntryPoint)
    .ToHashSet(StringComparer.Ordinal);

foreach (var expected in new[]
{
    "fe_reader_c_abi_version_major",
    "fe_reader_c_abi_version_minor",
    "fe_reader_c_abi_version_patch",
    "fe_reader_c_abi_supports_apply",
    "fe_reader_c_abi_supports_plan_only",
    "fe_reader_c_abi_plan_noop_contract",
})
{
    Require(entryPoints.Contains(expected), $"missing P/Invoke entry point: {expected}");
}

Console.WriteLine("FeReader.Native smoke: ok");

static void Require(bool condition, string message)
{
    if (!condition)
    {
        throw new InvalidOperationException(message);
    }
}
