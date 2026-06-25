# Track BL: Advanced PDF Family Parity

## Overview

Implement advanced PDF family support and documented limitations for standards, attachments, portfolios, layers, active content, encryption, permissions, prepress and multimedia.

## Functional Requirements

- Inspect and report PDF/A, PDF/UA, PDF/X and PDF 2.0 markers through external validators where available.
- Inspect attachments, embedded files, portfolios and associated files with policy-aware extraction planning.
- Inspect optional content groups/layers and expose visibility state where renderer support exists.
- Quarantine active content, launch actions, rich media and embedded executables by default.
- Inspect encryption, permissions and signatures with clear mutation risk warnings.
- Inspect color spaces, ICC, output intents, spot colors, overprint and font embedding/subsetting diagnostics.

## Non-Functional Requirements

- Active content remains disabled/quarantined by default.
- Extraction and mutation of embedded files require policy approval and audit receipts.
- Standards claims require oracle output or documented limitations.

## Acceptance Criteria

- Advanced PDF families appear in the exhaustive parity registry with evidence or limitations.
- UI and CLI expose diagnostics without unsafe execution.
- Release notes and marketing claims remain aligned with evidence.

## Out of Scope

- Arbitrary JavaScript execution.
- Trusting embedded executables.
- Claiming full prepress certification without external validation.
