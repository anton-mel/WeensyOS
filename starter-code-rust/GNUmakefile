TARGET := x86_64-weensyos
KERNEL_LINKER_SCRIPT := linker.ld

RUST_OS := target/$(TARGET)/debug/libkernel.a
KERNEL := kernel.bin
ASSEMBLY_OBJECT_FILES := boot.o

all: $(KERNEL)

# Step 1: Build Rust OS kernel
$(RUST_OS): src/main.rs
	@echo "< [Step #1] Building Rust OS"
	@cargo build --target $(TARGET).json
	@echo "> Rust OS built successfully."

# Step 2: Assemble bootloader
$(ASSEMBLY_OBJECT_FILES): boot.s
	@echo "< [Step #2] Assembling bootloader"
	@as --32 -o $(ASSEMBLY_OBJECT_FILES) boot.s
	@echo "> Bootloader assembled successfully."

# Step 3: Linking kernel
$(KERNEL): $(RUST_OS) $(ASSEMBLY_OBJECT_FILES) $(KERNEL_LINKER_SCRIPT)
	@echo "< [Step #3] Linking Kernel: $(KERNEL_LINKER_SCRIPT)"
	@ld --script=$(KERNEL_LINKER_SCRIPT) -m elf_x86_64 -o $(KERNEL) $(ASSEMBLY_OBJECT_FILES) $(RUST_OS)
	@echo "> Kernel linked successfully."

clean:
	@echo "< Cleaning up"
	@cargo clean
	@rm -f $(ASSEMBLY_OBJECT_FILES) $(KERNEL)
	@echo "> Cleaned."

.PHONY: all clean


# arch ?= x86_64
# image = weensyos.img
# kernel := .build/kernel-$(arch).bin
# bootloader := src/kernel/bootstart.S
# iso := .build/$(image)
# all: $(iso)

# rust_os := target/$(arch)-weensyos/debug/libweensyos.a
# kernel_linker_script := src/link/kernel.ld
# assembly_source_files := $(wildcard src/build/*.asm)
# assembly_object_files := $(patsubst src/build/%.asm, \
# 	build/build/%.o, $(assembly_source_files))

# # compile assembly files
# build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
# 	@mkdir -p $(shell dirname $@)
# 	@nasm -felf64 $< -o $@


# iso: $(iso)
# $(iso): $(kernel)
# 	@echo "< [Step #3] Creating ISO image..."
# 	@cp $(kernel) .build/isofiles/boot/
# 	@cp $(grub_cfg) .build/isofiles/boot/grub
# 	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
# 	@echo "> ISO image ($(iso)) created successfully"
# # check for bootable info below
# 	@bash -c 'if isoinfo -d -i $(iso) | grep -q "bootable"; \
# 	then echo "isoinfo: [bootable]"; else echo "isoinfo: [not bootable]"; fi'
# 	@rm -r build/isofiles

# # CHANGE THIS (WORKING ON THIS)
# $(kernel): build $(rust_os) $(assembly_object_files) $(kernel_linker_script)
# 	@echo "< [Step #2] Linking Kernel: $(kernel_linker_script)"
# 	@ld --script=$(kernel_linker_script) -m elf_x86_64 -o $(kernel) $(assembly_object_files) $(rust_os)
# 	@echo "> Kernel linked successfully."



# # Step 2: Assemble bootloader
# $(ASSEMBLY_OBJECT_FILES): $(bootloader)
# 	@echo "< [Step #2] Assembling bootloader"
# 	@as --32 -o $(ASSEMBLY_OBJECT_FILES) $(bootloader)
# 	@echo "> Bootloader assembled successfully."

# # Step 1: Build Rust OS kernel
# build:
# 	@echo "< [Step #1] building kernel and mkbootdisk..."
# 	@mkdir -p .build/isofiles/boot/grub
# 	@cargo +nightly build --target x86_64-weensyos.json
# 	@echo "> Kernel and mkbootdisk built successfully."


# # QEMU configs
# run: $(iso)
# 	@qemu-system-x86_64 -s -hda $(iso) \
#         -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
#         -usb \
#         -usbdevice tablet \
#         -display curses \
#         -full-screen \
#         -serial file:log.txt


# .PHONY: all clean run iso cargo

# clean:
# 	@cargo clean
# 	@rm -rf .build Cargo.lock
