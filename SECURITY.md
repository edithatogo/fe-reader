# Security Policy

## Supported Versions

| Version | Supported |
| --- | --- |
| `0.1.0-preview.1` | Security reports accepted; preview support only |

## Reporting Vulnerabilities

Use GitHub private vulnerability reporting when available for this repository.
If private reporting is unavailable, open a minimal public issue that says a
private security report is needed, without exploit details, private documents,
paths, credentials or payloads.

Do not publish malicious PDF fixtures that exploit active vulnerabilities
without maintainer review.

Security-sensitive areas include:

- active content execution and quarantine
- plugin permissions
- MCP/native/mobile/browser automation mutation
- redaction verification
- document hash matching and transaction journaling
- package signing, provenance and update manifests

## Handling Sensitive Fixtures

Do not attach private PDFs, document text, support bundles, credentials,
customer paths or exploit payloads to public issues or pull requests. Use
synthetic fixtures or maintainer-approved private transfer instead.

## Disclosure

Maintainers will triage reports, identify affected contracts or packages, and
publish release evidence once a fix is available. Preview releases may receive
contract fixes without production support guarantees.
