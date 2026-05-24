# PDF Time Machine and Active Content Firewall

## PDF Time Machine

Many PDFs contain incremental updates. A user may need to know what changed, whether a signature revision was invalidated, whether redacted content still exists in an old revision, or which object changed between revisions.

Fe Reader should expose read-only revision analysis early:

```text
list revisions
show byte ranges
show changed objects
show signatures per revision
show metadata changes
show active content introduced by revision
show redaction risk if old revision contains sensitive text
```

Later, it can support safe export of a selected revision or a before/after comparison.

## Active Content Firewall

PDFs can contain JavaScript actions, launch actions, submit-form actions, remote links, embedded files, RichMedia and other active behaviours. Fe Reader should detect and quarantine these by default.

Default policy:

```text
JavaScript: disabled
Launch actions: blocked
Remote URI actions: prompt/open externally only
RichMedia: disabled unless sandboxed and approved
Embedded executable attachments: blocked/quarantined
SubmitForm: prompt and show destination
```

The firewall is not an antivirus product. Its job is to make active behaviours visible, controllable and auditable.

## Contracts

See:

```text
contracts/rust/time_machine.rs
contracts/rust/active_content_firewall.rs
schemas/active-content-finding.schema.json
```
