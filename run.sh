#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )

# Update venv
echo "Creating virtual environment ..."
"${SCRIPTPATH}/scripts/create_venv.sh"

# Create control fifo
echo "Creating control fifo ..."
if [[ -e "${SCRIPTPATH}/control_fifo" ]]; then
    rm "${SCRIPTPATH}/control_fifo"
fi
mkfifo "${SCRIPTPATH}/control_fifo"

# Add hooks to pianobar config
echo "Registering pianobar hooks ..."
"${SCRIPTPATH}/scripts/add_hooks_to_pianobar.sh"

# Activate venv
echo "Activating virtual environment ..."
source "${SCRIPTPATH}/.venv/bin/activate"

# Start processes
echo "Starting websocket server ..."
pianobar-websocket &
sleep 2

echo "Starting pianobar ..."
pianobar &

# Exit on failure
echo "Startup procedure finished."
wait -n
echo "One of the background processes exited with '$?'."

echo "Killing other background processes ..."
BACKGROUND_JOBS=$(jobs -p)
if [ -n "${BACKGROUND_JOBS}" ]; then
    echo "Killing ${BACKGROUND_JOBS} ..."
    kill ${BACKGROUND_JOBS}
fi
echo "Exiting program."
