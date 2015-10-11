all:
	cargo build --release

build:
	cargo build

clean:
	cargo clean

install: all
	cp target/release/linkstreams /usr/bin/linkstream

rollernet: all
	./script/test_rollernet.sh

enron: all
	./script/test_enron.sh
