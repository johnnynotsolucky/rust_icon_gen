[package]
name = "icon_sets"
version.workspace = true
edition = "2021"
description = "Generated icon data"
documentation = "https://docs.rs/icon_sets"
exclude = ["generator"]
categories = ["wasm"]
keywords = ["icons"]
license = "MIT"
repository = "https://github.com/johnnynotsolucky/rust_icon_gen"
readme = "README.md"

[dependencies]
once_cell = "1.18.0"

[features]
default = ["full"]
full = [
__features_full
]
__features_feature
