default: run
all:run

build:
	cargo build --release
run:
	cargo run --release

test:
	cargo test

open: open_pictures run

open_pictures:
	xdg-open ./rust-output.png
	xdg-open ./color_gamut.png