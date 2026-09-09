#!/bin/sh
# Task-local browser setup for the synthetic implementation smoke.
set -eu
mkdir -p .runtime-scratch
python3 -c 'from pathlib import Path; root=Path.cwd(); (root/".runtime-scratch/fonts.conf").write_text("<?xml version=\"1.0\"?><fontconfig><dir>/nvme/development/polyorama/crates/polyorama-ui-egui/assets/fonts</dir><cachedir>"+str(root/".runtime-scratch/font-cache")+"</cachedir></fontconfig>")'
export FONTCONFIG_FILE="$PWD/.runtime-scratch/fonts.conf"
export LD_LIBRARY_PATH="/home/rob/pixi/.pixi/envs/default/lib:$PWD/.runtime-scratch/alsa/usr/lib"
exec /home/rob/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome \
  --headless --no-sandbox --disable-dev-shm-usage --disable-background-networking \
  --disable-breakpad --disable-crash-reporter --use-gl=angle --use-angle=swiftshader \
  --enable-unsafe-swiftshader --remote-debugging-port=9317 \
  --user-data-dir="$PWD/.runtime-scratch/inspection-chrome" --window-size=1100,900 about:blank
