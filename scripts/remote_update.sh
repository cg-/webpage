#!/bin/bash
# This script is run on the remote server when a new commit is
# added to the main branch by the Github action.
#
# It builds the container, then cleans up the workspace since
# it is no longer needed.
echo Starting update at $(date -u) >> /tmp/update_log

cd ~/workspace
podman stop webpage
podman rm webpage
podman system prune -f --all
podman build -t webpage .
podman run -d -p 8080:8080 --name webpage -t webpage
rm -rf ~/workspace

echo Completed update at $(date -u) >> /tmp/update_log
