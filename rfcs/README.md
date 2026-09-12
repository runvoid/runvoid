# Runvoid RFC Process

The "RFC" (request for comments) process provides a consistent and controlled path for new features and design proposals to enter the Runvoid programming language and ecosystem.

## When to Use an RFC
You need to write an RFC for:
- Substantial changes to the language syntax, grammar, or semantics.
- Changes to the compilation pipeline, standard library, or ABI contracts.
- Major tooling features (such as package management, manifest structure, or debugger protocols).

You do **not** need an RFC for:
- Bug fixes and minor refactoring.
- Diagnostics, warnings, or error message improvements.
- Documentation, book updates, or test suite expansion.

## The Life Cycle of an RFC

```
[Draft] -> [Proposed] -> [Accepted] -> [Implemented]
                     \-> [Rejected]
```

1. **Draft**: The author writes the proposal following the template below and opens a discussion or draft PR.
2. **Proposed**: An official pull request is opened against the `rfcs/` directory with filename `0000-feature-name.md`.
3. **Accepted**: The core maintainers and community review the proposal, address concerns, assign an RFC number, and merge the RFC.
4. **Implemented**: Code is implemented in the compiler and standard library, linking back to the accepted RFC.
5. **Rejected**: If the proposal contradicts language design goals or is not viable, the PR is closed or marked rejected.

## RFC Template

```markdown
# RFC 0000: Feature Name

- Feature Name: `feature_name`
- Start Date: YYYY-MM-DD
- RFC PR: [runvoidLanguage/pull/000](https://github.com/runvoidLanguage/pull/000)
- Tracking Issue: [runvoidLanguage/issues/000](https://github.com/runvoidLanguage/issues/000)

## Summary
Brief explanation of the proposal.

## Motivation
Why are we doing this? What use cases does it support? What is the expected outcome?

## Guide-level Explanation
Explain the proposal as if it were part of the official Runvoid documentation or book.
Include code samples and real-world scenarios.

## Reference-level Explanation
Technical details, grammar changes, AST transformations, ABI implications, and error handling.

## Drawbacks
Why should we *not* do this?

## Rationale and Alternatives
- What other designs were considered?
- Why is this design superior?

## Prior Art
How do other languages solve this problem (Rust, Go, Python, Zig, C)?

## Unresolved Questions
What parts of the design remain open or require prototyping before stabilization?
```

## RFC Directory

| RFC | Title | Status |
|---|---|---|
| [0001](0001-project-manifests-and-modules.md) | Project Manifests (`runvoid.toml`) and Project Structure | Implemented |
