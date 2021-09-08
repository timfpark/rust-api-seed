#!/bin/bash

echo "post-create start" >> ~/status

# this runs in background after UI is available

# (optional) upgrade packages
sudo apt-get update
sudo apt-get upgrade -y
sudo apt-get autoremove -y
sudo apt-get clean -y

# add your commands here

curl -s https://fluxcd.io/install.sh | sudo bash
chmod 777 /usr/local/bin/flux

echo "post-create complete" >> ~/status
