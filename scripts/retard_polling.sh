#!/bin/bash
TMP_DIR="$PWD/poll_tmp"
GIT_URL=$1
REPO_DIR_RELATIVE=`basename "${GIT_URL%.git}"`

REPO_DIR_ABSOLUTE=$TMP_DIR/$REPO_DIR_RELATIVE
HW_PATH=$REPO_DIR_ABSOLUTE/$2

function log {
  echo $@ 1>&2
}
function latest_update {
  cd $HW_PATH
  for file in `ls`
  do
    log "checking $file"
    if [ "$file" = "score.txt" ];then
      continue
    fi
    echo `env -i git log --format='%ad' --date=raw --follow -- $HW_PATH/$file | awk '{print$1}' | head -n1`
  done
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

log $HW_PATH
latest_score=`(cd $HW_PATH && env -i git log --format="%ad" --date=raw --follow -- $HW_PATH/score.txt | awk '{print$1}' | head -n1)`
latest_commit=`latest_update | sort | uniq | sort -r | head -n1`

greater=`printf "${latest_commit}\n${latest_score}" | sort -r | head -n1`
log latest_score is  $latest_score
log latest_commit is $latest_commit
log greater is       $greater

exit_code=$((greater - latest_commit))
exit $exit_code