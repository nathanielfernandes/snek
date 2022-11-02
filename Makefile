NAME=snek
BUILD=target/wasm32-unknown-unknown/release/
DEST=docs/

run:
	@cargo run

.PHONY: build-wasm
build-wasm:
	@cargo build --target wasm32-unknown-unknown --release
	@mv $(BUILD)$(NAME).wasm $(DEST)