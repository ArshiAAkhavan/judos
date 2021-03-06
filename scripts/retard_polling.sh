#!/bin/bash
TMP_DIR="$PWD/poll_tmp"
mkdir -p $TMP_DIR

GRADE_FILE_NAME=grade.txt

GIT_URL=$1
REPO_DIR_RELATIVE=`basename "${GIT_URL%.git}"`

REPO_DIR_ABSOLUTE=$TMP_DIR/$REPO_DIR_RELATIVE
HW_PATH=$REPO_DIR_ABSOLUTE/$2

function log {
  echo $@ 1>&2
}

if [ -d $REPO_DIR_ABSOLUTE ];then
  log "repo was already there"
  cd $REPO_DIR_ABSOLUTE
  git pull 1>&2
  if [ $? -eq 128 ]; then
    log "merge accured temprory delete dir untill fix is ready"
    cd ../
    rm -rf $REPO_DIR_ABSOLUTE
    cd $TMP_DIR
    git clone $GIT_URL 1>&2
    cd $REPO_DIR_ABSOLUTE
  fi

else
  log "first time encountering repo"
  cd $TMP_DIR
  git clone $GIT_URL 1>&2
  cd $REPO_DIR_ABSOLUTE
fi

if [ ! -d $HW_PATH ];then
  log "$HW_PATH doesnt exist"
  exit -1
fi
cd $HW_PATH
log $PWD
latest_grade_commit=`env -i git log --format="%ad" --date=raw --author=$(git config user.name) --follow -- ./$GRADE_FILE_NAME | awk '{print$1}' | head -n1`
latest_commit=`env -i git log --format='%ad' --date=raw --invert-grep --author=$(git config user.name) --follow -- ./ | awk '{print$1}' | head -n1`
valid_commit_upper_bound=`date +%s`
greater=`printf "${latest_commit}\n${valid_commit_upper_bound}" | sort -r | head -n1`
if [[ "$greater" == "$latest_commit" ]]; then
  exit 127
fi

greater=`printf "${latest_commit}\n${latest_grade_commit}" | sort -r | head -n1`
log latest_score is  $latest_grade_commit
log latest_commit is $latest_commit
log greater is       $greater

exit_code=$((greater - latest_commit))
env -i git log --format='%h' --date=raw --invert-grep --author=$(git config user.name) --follow -- ./ | awk '{print$1}' | head -n1
exit $exit_code
