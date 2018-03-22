#!/usr/bin/env python
# coding: utf-8

import setuptools
import setuptools_rust as rust

setuptools.setup(
    setup_requires=[
        "setuptools",
        "setuptools-rust ~=0.9",
    ],

    rust_extensions=rust.find_rust_extensions(
        "bindings",
        binding=rust.Binding.PyO3,
        strip=rust.Strip.Debug,
    )
    # rust_extensions=[
    #     rust.RustExtension(
    #         "bindings.rust.zbox._zbox",
    #         "bindings/rust/zbox/Cargo.toml",
    #         binding=rust.Binding.PyO3,
    #         strip=rust.Strip.Debug,
    #     )
    # ]
)
