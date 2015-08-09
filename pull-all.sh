#!/bin/sh

remotes=$(git remote)

for remote in $remotes; do
    echo "*** pulling $remote ***"
    git pull $remote :
    echo ""
done

echo "*** pushing ***"
git push
