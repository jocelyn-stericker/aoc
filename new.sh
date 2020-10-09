#!/bin/bash
template="$1"
new="$2"

if test ! -d $template -o -z "$new" -o -e "$new"; then
	echo "Usage: new.sh p2016_01 p2016_02"
	exit 1
fi

set -x
cp -fr ./$template ./$new 
find ./p2016_02 -type f -printf "sed -i \"s/$template/$new/g\" %p\n" | bash
sed -i "/members/ a \    \"$new\"," ./Cargo.toml
git add .
./watch.sh $new
