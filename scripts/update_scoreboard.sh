#! /bin/bash

TMP_DIR="$PWD/scoreboard"

FILE_NAME=$1
GIT_URL=$2
STUDENT_ID=$3
SCORE=$4
STAGE_ID=$5

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

echo "/^$STUDENT_ID,/s/[^,]*/$SCORE/$STAGE_ID" $FILE_NAME
sed -i "/^$STUDENT_ID,/s/[^,]*/$SCORE/$STAGE_ID" $FILE_NAME


git commit -am "Update scoreboard for user $STUDENT_ID and stage $STAGE_ID"

git push

