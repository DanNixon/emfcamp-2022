#!/bin/sh

set -ex

cargo run --manifest-path=./config-generator/Cargo.toml -- generate-rubric-content > ansible/files/schedule/dapnet.txt
cargo run --manifest-path=./config-generator/Cargo.toml -- generate-printer-content > ansible/files/schedule/printer.txt
