# Merge Policy

A change may merge only when:

- the active phase gate passes;
- `fe_reader_core` has no forbidden dependencies;
- schemas updated if public JSON changed;
- CLI contract tests updated if output changed;
- high-risk operation tests cover failure and approval paths;
- platform integration changes document permission implications;
- distribution changes update package matrix;
- third-party dependency changes update fork/supply-chain notes.
