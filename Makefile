default: run
all:run

build: format fix
	cargo build --release
run: format fix
	cargo run --release

test: format fix
	cargo test


format:
	cargo fmt
check: format
	cargo check
fix:
	cargo clippy --allow-dirty --fix -- -Dclippy::pedantic

open: open_pictures run

open_pictures:
	xdg-open ./color_gamut.png
	xdg-open ./rust-output.png

animate: run
	ffmpeg -framerate 60 -i ./animation/%04d.png  -c:v libx264  output.mp4
