#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )

# Update venv
"${SCRIPTPATH}/scripts/create_venv.sh"

# Activate venv
source "${SCRIPTPATH}/.venv/bin/activate"

# Start processes
pianobar-websocket-simulate-events &
sleep 20 &

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
