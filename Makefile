
build:
	cargo build --bins --release

install:
	cp target/release/rust_format $$HOME/iceyee/bin
