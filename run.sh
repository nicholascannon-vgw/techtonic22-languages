#!/usr/bin/env bash
set -euo pipefail

lang=$1     # js | py | java | go | rust | c

{ # try
    cd $lang && \
        docker build -t techtonic-$lang:lts . && \
        docker run -p 8000:8000 techtonic-$lang:lts
} || { # catch
    echo "Invalid language: $lang"
}
