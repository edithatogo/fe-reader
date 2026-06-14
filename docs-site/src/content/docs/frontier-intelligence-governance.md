---
title: Frontier Intelligence Governance
description: Governance for optional local ML, RAG and intelligent assistance previews.
---

# Frontier Intelligence Governance

The `frontier_intelligence_preview` gate keeps optional local ML, RAG and
intelligent assistance disabled by default. It does not block desktop stable
launch.

Preview promotion requires privacy review, security review, synthetic or public
evaluation evidence, resource-budget evidence, explicit opt-in controls, ADR
approval and a rollback plan.

Run the governance check from the repository root:

```bash
python3 scripts/frontier_intelligence_governance_check.py
```

The detailed governance policy lives in
[`docs/frontier-intelligence-governance.md`](https://github.com/edithatogo/fe-reader/blob/main/docs/frontier-intelligence-governance.md).
