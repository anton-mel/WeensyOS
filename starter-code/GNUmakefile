IMAGE = weensyos.img
all: clean $(IMAGE)
-include build/rules.mk

# '$(TEST_FILE)' controls which of the default test cases will be replaced
# when running test-%
# By default this is p-test.c
# A backup exists in tests/bck
TEST_FILE ?= p-test.c
TEST_DIR = ./tests/

# DIRECTORIES for each part of the system
C_KERN_DIR  = ./kernel/c
R_KERN_DIR = ./kernel/rust
PROC_DIR = ./uspace
SHARED_DIR = ./shared

ROOT_DIR := $(shell realpath $(CURDIR))
RUST_ARCHIVE_DIR = $(R_KERN_DIR)/target/release

# '$(V)' controls whether the lab makefiles print verbose commands (the
# actual shell commands run by Make), as well as the "overview" commands
# (such as '+ cc lib/readline.c').
#
# For overview commands only, run 'make all'.
# For overview and verbose commands, run 'make V=1 all'.
V = 0

# $(INTERACTIVE) reports whether make is running in interactive shell or to file
# it will be used to enable/disable color coding
INTERACTIVE:=$(shell [ -t 0 ] && echo 1)

# COLOR OUTPUT CODES
ifdef INTERACTIVE
	ccred:=$(shell echo -e "\033[0;31m")
	ccyellow:=$(shell echo -e "\033[0;33m")
	ccgreen:=$(shell echo -e "\033[1;32m")
	ccend:=$(shell echo -e "\033[0m")
else
	ccred:=""
	ccyellow:=""
	ccgreen:=""
	ccend:=""
endif

# If you are working locally, set to 0 to disable
# If you are using node.zoo.cs.edu, set 1 to enable
USE_HOST_LOCK ?= 0

# locked img error messages
# messages to display if another instance of QEMU is running
errbef:="$(ccred)Error, could not obtain lock. Are you running a session in - $(ccgreen)"
erraft:="$(ccred)If so try running $(ccgreen)make kill-lock$(ccred),\
		otherwise $(ccyellow)ssh in manually and run make kill$(ccend)" 

# 'HOST_FILE' will contain the hostname of the machine running $(QEMU)
# This allows us to kill process in remote machine if needed
# 'HOST_LOCK' is the name of the lockfile
HOST_FILE='.hosts.txt'
HOST_LOCK='.hosts.lock'

ifeq ($(V),1)
compile = $(CC) $(CPPFLAGS) $(CFLAGS) $(DEPCFLAGS) $(1)
link = $(LD) $(LDFLAGS) $(1)
run = $(1) $(3)
cpy = $(shell cat $(1) > ./$(PROC_DIR)/$(TEST_FILE))
else
compile = @/bin/echo " " $(2) $< && $(CC) $(CPPFLAGS) $(CFLAGS) $(DEPCFLAGS) $(1)
link = @/bin/echo " " $(2) $(patsubst %.full,%,$@) && $(LD) $(LDFLAGS) $(1)
run = @$(if $(2),/bin/echo " " $(2) $(3) &&,) $(1) $(3)
cpy = @/bin/echo " " $(shell cat $(1) > ./$(PROC_DIR)/$(TEST_FILE))
endif


# Object sets

BOOT_OBJS = $(OBJDIR)/bootstart.o $(OBJDIR)/boot.o

KERNEL_OBJS = $(OBJDIR)/k-exception.o $(OBJDIR)/kernel.o \
	$(OBJDIR)/k-hardware.o $(OBJDIR)/vm.o $(OBJDIR)/k-loader.o
RUST_KERNEL_OBJS = $(RUST_ARCHIVE_DIR)/weensyos.o
KERNEL_LINKER_FILES = link/kernel.ld link/shared.ld

PROCESS_BINARIES = $(OBJDIR)/p-allocator $(OBJDIR)/p-allocator2 \
	$(OBJDIR)/p-allocator3 $(OBJDIR)/p-allocator4 \
	$(OBJDIR)/p-fork $(OBJDIR)/p-forkexit $(OBJDIR)/p-test
LIB_OBJS = $(OBJDIR)/lib.o
PROCESS_LIB_OBJS= $(OBJDIR)/process.o
ALLOCATOR_OBJS = $(OBJDIR)/p-allocator.o $(LIB_OBJS) $(PROCESS_LIB_OBJS)
PROCESS_OBJS = $(OBJDIR)/p-allocator.o $(OBJDIR)/p-fork.o \
	$(OBJDIR)/p-forkexit.o $(OBJDIR)/p-test.o 
PROCESS_LINKER_FILES = link/process.ld link/shared.ld


# Generic rules for making object files

