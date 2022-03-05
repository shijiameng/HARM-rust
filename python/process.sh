#!/bin/bash

if [ x$1 == x ]; then
	echo "usage: $0 <project_name>"
	exit 1
fi

BASE_PATH=../samples
SR_PATH=${BASE_PATH}/secure_lib
CMSE_LIB=${SR_PATH}/secure_runtime_CMSE_lib.o

OUTPUT_PATH=${SR_PATH}

python main.py -c ${CMSE_LIB} -p ${OUTPUT_PATH} -e 0x20000 \
	-i ${BASE_PATH}/$1.axf -o ${BASE_PATH}/$1_s.bin

if [ $? != 0 ]; then
	echo "Error!"
	exit 1
fi
