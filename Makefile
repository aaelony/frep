

format:
	cargo fmt

build:
	cargo build

run:
	cargo run


clippy:
	cargo clippy


release:
	cargo build --release

deploy: release
	cp target/release/frep ~/bin

docs:
	cargo doc

test:
	cargo test