rust:
	@echo "  BUILDING Rust objects..."
	@cd $(R_KERN_DIR) && cargo build --release

$(RUST_KERNEL_OBJS): | rust
	@echo "  UNPACKING and processing .o files..."
	@cd $(RUST_ARCHIVE_DIR) && \
	for lib in lib*.a; do \
		ar x "$$lib"; \
		libname=$$(basename "$$lib" .a); \
		libname=$${libname#lib}; \
		for obj in *.o; do \
			if echo "$$obj" | grep -q "^$$libname.*\.o$$"; then \
				mv "$$obj" "$(ROOT_DIR)/$(RUST_ARCHIVE_DIR)/$$libname.o"; \
			else \
				rm -f "$$obj"; \
			fi; \
		done; \
	done

$(LIB_OBJS): $(OBJDIR)/%.o: $(SHARED_DIR)/%.c $(BUILDSTAMPS)
	$(call compile,-I $(SHARED_DIR) -c $< -o $@,COMPILE)

$(PROCESS_LIB_OBJS): $(OBJDIR)/%.o: $(PROC_DIR)/%.c $(BUILDSTAMPS)
	$(call compile,-O1 -DWEENSYOS_PROCESS -I ./$(SHARED_DIR) -c $< -o $@,COMPILE)

$(PROCESS_OBJS): $(OBJDIR)/%.o: $(PROC_DIR)/%.c $(BUILDSTAMPS)
	$(call compile,-O1 -DWEENSYOS_PROCESS -I ./$(SHARED_DIR) -c $< -o $@,COMPILE)

$(OBJDIR)/%.o: $(C_KERN_DIR)/%.c $(BUILDSTAMPS)
	$(call compile,-DWEENSYOS_KERNEL -I ./$(SHARED_DIR) -c $< -o $@,COMPILE)

$(OBJDIR)/boot.o: $(OBJDIR)/%.o: $(C_KERN_DIR)/boot.c $(BUILDSTAMPS)
	$(call compile,-Os -fomit-frame-pointer -I ./$(SHARED_DIR) -c $< -o $@,COMPILE)

$(OBJDIR)/%.o: $(C_KERN_DIR)/%.S $(BUILDSTAMPS)
	$(call compile, -I ./$(SHARED_DIR) -c $< -o $@,ASSEMBLE)


# Specific rules for WeensyOS

$(OBJDIR)/kernel.full: $(RUST_KERNEL_OBJS) $(KERNEL_OBJS) $(LIB_OBJS) $(PROCESS_BINARIES) $(KERNEL_LINKER_FILES)
	$(call link,-T $(KERNEL_LINKER_FILES) -o $@ $(RUST_KERNEL_OBJS) $(KERNEL_OBJS) $(LIB_OBJS) -b binary $(PROCESS_BINARIES),LINK)

$(OBJDIR)/p-%.full: $(OBJDIR)/p-%.o $(LIB_OBJS) $(PROCESS_LIB_OBJS) $(PROCESS_LINKER_FILES)
	$(call link,-T $(PROCESS_LINKER_FILES) -o $@ $< $(PROCESS_LIB_OBJS) $(LIB_OBJS),LINK)

$(OBJDIR)/p-allocator%.full: $(ALLOCATOR_OBJS) link/p-allocator%.ld link/shared.ld
	$(call link,-T link/p-allocator$*.ld link/shared.ld -o $@ $(ALLOCATOR_OBJS),LINK)

$(OBJDIR)/%: $(OBJDIR)/%.full
	$(call run,$(OBJDUMP) -S $< >$@.asm)
	$(call run,$(NM) -n $< >$@.sym)
	$(call run,$(OBJCOPY) -j .text -j .rodata -j .data -j .bss $<,STRIP,$@)

$(OBJDIR)/bootsector: $(BOOT_OBJS) link/boot.ld link/shared.ld
	$(call link,-T link/boot.ld link/shared.ld -o $@.full $(BOOT_OBJS),LINK)
	$(call run,$(OBJDUMP) -S $@.full >$@.asm)
	$(call run,$(NM) -n $@.full >$@.sym)
	$(call run,$(OBJCOPY) -S -O binary -j .text $@.full $@)

$(OBJDIR)/mkbootdisk: build/mkbootdisk.c $(BUILDSTAMPS)
	$(call run,$(HOSTCC) -I./$(SHARED_DIR) -o $(OBJDIR)/mkbootdisk,HOSTCOMPILE,build/mkbootdisk.c)

weensyos.img: $(OBJDIR)/mkbootdisk $(OBJDIR)/bootsector $(OBJDIR)/kernel
	$(call run,$(OBJDIR)/mkbootdisk $(OBJDIR)/bootsector $(OBJDIR)/kernel > $@,CREATE $@)


run-%: run-qemu-%
	@:

run-qemu-%: run-$(QEMUDISPLAY)-%
	@:

run-graphic-%: %.img check-qemu
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    lockfile -r 0 ./$(HOST_LOCK) || (echo ${errbef} && cat $(HOST_FILE); echo ${erraft}; exit 1); \
	    hostname > $(HOST_FILE); \
	fi
	$(call run,$(QEMU_PRELOAD) $(QEMU) $(QEMUOPT) $(QEMUIMG),QEMU $<)
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    rm -f ./$(HOST_FILE); \
	    rm -f ./$(HOST_LOCK); \
	fi

run-console-%: %.img check-qemu
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    lockfile -r 0 ./$(HOST_LOCK) || (echo ${errbef} && cat $(HOST_FILE); echo ${erraft}; exit 1); \
	    hostname > $(HOST_FILE); \
	fi
	$(call run,$(QEMU_PRELOAD) $(QEMU) $(QEMUOPT) -display curses $(QEMUIMG),QEMU $<)
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    rm -f ./$(HOST_FILE); \
	    rm -f ./$(HOST_LOCK); \
	fi

run-monitor-%: %.img check-qemu
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    lockfile -r 0 ./$(HOST_LOCK) || (echo ${errbef} && cat $(HOST_FILE); echo ${erraft}; exit 1); \
	    hostname > $(HOST_FILE); \
	fi
	$(call run,$(QEMU_PRELOAD) $(QEMU) $(QEMUOPT) -monitor stdio $(QEMUIMG),QEMU $<)
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    rm -f ./$(HOST_FILE); \
	    rm -f ./$(HOST_LOCK); \
	fi

run-gdb-%: run-gdb-$(QEMUDISPLAY)-%
	@:

run-gdb-graphic-%: %.img check-qemu
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    lockfile -r 0 ./$(HOST_LOCK) || (echo ${errbef} && cat $(HOST_FILE); echo ${erraft}; exit 1); \
	    hostname > $(HOST_FILE); \
	fi
	$(call run,$(QEMU_PRELOAD) $(QEMU) $(QEMUOPT) -gdb tcp::1234 $(QEMUIMG) &,QEMU $<)
	$(call run,sleep 0.5; gdb -x .gdbinit,GDB)
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    rm -f ./$(HOST_FILE); \
	    rm -f ./$(HOST_LOCK); \
	fi

run-gdb-console-%: %.img check-qemu
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    lockfile -r 0 ./$(HOST_LOCK) || (echo ${errbef} && cat $(HOST_FILE); echo ${erraft}; exit 1); \
	    hostname > $(HOST_FILE); \
	fi
	$(call run,$(QEMU_PRELOAD) $(QEMU) $(QEMUOPT) -display curses -gdb tcp::1234 $(QEMUIMG),QEMU $<)
	@if [ "$(USE_HOST_LOCK)" -eq 1 ]; then \
	    rm -f ./$(HOST_FILE); \
	    rm -f ./$(HOST_LOCK); \
	fi

run: run-qemu-$(basename $(IMAGE))
run-qemu: run-qemu-$(basename $(IMAGE))
run-graphic: run-graphic-$(basename $(IMAGE))
run-console: run-console-$(basename $(IMAGE))
run-monitor: run-monitor-$(basename $(IMAGE))
run-quit: run-quit-$(basename $(IMAGE))
run-gdb: run-gdb-$(basename $(IMAGE))
run-gdb-graphic: run-gdb-graphic-$(basename $(IMAGE))
run-gdb-console: run-gdb-console-$(basename $(IMAGE))
run-graphic-gdb: run-gdb-graphic-$(basename $(IMAGE))
run-console-gdb: run-gdb-console-$(basename $(IMAGE))

.PHONY:
test-%: $(TEST_DIR)/p-%.c
	$(call cpy, $<)

.PHONY:
restore:
	@$(shell cp $(TEST_DIR)/bck/p-*.c ./$(PROC_DIR)/)
	@echo "Restored uspace process code to default!"

# Kill all my qemus
kill:
	-killall -u $$(whoami) $(QEMU)
	@sleep 0.2; if ps -U $$(whoami) | grep $(QEMU) >/dev/null; then killall -9 -u $$(whoami) $(QEMU); fi

.PHONY:
kill-lock:
	HOST=$$(cat $(HOST_FILE));HOST_CMD=hostname; if [ "$$HOST" == "$$HOST_CMD" ]; then make kill; else echo $$HOST;CWD=$$(pwd);ssh $$HOST "cd $$CWD;make kill;" && make clean-secret;fi

.PHONY:
clean-secret:
	rm -f $(HOST_LOCK) $(HOST_FILE)
