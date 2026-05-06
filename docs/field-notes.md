# Field Notes

The fixture is small on purpose, which makes each domain case carry real weight.

The domain cases cover `trust boundary`, `claim drift`, `replay exposure`, and `policy width`. They sit beside the smaller starter fixture so the project has both a compact scoring check and a domain-flavored review check.

`stale` is the strongest case at 216 on `trust boundary`. `baseline` is the cautious anchor at 153 on `trust boundary`.

The extra check gives the repository a behavior path that can fail for a domain reason, not only a syntax reason.
