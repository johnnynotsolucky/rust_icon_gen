[package]
name = "leptos_icon_gen"
version.workspace = true
edition = "2021"
description = "Icon generation macros for Leptos"
documentation = "https://docs.rs/leptos_icon_gen"
exclude = ["generator"]
categories = ["wasm"]
keywords = ["leptos", "icons", "macros"]
license = "MIT"
repository = "https://github.com/johnnynotsolucky/rust_icon_gen"
readme = "README.md"

[lib]
proc-macro = true

[dependencies]
icon_sets = { workspace = true }
once_cell = "1.18.0"
proc-macro2 = "1.0.69"
quote = "1.0"
syn = { version = "2.0", features = ["full"] }

[features]
default = ["full"]
full = [
__features_full
]
__features_feature
