arch ?= x86_64
image = weensyos.img
kernel := build/kernel-$(arch).bin
iso := build/$(image)
all: $(iso)

rust_os := target/$(arch)-weensyos/debug/libweensyos.a
kernel_linker_script := src/link/kernel.ld
assembly_source_files := $(wildcard src/build/*.asm)
assembly_object_files := $(patsubst src/build/%.asm, \
	build/build/%.o, $(assembly_source_files))

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@


iso: $(iso)
$(iso): $(kernel)
	@echo "< [Step #3] Creating ISO image..."
	@cp $(kernel) .build/isofiles/boot/
# WORKING HERE ABOVE
	@cp $(grub_cfg) .build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@echo "> ISO image ($(iso)) created successfully"
# check for bootable info below
	@bash -c 'if isoinfo -d -i $(iso) | grep -q "bootable"; \
	then echo "isoinfo: [bootable]"; else echo "isoinfo: [not bootable]"; fi'
	@rm -r build/isofiles

# CHANGE THIS (WORKING ON THIS)
$(kernel): build $(rust_os) $(assembly_object_files) $(kernel_linker_script)
	@echo "< [Step #2] Linking Kernel: $(kernel_linker_script)"
	@ld --script=$(kernel_linker_script) -m elf_x86_64 -o $(kernel) $(assembly_object_files) $(rust_os)
	@echo "> Kernel linked successfully."

build:
	@echo "< [Step #1] building kernel and mkbootdisk..."
	@mkdir -p .build/isofiles/boot/grub
# WORKING HERE BELOW
	@cargo +nightly build --target x86_64-weensyos.json
	@echo "> Kernel and mkbootdisk built successfully."


# QEMU configs
run: $(iso)
	@qemu-system-x86_64 -s -hda $(iso) \
        -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
        -usb \
        -usbdevice tablet \
        -display curses \
        -full-screen \
        -serial file:log.txt


.PHONY: all clean run iso cargo

clean:
	@cargo clean
	@rm -rf build Cargo.lock
