using System.Runtime.InteropServices;

namespace FeReader.Native;

/// <summary>
/// Managed entrypoint for Fe Reader's preview C ABI wrapper.
/// </summary>
public static class FeReaderNative
{
    /// <summary>
    /// Native library name used by P/Invoke resolution.
    /// </summary>
    public const string NativeLibraryName = "fe_reader_c_abi";

    /// <summary>
    /// Managed package version.
    /// </summary>
    public const string PackageVersion = "0.1.0-preview.1";

    /// <summary>
    /// Expected C ABI major version for this package.
    /// </summary>
    public const uint AbiVersionMajor = 0;

    /// <summary>
    /// Expected C ABI minor version for this package.
    /// </summary>
    public const uint AbiVersionMinor = 1;

    /// <summary>
    /// Expected C ABI patch version for this package.
    /// </summary>
    public const uint AbiVersionPatch = 0;

    /// <summary>
    /// Returns the native ABI version. Requires the native library to be resolvable.
    /// </summary>
    public static FeReaderNativeVersion GetNativeVersion() =>
        new(
            NativeMethods.FeReaderCAbiVersionMajor(),
            NativeMethods.FeReaderCAbiVersionMinor(),
            NativeMethods.FeReaderCAbiVersionPatch()
        );

    /// <summary>
    /// Returns whether the native ABI exposes apply operations. Wave 0 must return false.
    /// </summary>
    public static bool SupportsApply() => NativeMethods.FeReaderCAbiSupportsApply() != 0;

    /// <summary>
    /// Returns whether the native ABI exposes plan-only probes.
    /// </summary>
    public static bool SupportsPlanOnly() => NativeMethods.FeReaderCAbiSupportsPlanOnly() != 0;

    /// <summary>
    /// Returns the native no-write plan contract probe. Requires the native library to be resolvable.
    /// </summary>
    public static FeReaderPlanContract GetNoopPlanContract() =>
        NativeMethods.FeReaderCAbiPlanNoopContract();

    /// <summary>
    /// Returns static managed contract metadata without loading the native library.
    /// </summary>
    public static FeReaderManagedContract ManagedContract => new(
        PackageVersion,
        new FeReaderNativeVersion(AbiVersionMajor, AbiVersionMinor, AbiVersionPatch),
        ExposesApply: false,
        ExposesPlanOnly: true,
        MutationPolicy: "read_only_or_plan_only"
    );
}

/// <summary>
/// Managed package contract metadata.
/// </summary>
/// <param name="PackageVersion">The NuGet package version.</param>
/// <param name="ExpectedAbiVersion">The expected native ABI version.</param>
/// <param name="ExposesApply">Whether the managed wrapper exposes apply operations.</param>
/// <param name="ExposesPlanOnly">Whether the managed wrapper exposes plan-only probes.</param>
/// <param name="MutationPolicy">The wrapper mutation policy.</param>
public sealed record FeReaderManagedContract(
    string PackageVersion,
    FeReaderNativeVersion ExpectedAbiVersion,
    bool ExposesApply,
    bool ExposesPlanOnly,
    string MutationPolicy
);

/// <summary>
/// Native C ABI version tuple.
/// </summary>
/// <param name="Major">The ABI major version.</param>
/// <param name="Minor">The ABI minor version.</param>
/// <param name="Patch">The ABI patch version.</param>
public readonly record struct FeReaderNativeVersion(uint Major, uint Minor, uint Patch);

/// <summary>
/// Managed representation of the C ABI plan-only contract probe.
/// </summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct FeReaderPlanContract
{
    /// <summary>
    /// ABI major version used by this struct layout.
    /// </summary>
    public readonly uint AbiVersionMajor;

    /// <summary>
    /// Risk level encoded by the C ABI.
    /// </summary>
    public readonly uint RiskLevel;

    /// <summary>
    /// Write mode encoded by the C ABI.
    /// </summary>
    public readonly uint WriteMode;

    /// <summary>
    /// Non-zero when the plan may be applied without further review.
    /// </summary>
    public readonly uint ApprovedForApply;

    /// <summary>
    /// Number of planned operations.
    /// </summary>
    public readonly uint OperationCount;

    /// <summary>
    /// Native status code.
    /// </summary>
    public readonly int Status;
}

internal static partial class NativeMethods
{
    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_version_major")]
    internal static extern uint FeReaderCAbiVersionMajor();

    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_version_minor")]
    internal static extern uint FeReaderCAbiVersionMinor();

    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_version_patch")]
    internal static extern uint FeReaderCAbiVersionPatch();

    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_supports_apply")]
    internal static extern uint FeReaderCAbiSupportsApply();

    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_supports_plan_only")]
    internal static extern uint FeReaderCAbiSupportsPlanOnly();

    [DllImport(FeReaderNative.NativeLibraryName, EntryPoint = "fe_reader_c_abi_plan_noop_contract")]
    internal static extern FeReaderPlanContract FeReaderCAbiPlanNoopContract();
}
