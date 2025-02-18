#!/bin/bash
# Script to install GFPing on Linux.
# Please run this script in the Terminal application.
# Ensure the script is being run with root privileges.

if [[ $EUID -ne 0 ]]; then
  echo "💀 This script must be run as root." >&2
  exit 1
fi

# Check if the binary exists.
if [[ ! -f "gfping" ]]; then
  echo " 💀 'gfping' binary not found in the current directory. Please build or download the binary." >&2
  exit 1
fi

# Copy the binary to /usr/local/bin
cp gfping /usr/local/bin/
chmod +x /usr/local/bin/gfping

# Notify the user of successful installation
echo "😊 GFPing has been installed successfully!"
echo "😊 You can now run it from any directory using: 'gfping www.website.com'."

