pub static WS_CARGO: &str = r#"
[workspace]
members = [
    "common",
]

[profile.euphony]
inherits = "dev"
debug = false
lto = "off"

[profile.euphony.package."*"]
opt-level = 3

[profile.euphony.build-override]
opt-level = 3
"#;

pub static WS_GITIGNORE: &str = r#"
target
**/*.rs.bk
Cargo.lock
"#;

pub static WS_RUSTFMT: &str = r#"
edition = "2021"
format_macro_matchers = true
imports_granularity = "Crate"
use_field_init_shorthand = true
"#;

pub static COMMON_CARGO: &str = r#"
[package]
name = "common"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
euphony = "0.1"
"#;

pub static COMMON_LIB: &str = r#"
use euphony::prelude::*;

// put common code here
"#;

pub static COMP_CARGO: &str = r#"
[package]
name = "NAME"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
common = { path = "COMMON_PATH" }
euphony = "0.1"
"#;

pub static COMP_MAIN: &str = include_str!("./templates/main.rs");
