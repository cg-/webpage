#!/bin/bash
echo Starting update at $(date -u) >> /tmp/update_log

cd ~/workspace
podman stop webpage
podman build -t webpage .
podman run --name webpage -d -t webpage

echo Completed update at $(date -u) >> /tmp/update_log
