#!/bin/bash
template="$1"
new="$2"

if test ! -d $template -o -z "$new" -o -e "$new"; then
	echo "Usage: new.sh p2016_01 p2016_02"
	exit 1
fi

set -x
cp -fr ./$template ./$new 
gfind ./$new -type f -printf "gsed -i \"s/$template/$new/g\" %p\n" | bash
gsed -i "/members/ a \    \"$new\"," ./Cargo.toml
git add .
./watch.sh $new
