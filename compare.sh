#!/bin/bash
cat log.txt >> log-archive.txt
echo > log.txt
# cargo clean
echo "__________<>__________" >> log.txt
cargo run --profile dev >> log.txt

# echo "__________<>__________" >> log.txt
# cargo run -02 >> log.txt

echo "__________<>__________" >> log.txt
cargo run --profile release >> log.txt


echo "__________<>__________" >> log.txt
./python/benchmark.py >> log.txt