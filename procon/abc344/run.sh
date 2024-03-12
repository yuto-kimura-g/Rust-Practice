#!/bin/sh

# Usage: sh run.sh <problem> <testcase> <feature>
# Example: sh run.sh a 5 local

set -e # exit on error
set -u # use strict

problem=${1}
n=${2:-5}
feature=${3:-"judge"}

cargo build --bin "${problem}" --features "${feature}"

for i in `seq 1 ${n}`;
do
    if [ -e "in/${problem}${i}.in" ]; then
        echo "[in] ${problem}${i}.in"
    else
        break
    fi
    "target/debug/${problem}" < "in/${problem}${i}.in"
    # python3 "src/bin/${problem}.py" < "in/${problem}${i}.in"
done
