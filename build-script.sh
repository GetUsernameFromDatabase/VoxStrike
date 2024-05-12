#!/bin/bash

OUT_DIR="output"

while [[ $# -gt 0 ]]; do
  case $1 in
    -d|--destination)
      DESTINATION="$2"
      shift # past argument
      shift # past value
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done

cargo build --release

mkdir -p $OUT_DIR
if [ -d "$OUT_DIR" ]; then
    rm -rf $OUT_DIR/*
fi

cp log4rs.yml $OUT_DIR/
cp ggml-base.en.bin $OUT_DIR/
cp -r profiles $OUT_DIR/

if [[ "$OSTYPE" == "msys" ]]; then # msys -- git bash
  EXE=vox-strike.exe
  cp target/release/$EXE $OUT_DIR/
  signtool sign -a -fd SHA256 -tr http://timestamp.digicert.com -td SHA256 $OUT_DIR/$EXE
else
  echo "moving vox-strike binary for $OSTYPE not supported"
  exit
fi

if [ -n "$DESTINATION" ]; then
  echo "Moving $OUT_DIR to $DESTINATION"
  rm -rf "$DESTINATION/$OUT_DIR"
  mv  "$OUT_DIR" "$DESTINATION"
fi

