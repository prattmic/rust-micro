LLC ?= llc
RUSTC ?= rustc

all: main

main: src/main.s
	$(CC) -o $@ $<

%.s: %.ll
	$(LLC) -o $@ $<

%.ll: %.rs
	$(RUSTC) -o $@ $< --emit=llvm-ir

clean:
	rm -f main
	rm -f *.ll
	rm -f *.s
