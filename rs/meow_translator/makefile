all: add_localbin_to_path build
	cp target/release/meow_translator $(HOME)/.local/bin

install:
	cp target/release/meow_translator $(HOME)/.local/bin

build:
	cargo build --release

	if [ -f "./meow_translator" ]; then \
		rm ./meow_translator; \
	fi

	ln -s target/release/meow_translator ./meow_translator

add_localbin_to_path:
	if [ -d "$(HOME)/.local/bin/" ]; then \
		mkdir $(HOME)/.local/bin; \
	fi
	
	$(shell export PATH="$PATH:$HOME/.local/bin")
	@echo "$(HOME)/.local/bin was added to the PATH in this session."
	@echo "to make this change global, please do so in you shell's rc."

clean:
	rm -rvf target/release;

uninstall:
	rm $(HOME)/.local/bin/meow_translator;

build_devel:
	cargo build --devel

	if [ -f "./meow_translator_devel" ]; then \
		rm ./meow_translator_devel; \
	fi

	ln -s target/release/meow_translator ./meow_translator_devel