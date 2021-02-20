#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )

# Update venv
echo "Creating virtual environment ..."
"${SCRIPTPATH}/scripts/create_venv.sh"

# Activate venv
echo "Activating virtual environment ..."
source "${SCRIPTPATH}/.venv/bin/activate"

# Add hooks to pianobar config
echo "Registering pianobar hooks ..."
"${SCRIPTPATH}/scripts/add_hooks_to_pianobar.sh"

# Adding cleanup hooks
trap "jobs -p | xargs -r kill" SIGINT SIGTERM EXIT

# Start processes
echo "Starting websocket server ..."
pianobar-websocket &

# Exit on failure
echo "Startup procedure finished."
wait -n
echo "One of the background processes exited with '$?'."

echo "Exiting program."
