#!/bin/bash

# Script to determine the period to backup for, i.e. the previous month
# Output: YYYY-mm

set -e 

# Determine the period to operate on for Guppy

printf -v currentMonth '%(%m)T' -1
printf -v currentYear '%(%Y)T' -1

# Go back to last year if its currently january
if [ $currentMonth -eq 1 ]
then
    prevMonth="12"
    prevYear="$(($currentYear - 1))"
else
    prevMonth="$(($currentMonth - 1))"
    prevYear="$currentYear"
fi

# Make sure we keep two digits
if [ $prevMonth -lt 10 ]
then
	prevMonth="0$prevMonth"
fi

echo "$prevYear-$prevMonth"
