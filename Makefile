default: run
all:run

build: format
	cargo build --release
run: format
	cargo run --release

test: format
	cargo test


format:
	cargo fmt
check: format
	cargo check

open: open_pictures run

open_pictures:
	xdg-open ./color_gamut.png
	xdg-open ./rust-output.png

animate: run
	ffmpeg -framerate 60 -i ./animation/%04d.png  -c:v libx264  output.mp4
