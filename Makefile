prog :=tood

release :=--release
target :=release

build:
	cargo build $(release)

install:
	sudo cp target/$(target)/$(prog) /sbin/$(prog)

all: build install
 
help:
	@echo "usage: make $(prog) [debug=1]"
