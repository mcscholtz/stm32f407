# `stm32f407`

> Peripheral access API for STM32F407 microcontrollers

Generated using [svd2rust] v0.11.4 from [STM32F407.svd]

[STM32F407.svd]: http://www.st.com/resource/en/svd/stm32f4_svd.zip
[svd2rust]: https://github.com/japaric/svd2rust

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Build
```
export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH
dos2unix STM32F407.svd
svd2rust -i STM32F407.svd | rustfmt | tee src/lib.rs
xargo build --target thumbv7em-none-eabihf
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
