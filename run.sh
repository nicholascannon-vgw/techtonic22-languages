#!/usr/bin/env bash
set -e

export DOCKER_SCAN_SUGGEST=false

lang=$1     # js | py | java | go | rust | c

{ # try
    cd $lang
} || { # catch
    echo "Invalid language: $lang"
    exit 1
}

docker build -t techtonic-$lang:lts .

# remove the container if it failed last time
docker container rm -f $lang >/dev/null 2>&1

echo "Starting service..."
time docker run -p 8000:8000 -d --name $lang techtonic-$lang:lts
trap "docker container rm -f $lang" EXIT

docker logs --follow $lang
