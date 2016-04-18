all:
	cargo build --release

build:
	cargo build

clean:
	cargo clean

install: all
	cp target/release/linkstreams /usr/bin/linkstreams

rollernet: all
	./script/test_rollernet.sh $(DELTA)

enron: all
	./script/test_enron.sh $(DELTA)

plot:
	./script/plot.sh rollernet
	./script/plot.sh enron
