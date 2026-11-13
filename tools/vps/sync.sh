#!/bin/sh
# From this PC: copy the mod's sources to the build VPS and stage them into the
# tombstone worktree of the pinned Horizon clone (/opt/build/wt/tombstone).
set -e
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VPS="root@<VPS>"
KEY="$HOME<SSH_KEY>"
ssh -o BatchMode=yes -i "$KEY" "$VPS" 'rm -rf /opt/build/mods/tombstone && mkdir -p /opt/build/mods/tombstone'
tar -C "$ROOT" --exclude=./target --exclude=__pycache__ -cf - . | \
  ssh -o BatchMode=yes -i "$KEY" "$VPS" 'tar --no-same-owner -C /opt/build/mods/tombstone -xf - && bash /opt/build/mods/tombstone/tools/vps/stage.sh'
