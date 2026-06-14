# Mobile Public Launch Readiness

The `mobile_public_launch` feature gate tracks Android and iOS public launch
readiness after the desktop stable release. It does not block desktop stable launch.
Mobile packages stay deferred until store artifacts, signing evidence,
privacy declarations, accessibility evidence and power evidence are complete.

## Current Status

| Platform | Public channel | Status | Primary blockers |
| --- | --- | --- | --- |
| Android | Google Play, F-Droid evaluation | Deferred | signed AAB, Play Console release evidence, privacy declaration approval, accessibility and power evidence |
| iOS/iPadOS | TestFlight, App Store | Deferred | Apple Developer Program verification, signed archive, TestFlight evidence, privacy manifest approval, accessibility and power evidence |

The machine-readable source of truth is
[`packaging/mobile-public-launch.json`](../packaging/mobile-public-launch.json).
Run the advisory gate with:

```bash
python3 scripts/mobile_public_launch_check.py
```

## Automation Safety

Android intents, iOS App Intents, share extensions, shortcuts and mobile
automation remain read-only or plan-only. They must not apply document changes
unless the normal mutation pipeline is satisfied:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Required mutation guards are `patch_plan_id`, `document_hash_match`,
`policy_allow_rule`, `approval_token` and `audit_receipt`.

## Evidence Required Before Publication

Android publication requires:

- signed AAB or equivalent public package artifact;
- SHA-256 checksums and release provenance;
- Play Console access and release-track evidence;
- Android privacy declaration and data-safety review;
- SAF open/save, share, print and notification smoke evidence;
- TalkBack, touch target and reduced-motion accessibility evidence;
- Android power or battery profiling summary.

iOS/iPadOS publication requires:

- signed archive or store-ready package artifact;
- App Store Connect access and TestFlight evidence;
- App Store privacy manifest review;
- document browser, Files, share sheet, print and PencilKit smoke evidence;
- VoiceOver, Dynamic Type and touch/Pencil alternative accessibility evidence;
- iOS power or energy profiling summary.

## Rollback And Exit Criteria

The rollback path is to keep mobile packages deferred and publish desktop-only release
notes. The exit criteria are complete only when both Android and iOS have store
artifacts, signing evidence, privacy declarations, smoke evidence, accessibility
evidence and power evidence attached to the release.
