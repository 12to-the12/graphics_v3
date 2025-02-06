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

animate: run
	ffmpeg -framerate 60 -i ./animation/%04d.png  -c:v libx264  output.mp4
