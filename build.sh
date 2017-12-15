#!/bin/bash
#Prepare svd file
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
dos2unix STM32F407.svd
svd2rust -i STM32F407.svd | rustfmt | tee src/lib.rs
#build
xargo build --target thumbv7em-none-eabihf