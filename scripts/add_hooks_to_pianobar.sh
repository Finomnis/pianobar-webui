#!/bin/bash

# This script creates or updates the virtual environment named '.venv'
# in the repository root.
# To install a new package in the virtual environment, modify the
# 'requirements.txt' in the repository root and run this script again.

set -e -u
SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )
REPOROOT=$( cd "$SCRIPTPATH/.."; pwd -P )
PIANOBAR_CONFIG="${HOME}/.config/pianobar/config"

sed -i '/^\s*event_command\s*=.*$/d' "${PIANOBAR_CONFIG}"
sed -i '/^\s*fifo\s*=.*$/d' "${PIANOBAR_CONFIG}"

echo "event_command = pianobar-websocket-handle-events" >>"${PIANOBAR_CONFIG}"
echo "fifo = $REPOROOT/control_fifo" >>"${PIANOBAR_CONFIG}"
