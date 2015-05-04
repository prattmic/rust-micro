CC := arm-none-eabi-gcc
RUSTC ?= rustc

RUSTCFLAGS += -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g

# Only root module need be built, rustc will find the rest
MOD := src/main.o

# All sources root module depends on; rebuild if any change
EXTRA_SRCS := src/lang_items.rs

all: main

main: $(MOD)
	$(CC) -T stm32f4.ld -nostdlib -ffreestanding -o $@ $^

rust/src/libcore/lib.rs:
	git submodule init
	git submodule update

libcore.rlib: rust/src/libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) $<

$(MOD): libcore.rlib $(EXTRA_SRCS)

%.o: %.rs
	$(RUSTC) $(RUSTCFLAGS) -L . --emit obj -o $@ $<

clean:
	rm -f main libcore.rlib
	rm -f *.o src/*.o
