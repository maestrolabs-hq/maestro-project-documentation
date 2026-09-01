# Optional convenience tasks; every command remains runnable without just.

path_sep := if os_family() == "windows" { ";" } else { ":" }
export PATH := home_directory() / ".cargo" / "bin" + path_sep + home_directory() / ".local" / "bin" + path_sep + env('PATH')

install:
    rustup toolchain install --profile minimal 1.98.0
    rustup component add clippy rustfmt llvm-tools
    cargo binstall -y prek cargo-deny cargo-machete cargo-llvm-cov similarity-rs
    cargo install mdbook --version 0.5.4 --locked

setup:
    prek install --install-hooks

check:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets
    cargo machete
    cargo deny check
    cargo run -p sitecheck -- .
    mdbook test
    mdbook build

build:
    cargo run -p sitecheck -- .
    mdbook build

serve:
    mdbook serve --open

fmt:
    cargo fmt --all

doctor:
    @echo "just    $(command -v just)"
    @echo "cargo   $(command -v cargo)"
    @echo "mdbook  $(mdbook --version)"
    @echo "prek    $(command -v prek)"
    @echo "rustc   $(rustc --version)"
