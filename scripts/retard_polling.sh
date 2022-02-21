#!/bin/bash
TMP_DIR="$PWD/poll_tmp"
GRADE_FILE_NAME=grade.txt

GIT_URL=$1
REPO_DIR_RELATIVE=`basename "${GIT_URL%.git}"`

REPO_DIR_ABSOLUTE=$TMP_DIR/$REPO_DIR_RELATIVE
HW_PATH=$REPO_DIR_ABSOLUTE/$2

function log {
  echo $@ 1>&2
}
function latest_update {
  cd $HW_PATH
  echo ``
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

cd $HW_PATH
log $PWD
latest_grade_commit=`env -i git log --format="%ad" --date=raw --author=$(git config user.name) --follow -- ./$GRADE_FILE_NAME | awk '{print$1}' | head -n1`
latest_commit=`env -i git log --format='%ad' --date=raw --invert-grep --author=$(git config user.name) --follow -- ./ | awk '{print$1}' | head -n1`

greater=`printf "${latest_commit}\n${latest_grade_commit}" | sort -r | head -n1`
log latest_score is  $latest_grade_commit
log latest_commit is $latest_commit
log greater is       $greater

exit_code=$((greater - latest_commit))
exit $exit_code
