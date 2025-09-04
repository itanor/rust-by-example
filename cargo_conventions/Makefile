
--build-bin:
	@cargo build --bin $(BINARY)

:phony: build-main-bin
build-main-bin: BINARY=cargo_conventions
build-main-bin: --build-bin

:phony: build-secondary-bin
build-secondary-bin: BINARY=other_bin
build-secondary-bin: build-bin

--run-bin:
	@cargo run --bin $(BINARY)

:phony: run-main-bin
run-main-bin: BINARY=cargo_conventions
run-main-bin: --run-bin

:phony: run-secondary-bin
run-secondary-bin: BINARY=other_bin
run-secondary-bin: --run-bin