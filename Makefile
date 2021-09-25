.PHONY: pb
pb:
	@echo 'make proto dir ...'
	@mkdir -p src/pb
	@cargo run build.rs

.PHONY: build
build:
	@echo 'build ...'
	@cargo build --release