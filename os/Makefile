COMPILE_TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(COMPILE_TARGET)/release/os
KERNEL_BIN := $(KERNEL_ELF).bin

run: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000

test: strip
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=$(KERNEL_BIN),addr=0x80200000 \
		-s -S

strip: build
	rust-objcopy --strip-all $(KERNEL_ELF) \
		-O binary $(KERNEL_BIN)

build: env
	cargo build --release

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(COMPILE_TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

clean:
	@cargo clean
