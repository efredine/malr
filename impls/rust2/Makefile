STEPS = step0_repl step1_read_print step2_eval

all: $(STEPS)

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) types.rs reader.rs printer.rs

step0_repl: $(STEP0_DEPS)
step1_read_print step2_eval: $(STEP1_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
