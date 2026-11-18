#!/bin/sh
set -e

echo "Initializing bajam..."

# Initialize database
/app/setup

echo "Starting bajam daemon..."

exec /app/bajam-daemon

