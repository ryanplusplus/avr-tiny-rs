ELF:=target/avr-atmega328p/release/avr-tiny-rs.elf
DWDEBUG_DEVICE?=ttyUSB0

.PHONY: all
all: $(ELF)

.PHONY: $(ELF)
$(ELF):
	@export AVR_CPU_FREQUENCY_HZ=16000000 && cargo +nightly-2021-01-07 build --release

.PHONY: upload
upload: $(ELF)
	@(command -v setserial > /dev/null && setserial /dev/$(DWDEBUG_DEVICE) low_latency) || true
	@tools/bin/dwdebug device $(DWDEBUG_DEVICE),l $<,qr

.PHONY: clean
clean:
	@rm -rf target

.PHONY: build/debug-server-dwdebug
build/debug-server-dwdebug:
	@mkdir -p build
	@(command -v setserial > /dev/null && setserial /dev/$(DWDEBUG_DEVICE) low_latency) || true
	@echo "#!/bin/bash" > $@
	@echo "tools/bin/dwdebug verbose,gdbserver,device $(DWDEBUG_DEVICE)" >> $@
	@chmod +x $@

.PHONY: debug-deps
debug-deps: build/debug-server-dwdebug $(ELF)
