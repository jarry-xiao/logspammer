#!/bin/sh

set -eux

docker build .

docker run --network host \
  -v $(pwd):/work \
  -it \
  $(docker build -q - < Dockerfile) /bin/zsh < /dev/tty