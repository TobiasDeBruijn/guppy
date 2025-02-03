#!/bin/bash

# Wrapper script around the guppy.sh script.
# Not necessarily required, but makes invocation easier.

set -e

if [ -z "$1" ]
then
   echo "Usage: ./guppy-prive.sh <period>"
   exit 0
fi

# guppy.sh parent prefix period iam-email drive-folder-id
/srv/guppy/guppy.sh prive vaultwarden $1 guppy-prive@guppy-392611.iam.gserviceaccount.com 1k1KRaJ-vwgFKinWCBIiCR6sYqtSJboUU