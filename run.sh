#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )

# Update venv
"${SCRIPTPATH}/scripts/create_venv.sh"

# Create control fifo
if [[ -e "${SCRIPTPATH}/control_fifo" ]]; then
    rm "${SCRIPTPATH}/control_fifo"
fi
mkfifo "${SCRIPTPATH}/control_fifo"

# Add hooks to pianobar config
"${SCRIPTPATH}/scripts/add_hooks_to_pianobar.sh"

# Activate venv
source "${SCRIPTPATH}/.venv/bin/activate"

# Start processes
pianobar-websocket &
sleep 2
pianobar &

# Exit on failure
wait -n
echo "One of the background processes exited with '$?'."

echo "Killing other background processes ..."
BACKGROUND_JOBS=$(jobs -p)
if [ -n "${BACKGROUND_JOBS}" ]; then
    echo "Killing ${BACKGROUND_JOBS} ..."
    kill ${BACKGROUND_JOBS}
fi
echo "Exiting program."
