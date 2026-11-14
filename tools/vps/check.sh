#!/bin/bash
# Run on the build VPS (after stage.sh). Type-check the mod crate, run its unit
# tests, then check the dedicated-server engine (horizon-native-game without
# default features) and the client flavour (default features).
. ~/.cargo/env
export CARGO_TARGET_DIR=/opt/build/target CARGO_BUILD_JOBS=4
cd /opt/build/wt/tombstone
bash /opt/build/mods/tombstone/tools/vps/stage.sh
echo "== mod crate check (server flavour)"; date
nice -n 10 cargo check -p mod-tombstone --message-format short 2>&1 | grep -vE "^\s+(Compiling|Checking)"
echo "check_exit=${PIPESTATUS[0]}"
if [ "$1" = full ]; then
  echo "== mod tests"; date
  nice -n 10 cargo test -p mod-tombstone --message-format short 2>&1 | grep -vE "^\s+(Compiling|Checking)" | tail -40
  echo "test_exit=${PIPESTATUS[0]}"
  echo "== dedicated-server engine build (no default features)"; date
  nice -n 10 cargo build -p horizon-native-game --no-default-features --message-format short 2>&1 | grep -vE "^\s+(Compiling|Checking)"
  echo "server_exit=${PIPESTATUS[0]}"
  echo "== client flavour check (default features)"; date
  nice -n 10 cargo check -p horizon-native-game --message-format short 2>&1 | grep -vE "^\s+(Compiling|Checking)"
  echo "client_exit=${PIPESTATUS[0]}"
fi
date; echo DONE
