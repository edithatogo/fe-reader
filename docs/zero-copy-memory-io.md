# Zero-Copy, Memory Mapping and Resource Limits

## Goal

Improve performance without making hostile PDFs dangerous.

## Strategy

Use a tiered input model:

```text
Small files: owned bytes or buffered reader
Medium files: bounded buffered reader with object-cache windows
Large files: optional memory mapping through a controlled adapter
Hostile/suspect files: no speculative full decode, strict limits, safe-open mode
```

## Memory-map policy

`memmap2` may be used behind an adapter for large, immutable local files. The adapter must:

- reject files whose size changes during processing where detectable;
- avoid mutable maps for untrusted PDF input;
- fall back to buffered I/O for virtual/cloud/document-provider files;
- not assume memory mapping is available on mobile or sandboxed app-store contexts;
- record whether mmap was used in diagnostic output and performance reports.

## Resource limits

```text
max_objects
max_page_count_before_lazy_mode
max_decoded_stream_bytes
max_nested_forms
max_recursion_depth
max_text_spans_per_page
max_render_tile_pixels
max_repair_attempts
max_oracle_runtime_seconds
```

See `contracts/rust/io_resource_limits.rs`.
