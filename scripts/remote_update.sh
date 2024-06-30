#!/bin/bash
echo Starting update at $(date -u) >> /tmp/update_log

cd ~/workspace
podman stop webpage
podman build -t webpage .
podman run -t webpage --name webpage

echo Completed update at $(date -u) >> /tmp/update_log
