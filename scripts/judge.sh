#!/bin/bash

TMP_DIR="$PWD/tmp"
mkdir -p $TMP_DIR

DIR=$TMP_DIR/$RANDOM
GRADE_FILE_NAME=grade.txt

IMAGE_NAME=$1
GIT_URL=$2
COPY_FROM=$DIR/$3
TARGET_DIR_IN_IMAGE=$4
RESULT_PATH=$5


function log {
  echo $@ 1>&2
}
# clone the repo
git clone $GIT_URL $DIR 1>/dev/null
commit_hash=$(cd $DIR && env -i git log --format="%h" | head -n1)

# copy repo to judge image
cid=`docker container create $IMAGE_NAME` 
docker cp $COPY_FROM $cid:$TARGET_DIR_IN_IMAGE

# run the judge
docker container start $cid 1>/dev/null
docker wait $cid 1>/dev/null

# updating the score
cd $COPY_FROM
docker cp $cid:/$RESULT_PATH $GRADE_FILE_NAME 
echo "epoch: $(date +%s)" >> $GRADE_FILE_NAME
git add $GRADE_FILE_NAME > /dev/null
git commit -m "update grade" > /dev/null
log pushing:
git push > /dev/null
log pushed 

# clean up
cd /
rm -rf $DIR

# return judge logs as output
docker logs $cid 

# delete judge container
docker container rm $cid 1>/dev/null

