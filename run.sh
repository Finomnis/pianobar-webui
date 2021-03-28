#!/bin/bash

set -eu

SCRIPTPATH=$( cd "$(dirname "$(readlink -f "$0")")"; pwd -P )

# Set working directory
cd "$SCRIPTPATH"

# Create build directory
mkdir -p build

# Check if rust is installed
if ! command -v cargo &> /dev/null
then
    echo "The Rust compiler does not seem to be installed on this system."
    echo "It is required for this program to run."
    echo "For more information, visit: https://rustup.rs/"
    exit 1
fi

# Compile the server binaries
cargo install --path pianobar_webserver --root build

# Check if node is installed
if ! command -v npm &> /dev/null
then
    echo "The node package manager does not seem to be installed on this system."
    echo "It is required for this program to run."
    echo "For more information, visit: https://github.com/nvm-sh/nvm/blob/master/README.md"
    exit 1
fi

# Compile the webui
(cd pianobar_webui; npm ci; npm run build)
rm -rf build/html
cp -r pianobar_webui/build build/html

# Check if pianobar is installed
if ! command -v pianobar &> /dev/null
then
    echo "Pianobar does not seem to be installed on this system."
    echo "It is required for this program to run."
    echo "For more information, visit: https://github.com/PromyLOPh/pianobar"
    exit 1
fi

# Start processes
echo "Starting server ..."
echo "When startup finishes, it should be reachable at: http://127.0.0.1:3030"
build/bin/pianobar_webserver -v -w build/html -p 3030
