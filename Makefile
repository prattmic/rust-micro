CC := arm-none-eabi-gcc
RUSTC ?= rustc

RUSTCFLAGS += -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g

all: main

main: main.o
	$(CC) -T stm32f4.ld -nostdlib -ffreestanding -o main main.o

rust/src/libcore/lib.rs:
	git submodule init
	git submodule update

libcore.rlib: rust/src/libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) $<

main.o: src/main.rs libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) --emit obj src/main.rs -L .

clean:
	rm -f main libcore.rlib
	rm -f *.o
