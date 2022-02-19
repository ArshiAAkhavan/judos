#!/bin/bash

TMP_DIR="$PWD/tmp"
DIR=$TMP_DIR/$RANDOM
TARGET_DIR_IN_IMAGE=/opt/content

IMAGE_NAME=$1
GIT_URL=$2
COPY_FROM=$DIR/$3


function log {
  echo $@ 1>&2
}
# clone the repo
git clone $GIT_URL $DIR 1>/dev/null

# copy repo to judge image
cid=`docker container create $IMAGE_NAME` 
docker cp $COPY_FROM $cid:$TARGET_DIR_IN_IMAGE

# run the judge
docker container start $cid 1>/dev/null
docker wait $cid 1>/dev/null

# updating the score
docker logs $cid > $COPY_FROM/score.txt

cd $COPY_FROM
git add score.txt
git commit -m "update score"
log pushing:
git push
log pushed

# clean up
cd /
rm -rf $DIR

# return judge logs as output
docker logs $cid 

# delete judge container
docker container rm $cid 1>/dev/null

