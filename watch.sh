#!/bin/bash
project="$1"

if test ! -d $project; then
	echo "Usage: watch.sh p2016_01"
	exit 1
fi

function commit {
	git commit -a -m "$project"
}

trap commit SIGINT

set -x

cargo watch -x "test -p $project"
