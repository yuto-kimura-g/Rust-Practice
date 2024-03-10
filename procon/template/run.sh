#!/bin/sh

set -e # exit on error
set -u # use strict

p=${1}
i=${2:-5}

cargo build --bin "${p}"

for i in `seq 1 ${i}`;
do
    if [ -e "in/${p}${i}.in" ]; then
        echo "[in] ${p}${i}.in"
    else
        break
    fi
    "target/debug/${p}" < "in/${p}${i}.in" 2> /dev/null
    # python3 "src/bin/${p}.py" < "in/${p}${i}.in"
done
