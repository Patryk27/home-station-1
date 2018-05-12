#!/usr/bin/env bash

source make-config.sh

# Include a few color constants
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Set the target
TARGET='arm-unknown-linux-gnueabi'

function print_help {
    echo -e "${GREEN}Usage:${NC}"
    echo -e "   make.sh action"
    echo
    echo -e "${GREEN}Actions:${NC}"
    echo -e "   -h   print this help"
    echo -e "   -b   build"
    echo -e "   -u   upload to remote"
    echo -e "   -r   run at remote"
    echo -e "   -f   build, upload & run"
    echo
    echo -e "${GREEN}Example:${NC}"
    echo -e "   make.sh -b -u"

    exit 0
}

function build {
    echo -e "${YELLOW}Building...${NC}"

    cargo build --target=$TARGET --release

    if [ $? -ne 0 ]; then
        echo -e "${RED}Building failed, aborting.${NC}"
        exit 1
    fi
}

function upload {
	echo -e "${YELLOW}Uploading...${NC}"

	sshpass -p $RPI_PASS rsync -az target/$TARGET/release/home-station "${RPI_USER}@${RPI_HOST}:${RPI_TARGET_DIR}/home-station"

    if [ $? -ne 0 ]; then
        echo -e "${RED}Uploading failed, aborting.${NC}"
        exit 1
    fi
}

function run {
    echo -e "${YELLOW}Running...${NC}"

    sshpass -p $RPI_PASS ssh -t "${RPI_USER}@${RPI_HOST}" "${RPI_TARGET_DIR}/home-station"
}

if [[ $# -eq 0 ]]; then
    print_help
    exit 0
fi

while getopts "hburf" opt; do
    case "$opt" in
    h)
        print_help
        exit 0
        ;;

    b)
        build
        ;;

    u)
        upload
        ;;

    r)
        run
        ;;

    f)
        build && upload && run
    esac
done