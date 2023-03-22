#!/bin/sh

# Start Xvfb
# echo "Starting Xvfb"
# Xvfb :99 -ac -nocursor -screen 0 1900x1080x24 +extension RANDR +extension COMPOSITE +extension DOUBLE-BUFFER +extension XFIXES  -noreset +extension RENDER &
# Xvfb_pid="$!"
# export DISPLAY=:99
# echo "Waiting for Xvfb (PID: $Xvfb_pid) to be ready..."
# xdpyinfo -display "${DISPLAY}"
# while ! xdpyinfo -display "${DISPLAY}" > /dev/null 2>&1; do
#     sleep 0.1
# done
# echo "Xvfb is running."
# dbus-run-session -- sway &
# Execute passed command.
"openmobilemaps-rs"

cp glium-example-screenshot_framebuffer.png output/
# glxinfo | head -180
# trap "echo 'Stopping'; kill -SIGTERM $Xvfb_pid" SIGINT SIGTERM
# # Wait for process to end.
# kill $Xvfb_pid
# echo "Waiting for Xvfb (PID: $Xvfb_pid) to shut down..."
# wait $Xvfb_pid