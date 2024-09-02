#!/bin/sh

# Check if a command is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <command> [args...]"
  exit 1
fi

# Execute the specified command
exec "$@"