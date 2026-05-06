# sigvault

`sigvault` is a focused Rust codebase around verify signed envelope canonicalization and replay guards. It is meant to be easy to inspect, run, and extend without a hosted service.

## Sigvault Walkthrough

I would read the project from the outside in: command, fixture, model, then roadmap. That keeps the security tooling idea grounded in files that can be checked locally.

## Reason For The Project

This is not a wrapper around a service. It is a self-contained project that shows how the model behaves when demand, capacity, latency, risk, and weight move in different directions.

## How It Is Put Together

The design is intentionally direct: parse or construct a signal, score it, classify it, and verify the expected branch. This makes the repository useful for studying security tooling behavior without needing a service or database unless the language project itself is SQL. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Data Notes

`pressure` is the first example I would inspect because it lands on the `review` path with a score of 77. The broader file also keeps `degraded` at -29 and `surge` at 232, which gives the model a useful low-to-high spread.

## Capabilities

- Uses fixture data to keep policy checks changes visible in code review.
- Includes extended examples for replay guards, including `surge` and `degraded`.
- Documents claim validation tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.

## Getting It Running

Use a normal shell with Rust available on `PATH`. The verifier is written as a PowerShell script because the portfolio was assembled on Windows.

## Check The Work

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Where Things Live

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Tradeoffs

The fixture set is deliberately small. That keeps the review surface clear, but it also means the model should not be treated as a complete domain simulator.

## Possible Extensions

- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add a short report command that prints the score breakdown for a single scenario.
- Add malformed input fixtures so the failure path is as visible as the happy path.
- Add one more security tooling fixture that focuses on a malformed or borderline input.

## Command Examples

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.
