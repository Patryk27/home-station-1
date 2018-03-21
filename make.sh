#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

RPI_HOST='pi'
RPI_USER='pi'
RPI_PASS='pi'

TARGET='arm-unknown-linux-gnueabi'

export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_DIR=../libs/arm/openssl/_out

clear

# -- clean -- #
echo -e "${YELLOW}Cleaning...${NC}"

rm -f target/$TARGET/release/home-station

# -- build -- #
echo -e "${YELLOW}Building...${NC}"

cargo build --target=$TARGET --release

if [ ! -f target/$TARGET/release/home-station ]; then
	echo -e "${RED}Building failed, aborting.${NC}"
	exit 1
fi

# -- deploy -- #
if [ "$1" == "deploy" ]; then
    # -- upload -- ##
	echo -e "${YELLOW}Uploading...${NC}"

	sshpass -p $RPI_PASS rsync -az target/$TARGET/release/home-station "${RPI_USER}@${RPI_HOST}":/home/pi/home-station

    if [ "$?" -ne "0" ]; then
        echo -e "${RED}Uploading failed, aborting.${NC}"
        exit 2
    fi

    # -- run -- #
    echo -e "${YELLOW}Running...${NC}"

    sshpass -p $RPI_PASS ssh -t "${RPI_USER}@${RPI_HOST}" /home/pi/home-station
fi
