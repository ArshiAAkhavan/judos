#! /bin/bash

TMP_DIR="$PWD/scoreboard"
mkdir -p $TMP_DIR

SCOREFILE_NAME=$1
GIT_URL=$2
STUDENT_ID=$3
SCORE=$4
STAGE_NAME=$5
COMMITFILE_NAME=$6
COMMIT=$7

REPO_DIR_RELATIVE=`basename "${GIT_URL%.git}"`
REPO_DIR_ABSOLUTE=$TMP_DIR/$REPO_DIR_RELATIVE

function log {
  echo $@ 1>&2
}

if [ -d $REPO_DIR_ABSOLUTE ];then
  log "repo was already there"
  cd $REPO_DIR_ABSOLUTE
  git pull
else
  log "first time encountering repo"
  cd $TMP_DIR
  git clone $GIT_URL
  cd $REPO_DIR_ABSOLUTE
fi
STAGE_ID=`cat $SCOREFILE_NAME| head -n1 | tr ',' '\n' | awk "/$STAGE_NAME/ {print FNR}"`
echo "/^$STUDENT_ID,/s/[^,]*/$SCORE/$STAGE_ID" $SCOREFILE_NAME
sed -i "/^$STUDENT_ID,/s/[^,]*/$SCORE/$STAGE_ID" $SCOREFILE_NAME
sed -i "/^$STUDENT_ID,/s/[^,]*/$COMMIT/$STAGE_ID" $COMMITFILE_NAME


git commit -am "Update scoreboard for user $STUDENT_ID and stage $STAGE_ID"

git push

