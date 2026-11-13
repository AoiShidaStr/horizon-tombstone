#!/bin/sh
# From this PC: pack the .hmod and .hnmod locally, copy them to the build VPS and
# run cargo-horizon-mod's own reader over each (built there from the pinned
# Horizon revision: /opt/build/target/debug/cargo-horizon-mod).
set -e
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VPS="root@<VPS>"
KEY="$HOME<SSH_KEY>"
sh -c "cd '$ROOT' && python -I tools/pack.py"
scp -q -o BatchMode=yes -i "$KEY" "$ROOT/target/tombstone.hmod" "$ROOT/target/tombstone.hnmod" "$VPS:/opt/build/"
ssh -o BatchMode=yes -i "$KEY" "$VPS" 'bash -s' <<'EOF'
set -o pipefail
cd /opt/build
for f in tombstone.hmod tombstone.hnmod; do
  echo "== $f"
  ./target/debug/cargo-horizon-mod horizon-mod check $f 2>&1 | tail -30
  echo "exit=$?"
done
EOF
