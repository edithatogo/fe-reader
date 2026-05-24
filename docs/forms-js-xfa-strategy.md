# Forms, JavaScript Actions and XFA Strategy

## Forms

Prioritise AcroForms:

- inspect fields
- fill fields
- validate required fields
- flatten fields into page content
- generate form summary
- later: author/edit fields

## PDF JavaScript

PDF JavaScript is not a general app automation feature in Fe Reader. It is a compatibility risk and potential security risk.

Policy:

- recognise actions
- list actions in inspection UI
- disable execution by default
- never execute in core
- later: sandboxed limited execution only if justified by real forms compatibility

## XFA

XFA is treated as legacy/special-case:

- detect XFA
- warn user when a form depends on XFA
- render/fill support is not MVP
- use external/reference viewers during testing when needed
- do not let XFA scripts mutate documents or access external resources

## Attachments and launch actions

Embedded files may be extracted only after explicit user approval. Launch/open actions are disabled by default.
