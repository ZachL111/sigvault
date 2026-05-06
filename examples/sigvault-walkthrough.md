# Sigvault Walkthrough

I use this file as a small checklist before changing the Rust implementation.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | trust boundary | 153 | ship |
| stress | claim drift | 187 | ship |
| edge | replay exposure | 190 | ship |
| recovery | policy width | 190 | ship |
| stale | trust boundary | 216 | ship |

Start with `stale` and `baseline`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`stale` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
