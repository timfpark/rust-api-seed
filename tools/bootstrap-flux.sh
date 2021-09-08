#!/bin/bash

# Clone workload cluster repo

cd /workspaces

echo $GITHUB_USER
echo $GITHUB_TOKEN

git clone https://$GITHUB_USER:$GITHUB_TOKEN@github.com/timfpark/workload-cluster-gitops
cd workload-cluster-gitops

export CODE_BRANCH=`git rev-parse --symbolic-full-name --abbrev-ref HEAD`
export GITOPS_CODESPACE_APP_PATH=codespaces/$RepositoryName
export GITOPS_USER_CODESPACE_PATH=$GITOPS_CODESPACE_APP_PATH/$GITHUB_USER
export GITOPS_BRANCH_CODESPACE_PATH=$GITOPS_USER_CODESPACE_PATH/$CODE_BRANCH

mkdir -p $GITOPS_USER_CODESPACE_PATH

# Template out codespace workload if it doesn't exist
if [ -d "$GITOPS_BRANCH_CODESPACE_PATH" ]
then
    echo "Tempalting out path"
    cp -r $GITOPS_CODESPACE_APP_PATH/template $GITOPS_BRANCH_CODESPACE_PATH

    git add -A $GITOPS_BRANCH_CODESPACE_PATH
    git commit -m "Template out $CODE_BRANCH branch for app $RepositoryName for codespace"
    git push origin main
fi

# flux bootstrap github --owner=$GITHUB_USER --components-extra=image-reflector-controller,image-automation-controller --repository=workload-cluster-gitops --branch=main --path=$GITOPS_BRANCH_CODESPACE_PATH --personal --network-policy=false
