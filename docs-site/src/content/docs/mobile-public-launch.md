---
title: Mobile Public Launch
description: Android and iOS public launch readiness gate for Fe Reader.
---

# Mobile Public Launch

The `mobile_public_launch` feature gate tracks Android and iOS public launch
readiness separately from desktop release readiness. It does not block desktop stable launch.

Mobile packages remain deferred until store artifacts, signing evidence, privacy
declarations, accessibility evidence, power evidence and smoke evidence are
complete.

Run the advisory gate from the repository root:

```bash
python3 scripts/mobile_public_launch_check.py
```

The detailed checklist lives in
[`docs/mobile-public-launch-readiness.md`](https://github.com/edithatogo/fe-reader/blob/main/docs/mobile-public-launch-readiness.md).
