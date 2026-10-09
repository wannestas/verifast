#!/usr/bin/env bash
# Runs each Miri twin in this directory under Tree Borrows and checks Miri's verdict
# against the file's `// miri: ub|ok` header. Exits non-zero on any mismatch.
# Usage: ./run-miri.sh [FILE.rs...]   (default: all *.rs here)
set -u
cd "$(dirname "$0")"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/src"
cat > "$work/Cargo.toml" <<'TOML'
[package]
name = "miri-twin"
version = "0.0.0"
edition = "2024"
TOML
export MIRIFLAGS="-Zmiri-tree-borrows ${MIRIFLAGS:-}"
files=("$@"); [ ${#files[@]} -eq 0 ] && files=(*.rs)
failed=0
for f in "${files[@]}"; do
  expected=$(sed -nE '1s;^// miri: (ub|ok)$;\1;p' "$f")
  if [ -z "$expected" ]; then echo "$f: missing '// miri: ub|ok' header"; failed=1; continue; fi
  cp "$f" "$work/src/main.rs"
  out=$(cargo miri run -q --manifest-path "$work/Cargo.toml" 2>&1)
  if grep -q "Undefined Behavior" <<<"$out"; then actual=ub
  elif [ -n "$(grep -E '^error(\[|:)' <<<"$out")" ]; then actual=error
  else actual=ok; fi
  if [ "$actual" = "$expected" ]; then echo "$f: $actual (as expected)"
  else echo "$f: expected $expected, got $actual"; echo "$out" | sed 's/^/    /'; failed=1; fi
done
exit $failed
