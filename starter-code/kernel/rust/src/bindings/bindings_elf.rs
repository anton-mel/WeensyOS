#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ElfProgram {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_va: u64,
    pub p_pa: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ElfHeader {
    pub e_magic: u32,
    pub e_elf: [u8; 12usize],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

pub const ELF_MAGIC: u32 = 1179403647;

// Values for elf_program::p_type
pub const ELF_PTYPE_LOAD: u32 = 1;

// Flag bits for elf_program::p_flags
pub const ELF_PFLAG_EXEC: u32 = 1;
pub const ELF_PFLAG_WRITE: u32 = 2;
pub const ELF_PFLAG_READ: u32 = 4;

// Values for elf_section::s_type
pub const ELF_STYPE_NULL: u32 = 0;
pub const ELF_STYPE_PROGBITS: u32 = 1;
pub const ELF_STYPE_SYMTAB: u32 = 2;
pub const ELF_STYPE_STRTAB: u32 = 3;

// Values for elf_section::s_name
pub const ELF_SNAME_UNDEF: u32 = 0;
