# Google Play Publishing Checklist

- Build a signed AAB and record the SHA-256 checksum in release evidence.
- Configure signing key/upload key storage without committing private keys.
- Confirm Play Console access, package name ownership and release track.
- Declare document access, notification and storage permissions accurately.
- Provide Android privacy declaration and data-safety disclosures for local document processing.
- Test SAF open/save, share target, print and notification flows on emulator and at least one real device.
- Attach TalkBack, touch target, reduced-motion and keyboard accessibility evidence.
- Attach Android power or battery profiling evidence for open, scroll and annotation flows.
- Add screenshots for phone and tablet.
- Keep Android intents read-only or plan-only unless `patch_plan_id`, `document_hash_match`, `policy_allow_rule`, `approval_token` and audit receipt requirements are satisfied.
