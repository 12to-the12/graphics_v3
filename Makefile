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

animate: run
	ffmpeg -framerate 60 -i ./animation/%04d.png  -c:v libx264  output.mp4
