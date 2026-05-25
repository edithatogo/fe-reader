---
title: Rendering Contract
description: Rendering is adapter-owned and starts with deterministic, testable tile contracts.
---

Track B establishes the render backend boundary without moving renderer dependencies into `fe_reader_core`.

The current scaffold includes:

- validated tile requests
- stable cache keys
- deterministic `NullRenderBackend`
- bounded tile cache
- PDFium adapter boundary with explicit unavailable-runtime behavior
- conservative GPU compositor and hardware acceleration flags

Real PDFium runtime discovery, visual oracle fixtures, and GPU rendering remain governed adapter work.
