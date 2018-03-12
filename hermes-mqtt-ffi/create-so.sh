#!/bin/bash -e

USAGE="Usage: $0 <hermes_static_lib.a>"

STATIC_LIB=${1?$USAGE}

[ -f "$STATIC_LIB" ] || ( echo "could not find file $STATIC_LIB" ; exit 1 )


DIR=`dirname "$STATIC_LIB"`
STATIC_FILE_NAME=`basename "$STATIC_LIB"`
BASE_NAME=${STATIC_FILE_NAME%.a}
SO_LIB="$DIR/$BASE_NAME.so"

WORK_DIR="$DIR/tmp_create_so"

if [ -f "$SO_LIB" ] 
then
	echo "file already exists $SO_LIB"  
	exit 1 
fi

mkdir "$WORK_DIR"

(cd "$WORK_DIR" && ar vx "../$STATIC_FILE_NAME")

gcc "$WORK_DIR"/*.o -o "$SO_LIB" -shared -lutil -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -lutil

rm "./$WORK_DIR" -rf 
