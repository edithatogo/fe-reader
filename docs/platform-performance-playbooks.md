# Platform Performance Playbooks

Fe Reader keeps platform profiling separate from generic Rust benchmarks. Use these playbooks when a release candidate needs native traces or when a regression only reproduces on one operating system or input stack.

## Linux

- Primary tools: `perf`, `cargo flamegraph`, `heaptrack`, Valgrind Callgrind/DHAT.
- Start from `scripts/perf_profile_linux.sh`.
- Use this lane for CLI startup, page open, tile rendering, and renderer allocation profiling.

## macOS

- Primary tools: Instruments Time Profiler, Allocations, Energy Log, Metal System Trace.
- Start from `scripts/perf_profile_macos.sh`.
- Use this lane for app startup, sandbox/bookmark access, file open, native shell rendering, and battery impact.

## Windows

- Primary tools: Windows Performance Recorder, Windows Performance Analyzer, ETW, Visual Studio Profiler.
- Start from `scripts/perf_profile_windows.ps1`.
- Use this lane for startup, file open, tile rendering, and WebView/native bridge profiling.

## Android

- Primary tools: Android Studio Profiler, Perfetto, simpleperf, systrace.
- Start from `scripts/perf_profile_android.sh`.
- Use this lane for scroll latency, annotation latency, file access, and battery impact.

## iOS

- Primary tools: Xcode Instruments, Metal System Trace, Time Profiler, Allocations, Energy Log.
- Start from `scripts/perf_profile_ios.sh`.
- Use this lane for Pencil latency, file coordination, memory pressure, and rendering traces.

## Web

- Primary tools: Chrome Performance panel, Lighthouse, WebGPU capture tools.
- Start from `scripts/perf_profile_web.sh`.
- Use this lane for WASM load, file access, and tile compositing.

## Reporting

Each playbook writes a short summary under `artifacts/perf/platform/<platform>/summary.md` and, where the toolchain supports it, stores native trace artifacts next to it. The release candidate report should attach those summaries alongside the cross-platform benchmark manifest.
