default: run
all:run

build:
	cargo build --release
run:
	cargo run --release

test:
	cargo test


check:
	cargo check

open: open_pictures run

open_pictures:
	xdg-open ./color_gamut.png
	xdg-open ./rust-output.png
