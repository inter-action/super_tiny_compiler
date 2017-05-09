# PATH  := node_modules/.bin:$(PATH)
SHELL := /bin/bash

.PHONY: test

test:
	cargo test

test_with_trace:
	RUST_BACKTRACE=1 cargo test

test_with_stdout:
	cargo test -- --nocapture


compile_watch:
	watchexec -e rs -f src cargo build

build:
	cargo build

run:
	cargo run

debug_run:
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

fmt: 
	cargo fmt

clean_src:
	find src -type file -name "*.bk" -exec rm -rf {} \;
