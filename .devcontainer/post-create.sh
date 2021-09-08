#!/bin/bash

echo "post-create start" >> ~/status

# this runs in background after UI is available

# (optional) upgrade packages
# sudo apt-get update
# sudo apt-get upgrade -y
# sudo apt-get autoremove -y
# sudo apt-get clean -y

# add your commands here

curl -s https://fluxcd.io/install.sh | sudo bash
sudo chmod a+x /usr/local/bin/flux

k3d cluster create --registry-use k3d-registry.localhost:5500 --config .devcontainer/k3d.yaml --k3s-server-arg "--no-deploy=traefik" --k3s-server-arg "--no-deploy=servicelb"

# wait for cluster to be ready
kubectl wait node --for condition=ready --all --timeout=60s
sleep 5
kubectl wait pod -A --all --for condition=ready --timeout=60s

echo "post-create complete" >> ~/status
