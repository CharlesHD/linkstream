all:
	cargo build --release

build:
	cargo build

clean:
	cargo clean

install: all
	cp target/release/linkstreams /usr/bin/linkstream
