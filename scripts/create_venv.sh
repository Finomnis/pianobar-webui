#!/bin/bash

# This script creates or updates the virtual environment named '.venv'
# in the repository root.
# To install a new package in the virtual environment, modify the
# 'requirements.txt' in the repository root and run this script again.

set -e -u
SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )
REPOROOT=$( cd "$SCRIPTPATH/.."; pwd -P )

venvpath="$REPOROOT/.venv"

# Create environment if it doesn't exist yet, or update it otherwise:
if [[ ! -d "$venvpath" ]]; then
    python3 -m venv "$venvpath"
else
    python3 -m venv --upgrade "$venvpath"
fi

# Activate environment
source "$venvpath/bin/activate"

# Install newer pip and requirements
pip3 install --upgrade pip==21.0.1
pip3 install --upgrade wheel==0.36.2

(cd "$REPOROOT" && pip3 install -r ./requirements.txt)

echo ""
echo "Setting up environment finished!"
echo "Activate environment:"
echo "source ${venvpath}/bin/activate"
