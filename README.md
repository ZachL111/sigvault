# sigvault

`sigvault` is a compact Rust repository for security tooling, centered on this goal: Verify signed envelope canonicalization and replay guards.

## Project Rationale

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how trust boundary and replay exposure should influence a review result.

## Sigvault Review Notes

`stale` and `baseline` are the cases worth reading first. They show the optimistic and cautious ends of the fixture.

## Feature Set

- `fixtures/domain_review.csv` adds cases for trust boundary and claim drift.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/sigvault-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `trust boundary` and `trust boundary`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Architecture

The core code exposes a scoring path and the added review layer uses `signal`, `slack`, `drag`, and `confidence`. The domain terms are `trust boundary`, `claim drift`, `replay exposure`, and `policy width`.

The Rust addition stays small enough to inspect in one sitting.

## Usage

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Test Command

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Next Improvements

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
