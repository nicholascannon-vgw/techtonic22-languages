#!/usr/bin/env bash

# silence some docker logs
export DOCKER_SCAN_SUGGEST=false

lang=$1     # js | go | c
cd $lang

docker build -t techtonic-$lang:lts .
# remove the container if it failed last time
docker container rm -f $lang >/dev/null 2>&1
docker run -p 8000:8000 --name $lang techtonic-$lang:lts
