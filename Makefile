LOG_LEVEL?=debug
default: run

run:
	cargo run 

# powershell
log:
	@echo "develop mode: log level(${LOG_LEVEL})"
	@echo RUST_BACKTRACE=1;RUST_LOG=${LOG_LEVEL} && cargo run

lint:
	cargo clippy --all-targets --all-features

fmt:
	cargo fmt --all 

test:
	cargo test

clean:
	cd graph & del *.csv *.gif *.jpg *.png

.PHONY: log run lint fmt test clean
