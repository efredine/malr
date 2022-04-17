STEPS = step0_repl step1_read_print

all: $(STEPS)

%: %.rs
	cargo build --release --bin $*
	cp target/release/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS)

step0_repl: clean $(STEP0_DEPS)
step1_read_print: clean $(STEP1_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
