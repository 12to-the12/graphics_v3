default: run
all:run

build:
	cargo build --release
run:
	cargo run --release

test:
	cargo test
