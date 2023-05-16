clean:
	rm -rv target/release
	rm -rv target/debug
	rm meow_translator

build:
	cargo build --release
	ln -s target/release/meow_translator ./meow_translator

build_devel:
	cargo build --debug
	ln -s target/debug_meow_translator ./meow_translator