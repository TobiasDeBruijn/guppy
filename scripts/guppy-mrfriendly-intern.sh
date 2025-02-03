#!/bin/bash

# Wrapper script around the guppy.sh script.
# Not necessarily required, but makes invocation easier.

set -e

if [ -z "$1" ]
then
    echo "Usage: ./guppy-mrfriendly-intern.sh <period>"
    exit 0
fi

# guppy.sh parent prefix period iam-email drive-folder-id
/srv/guppy/guppy.sh mrfriendly-intern espocrm $1 guppy-mrfriendly@guppy-392611.iam.gserviceaccount.com 1mYjgHYXPNpuv7GgB1d2HygVv5W-HUkS3