#!/bin/bash

# General wrapper around the guppy program.
# Packages and compresses the source files and uploads it to Google Drive 


set -e

USAGE="Usage: guppy.sh <parent> <prefix> <period> <IAM email> <GDrive Folder ID>"
if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ -z "$4" ] || [ -z "$5" ]
then
    echo $USAGE
    exit 0
fi

# Read arguments
PARENT=$1
PREFIX=$2
PERIOD=$3
EMAIL=$4
GDRIVE_FOLDER=$5

# Define the source
SOURCE_TARS="/mnt/backups/$PARENT/$PREFIX-$PERIOD-*.tar.gz"
# Output directory
OUTPUT_TAR=/tmp/$PREFIX-$PERIOD.tar

# Package and compress source into target
tar -cvf $OUTPUT_TAR $SOURCE_TARS

# Upload to Google Drive
guppy \
    --pem /srv/guppy/$PARENT.pem \
    --email $EMAIL \
    upload \
    --folder $GDRIVE_FOLDER \
    --team-drive 0AMxPSXTCWJtKUk9PVA \
    --source $OUTPUT_TAR

# Clean up sources and output tar
rm -rf $OUTPUT_TAR $SOURCE_TARS
