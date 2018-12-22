#!/bin/sh
set -ex

cargo build --target wasm32-unknown-unknown
wasm-bindgen --no-modules target/wasm32-unknown-unknown/debug/js_hello_world.wasm --out-dir .

python -m SimpleHTTPServer