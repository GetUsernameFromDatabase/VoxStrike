#!/bin/bash

cargo build --release

mkdir -p output
if [ -d "output" ]; then
    rm -rf build/*
fi

cp log4rs.yml output/
cp ggml-base.en.bin output/
cp -r profiles output/

if [[ "$OSTYPE" == "msys" ]]; then # git bash
  cp target/release/vox-strike.exe output/
else
  echo "moving vox-strike binary for $OSTYPE not supported"
  exit
fi