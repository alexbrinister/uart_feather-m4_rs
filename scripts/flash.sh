#! /usr/bin/env bash

function usage() {
  echo "Usage: $(basename $0) [-h] [-b <filename>] [-d <devicefile>]"
  echo '     -h    shows the usage prompt'
  echo '     -b    the binary file to flash to the Feather M4'
  echo '     -d    the device file (in /dev) to flash the binary to'

  exit 1
}

function check_for_bossa() {
  if [[ ! -e `which bossac` ]]; then
    echo "You do not have bossac installed!"
    exit 1
  fi 
}

function run_bossa() {
  if ! check_for_bossa; then
    exit 1
  fi

  bossac \
    --port "$2" \
    --offset 0x4000 \
    --erase \
    --write \
    --verify \
    --reset \
    "$1"
}

if [[ ${#} -eq 0 ]]; then
  usage
fi

optstring="hb:d:"

while getopts ${optstring} arg; do
  case "${arg}" in
    h)
      usage
      ;;
    b)
      BINPATH="${OPTARG}"
      ;;
    d)
      DEV="${OPTARG}"
      ;;
    ?)
      echo "Invalid option: -${OPTARG}."
      echo
      usage
      ;;
  esac
done

# Makes sure files exist before passing them to bossac
if [[ ! -e "${BINPATH}" ]]; then
  echo "File ${BINPATH} does not exist"
  exit 1
fi

if [[ ! -e "${DEV}" ]]; then
  echo "Device ${DEV} does not exist"
  exit 1
fi

# Run bossac
run_bossa "${BINPATH}" "${DEV}"
