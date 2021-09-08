#!/bin/bash

echo "post-start start" >> ~/status

# this runs in background each time the container starts

k3d cluster create --registry-use k3d-registry.localhost:5500 --config .devcontainer/k3d.yaml --k3s-server-arg "--no-deploy=traefik" --k3s-server-arg "--no-deploy=servicelb"

# wait for cluster to be ready
kubectl wait node --for condition=ready --all --timeout=60s
sleep 5
kubectl wait pod -A --all --for condition=ready --timeout=60s

echo "post-start complete" >> ~/status
