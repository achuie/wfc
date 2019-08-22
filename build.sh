#!/bin/env sh

cargo build
cp target/debug/libwavefunctioncollapse.so wavefunctioncollapse.so
