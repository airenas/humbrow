-include Makefile.options
###############################################################################
run:
	RUST_LOG=DEBUG cargo run --bin humbrow -- -p=8001
.PHONY: run
###############################################################################
run/build: build/local
	RUST_LOG=DEBUG target/release/humbrow
.PHONY: run/build
###############################################################################
build/local: 
	cargo build --release
.PHONY: build/local
###############################################################################
test/unit:
	RUST_LOG=DEBUG cargo test --no-fail-fast
.PHONY: test/unit		
test/lint:
	@cargo clippy -V
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: test/lint	
###############################################################################
clean:
	rm -r -f target
.PHONY: clean
