# Miri twins for the Tree Borrows tests

Each `.rs` file here is a plain-Rust twin of a VeriFast test. Its first line, `// miri: ub` or `// miri: ok`,
states whether Miri with `-Zmiri-tree-borrows` must report undefined behaviour. VeriFast's verdict must be
sound with respect to Miri's: if VeriFast accepts the annotated test, the twin must be `ok`.

Run them with `./run-miri.sh` (or `./run-miri.sh FILE.rs…`) from the Nix dev shell, which provides
`cargo miri`. They are not part of `testsuite.mysh`.

These are intended for the development of the aliasing rules in VeriFast, and should likely not last longer.