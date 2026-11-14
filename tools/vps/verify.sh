#!/bin/bash
set -euo pipefail

cd /opt/build
log=/opt/build/tombstone-rebuild.log
bash /opt/build/mods/tombstone/tools/vps/check.sh full 2>&1 | tee "$log"

for phase in check test server client; do
  grep -qx "${phase}_exit=0" "$log"
done

./target/debug/cargo-horizon-mod horizon-mod check tombstone.hmod
./target/debug/cargo-horizon-mod horizon-mod check tombstone.hnmod
