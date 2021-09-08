#!/bin/bash

echo "post-start start" >> ~/status

# this runs in background each time the container starts

# GITHUB_TOKEN=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 flux bootstrap github --owner=timfpark --components-extra=image-reflector-controller,image-automation-controller --repository=workload-cluster-gitops --branch=main --path=tim-dev --personal --network-policy=false

echo "post-start complete" >> ~/status
