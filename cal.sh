#!/bin/bash
echo Date:
read date
echo Event:
read event 
echo Time
read tim 

echo $event, at, $tim, $date

if [ -f "events/$date.json" ]; then
    echo "$date exist"
    ADDEVENT="$(jq --arg m "$event" '.event[.event | length] |= . + $m' /home/blujay/code/rustful-notifications/events/$date.json > events/$date.tmp && mv events/$date.tmp events/$date.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$tim'"' /home/blujay/code/rustful-notifications/events/$date.json > events/$date.tmp && mv events/$date.tmp events/$date.json )"
else 
    echo "$date does not exist"
    echo '{ "event": [], "time": [] }' > "events/$date.json"
    ADDEVENT="$(jq --arg m "$event" '.event[.event | length] |= . + $m' /home/blujay/code/rustful-notifications/events/$date.json > events/$date.tmp && mv events/$date.tmp events/$date.json )"
    ADDTIME="$(jq '.time[.time | length] |= . + "'$tim'"' /home/blujay/code/rustful-notifications/events/$date.json > events/$date.tmp && mv events/$date.tmp events/$date.json )"
fi