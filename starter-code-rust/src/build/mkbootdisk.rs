#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#![no_main]
#![no_std]

/* This program makes a boot image.
 * It takes at least one argument, the boot sector file.
 * Any succeeding arguments are written verbatim to the output file.
 *
 * Before jumping to the boot sector, the BIOS checks that the last
 * two bytes in the sector equal 0x55 and 0xAA.
 * This code makes sure the code intended for the boot sector is at most
 * 512 - 2 = 510 bytes long, then appends the 0x55-0xAA signature.
 */

use core::ffi::{c_char, c_uchar, c_schar, c_int, c_uint, c_void, c_long, c_ulong, c_ushort};
use crate::shared;

extern "C" {
    fn open(__file: *const c_char, __oflag: c_int, _: ...) -> c_int;
    fn lseek(__fd: c_int, __offset: __off64_t, __whence: c_int) -> __off64_t;
    fn read(__fd: c_int, __buf: *mut c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: c_int, __buf: *const c_void, __n: size_t) -> ssize_t;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> c_int;
    fn fopen(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const c_char, _: ...) -> c_int;
    fn fread(
        __ptr: *mut c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn perror(__s: *const c_char);
    fn fileno(__stream: *mut FILE) -> c_int;
    fn memset(
        _: *mut c_void,
        _: c_int,
        _: c_ulong,
    ) -> *mut c_void;
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
    fn strerror(_: c_int) -> *mut c_char;
    fn strtoul(
        __nptr: *const c_char,
        __endptr: *mut *mut c_char,
        __base: c_int,
    ) -> c_ulong;
    fn exit(_: c_int) -> !;
    fn __ctype_b_loc() -> *mut *const c_ushort;
    fn __errno_location() -> *mut c_int;
}

pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __ssize_t = c_long;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut c_void,
    pub __pad2: *mut c_void,
    pub __pad3: *mut c_void,
    pub __pad4: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}

pub type _IO_lock_t = ();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: c_int,
}

pub type FILE = _IO_FILE;
pub type C2RustUnnamed = c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Partitiondesc {
    pub boot: uint8_t,
    pub chs_begin: [uint8_t; 3],
    pub type_0: uint8_t,
    pub chs_end: [uint8_t; 3],
    pub lba_start: uint32_t,
    pub lba_length: uint32_t,
}

#[no_mangle]
pub static mut diskfd: c_int = 0;
#[no_mangle]
pub static mut maxoff: off_t = 0 as c_int as off_t;
#[no_mangle]
pub static mut curoff: off_t = 0 as c_int as off_t;

#[no_mangle]
pub unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: mkbootdisk BOOTSECTORFILE [FILE | @SECNUM]...\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        stderr,
        b"   or: mkbootdisk -p DISK [FILE | @SECNUM]...\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        stderr,
        b"   or: mkbootdisk -m KERNELFILE\n\0" as *const u8 as *const c_char,
    );
    exit(1 as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn fopencheck(mut name: *const c_char) -> *mut FILE {
    let mut f: *mut FILE = fopen(name, b"rb\0" as *const u8 as *const c_char);
    if f.is_null() {
        fprintf(
            stderr,
            b"%s: %s\n\0" as *const u8 as *const c_char,
            name,
            strerror(*__errno_location()),
        );
        usage();
    }
    return f;
}

#[no_mangle]
pub unsafe extern "C" fn diskwrite(mut data: *const c_void, mut amt: size_t) {
    if maxoff != 0
        && (curoff as c_ulong).wrapping_add(amt) > maxoff as c_ulong
    {
        fprintf(
            stderr,
            b"more data than allowed in partition!\n\0" as *const u8
                as *const c_char,
        );
        usage();
    }
    while amt > 0 as c_int as c_ulong {
        let mut w: ssize_t = write(diskfd, data, amt);
        if w == -(1 as c_int) as c_long
            && *__errno_location() != 4 as c_int
        {
            perror(b"write\0" as *const u8 as *const c_char);
            usage();
        } else if w == 0 as c_int as c_long {
            fprintf(
                stderr,
                b"write hit end of file\n\0" as *const u8 as *const c_char,
            );
            usage();
        } else if w > 0 as c_int as c_long {
            amt = (amt as c_ulong).wrapping_sub(w as c_ulong) as size_t
                as size_t;
            curoff += w;
            data = (data as *const c_uchar).offset(w as isize)
                as *const c_void;
        }
    }
}

#[no_mangle]
unsafe fn main(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut buf: [c_char; 4096] = [0; 4096];
    let mut zerobuf: [c_char; 512] = [0; 512];
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut n: size_t = 0;
    let mut nsectors: size_t = 0;
    let mut i: c_int = 0;
    let mut bootsector_special: c_int = 1 as c_int;
    diskfd = fileno(stdout);
    if argc >= 2 as c_int
        && strcmp(
            *argv.offset(1 as c_int as isize),
            b"-p\0" as *const u8 as *const c_char,
        ) == 0 as c_int
    {
        if argc < 3 as c_int {
            usage();
        }
        diskfd = open(*argv.offset(2 as c_int as isize), 0o2 as c_int);
        if diskfd < 0 as c_int {
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const c_char,
                *argv.offset(2 as c_int as isize),
                strerror(*__errno_location()),
            );
            usage();
        }
        if find_partition(
            0 as c_int as off_t,
            0 as c_int as off_t,
            0 as c_int,
        ) <= 0 as c_int
        {
            fprintf(
                stderr,
                b"%s: no JOS partition (type 0x27) found!\n\0" as *const u8
                    as *const c_char,
                *argv.offset(2 as c_int as isize),
            );
            usage();
        }
        argc -= 2 as c_int;
        argv = argv.offset(2 as c_int as isize);
        bootsector_special = 0 as c_int;
    }
    if argc >= 2 as c_int
        && strcmp(
            *argv.offset(1 as c_int as isize),
            b"-m\0" as *const u8 as *const c_char,
        ) == 0 as c_int
    {
        if argc < 3 as c_int {
            usage();
        }
        do_multiboot::<elf_program>(*argv.offset(2 as c_int as isize));
    }
    if argc < 2 as c_int {
        usage();
    }
    if bootsector_special != 0 {
        f = fopencheck(*argv.offset(1 as c_int as isize));
        n = fread(
            buf.as_mut_ptr() as *mut c_void,
            1 as c_int as size_t,
            4096 as c_int as size_t,
            f,
        );
        if n > 510 as c_int as c_ulong {
            fprintf(
                stderr,
                b"%s: boot block too large: %s%u bytes (max 510)\n\0" as *const u8
                    as *const c_char,
                *argv.offset(1 as c_int as isize),
                if n == 4096 as c_int as c_ulong {
                    b">= \0" as *const u8 as *const c_char
                } else {
                    b"\0" as *const u8 as *const c_char
                },
                n as c_uint,
            );
            usage();
        }
        fclose(f);
        memset(
            buf.as_mut_ptr().offset(n as isize) as *mut c_void,
            0 as c_int,
            (510 as c_int as c_ulong).wrapping_sub(n),
        );
        buf[510 as c_int as usize] = 0x55 as c_int as c_char;
        buf[511 as c_int as usize] = 0xaa as c_int as c_char;
        diskwrite(buf.as_mut_ptr() as *const c_void, 512 as c_int as size_t);
        nsectors = 1 as c_int as size_t;
        argc -= 1;
        argv = argv.offset(1);
    } else {
        nsectors = 0 as c_int as size_t;
    }
    memset(
        zerobuf.as_mut_ptr() as *mut c_void,
        0 as c_int,
        512 as c_int as c_ulong,
    );
    i = 1 as c_int;
    while i < argc {
        let mut pos: size_t = 0;
        let mut str: *mut c_char = 0 as *mut c_char;
        let mut skipto_sector: c_ulong = 0;
        if *(*argv.offset(i as isize)).offset(0 as c_int as isize) as c_int
            == '@' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*argv.offset(i as isize)).offset(1 as c_int as isize)
                        as c_int as isize,
                ) as c_int
                & _ISdigit as c_int as c_ushort as c_int != 0
            && {
                skipto_sector = strtoul(
                    (*argv.offset(i as isize)).offset(1 as c_int as isize),
                    &mut str,
                    0 as c_int,
                );
                *str as c_int == 0 as c_int
            }
        {
            if nsectors > skipto_sector {
                fprintf(
                    stderr,
                    b"mkbootdisk: can't skip to sector %u, already at sector %u\n\0"
                        as *const u8 as *const c_char,
                    skipto_sector as c_uint,
                    nsectors as c_uint,
                );
                usage();
            }
            while nsectors < skipto_sector {
                diskwrite(
                    zerobuf.as_mut_ptr() as *const c_void,
                    512 as c_int as size_t,
                );
                nsectors = nsectors.wrapping_add(1);
            }
        } else {
            f = fopencheck(*argv.offset(i as isize));
            pos = 0 as c_int as size_t;
            loop {
                n = fread(
                    buf.as_mut_ptr() as *mut c_void,
                    1 as c_int as size_t,
                    4096 as c_int as size_t,
                    f,
                );
                if !(n > 0 as c_int as c_ulong) {
                    break;
                }
                diskwrite(buf.as_mut_ptr() as *const c_void, n);
                pos = (pos as c_ulong).wrapping_add(n) as size_t as size_t;
            }
            if pos.wrapping_rem(512 as c_int as c_ulong)
                != 0 as c_int as c_ulong
            {
                diskwrite(
                    zerobuf.as_mut_ptr() as *const c_void,
                    (512 as c_int as c_ulong)
                        .wrapping_sub(
                            pos.wrapping_rem(512 as c_int as c_ulong),
                        ),
                );
                pos = (pos as c_ulong)
                    .wrapping_add(
                        (512 as c_int as c_ulong)
                            .wrapping_sub(
                                pos.wrapping_rem(512 as c_int as c_ulong),
                            ),
                    ) as size_t as size_t;
            }
            nsectors = (nsectors as c_ulong)
                .wrapping_add(pos.wrapping_div(512 as c_int as c_ulong))
                as size_t as size_t;
            fclose(f);
        }
        i += 1;
    }
    while nsectors < 1024 as c_int as c_ulong {
        diskwrite(
            zerobuf.as_mut_ptr() as *const c_void,
            512 as c_int as size_t,
        );
        nsectors = nsectors.wrapping_add(1);
    }
    return 0 as c_int;
}


const SECTORSIZE: usize = 512;

#[no_mangle]
unsafe extern "C" fn readsect(mut buf: *mut c_void, mut sectno: uint32_t) {
    let mut s: ssize_t = 0;
    let mut o: off_t = lseek(
        diskfd,
        sectno as off_t * 512 as c_int as off_t,
        0 as c_int,
    );
    if o == -(1 as c_int) as off_t {
        perror(b"lseek\0" as *const u8 as *const c_char);
        usage();
    }
    loop {
        s = read(diskfd, buf, 512 as c_int as size_t);
        if !(s == -(1 as c_int) as c_long
            && *__errno_location() == 4 as c_int)
        {
            break;
        }
    }
    if s != 512 as c_int as c_long {
        perror(b"read\0" as *const u8 as *const c_char);
        usage();
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_partition(
    mut partition_sect: off_t,
    mut extended_sect: off_t,
    mut partoff: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut r: c_int = 0;
    let mut buf: [uint8_t; 512] = [0; 512];
    let mut o: off_t = 0;
    let mut ptable: *mut Partitiondesc = 0 as *mut Partitiondesc;
    readsect(buf.as_mut_ptr() as *mut c_void, partition_sect as uint32_t);
    if buf[510 as c_int as usize] as c_int != 0x55 as c_int
        || buf[(510 as c_int + 1 as c_int) as usize] as c_int
            != 0xaa as c_int
    {
        return 0 as c_int;
    }
    ptable = buf.as_mut_ptr().offset(446 as c_int as isize) as *mut Partitiondesc;
    i = 0 as c_int;
    while i < 4 as c_int {
        if !((*ptable.offset(i as isize)).lba_length == 0 as c_int as c_uint)
        {
            if (*ptable.offset(i as isize)).type_0 as c_int == 0x27 as c_int
            {
                partition_sect += (*ptable.offset(i as isize)).lba_start as off_t;
                fprintf(
                    stderr,
                    b"Using partition %d (start sector %ld, sector length %ld)\n\0"
                        as *const u8 as *const c_char,
                    partoff + i + 1 as c_int,
                    partition_sect,
                    (*ptable.offset(i as isize)).lba_length as c_long,
                );
                o = lseek(
                    diskfd,
                    partition_sect * 512 as c_int as c_long,
                    0 as c_int,
                );
                if o != partition_sect * 512 as c_int as c_long {
                    fprintf(
                        stderr,
                        b"cannot seek to partition start: %s\n\0" as *const u8
                            as *const c_char,
                        strerror(*__errno_location()),
                    );
                    usage();
                }
                maxoff = (*ptable.offset(i as isize)).lba_length as off_t
                    * 512 as c_int as c_long;
                return 1 as c_int;
            } else {
                if (*ptable.offset(i as isize)).type_0 as c_int
                    == 0x5 as c_int
                    || (*ptable.offset(i as isize)).type_0 as c_int
                        == 0xf as c_int
                    || (*ptable.offset(i as isize)).type_0 as c_int
                        == 0x85 as c_int
                {
                    let mut inner_sect: off_t = extended_sect;
                    if inner_sect == 0 {
                        inner_sect = (*ptable.offset(i as isize)).lba_start as off_t;
                    }
                    r = find_partition(
                        (*ptable.offset(i as isize)).lba_start as c_long
                            + extended_sect,
                        inner_sect,
                        if partoff != 0 {
                            partoff + 1 as c_int
                        } else {
                            4 as c_int
                        },
                    );
                    if r > 0 as c_int {
                        return r;
                    }
                }
            }
        }
        i += 1;
    }
    return 0 as c_int;
}

#[no_mangle]
unsafe fn do_multiboot<elf_program>(filename: *const c_char) {
    let mut buf: [u8; SECTORSIZE] = [0; SECTORSIZE];
    let elfh = buf.as_ptr() as *const elf_header;
    let mut o: off_t = 0;

    diskfd = open(filename, O_RDWR);
    if diskfd < 0 {
        let err = strerror(*__errno_location());
        fprintf(
            stderr,
            b"%s: %s\n\0".as_ptr() as *const c_char,
            filename,
            err,
        );
        usage();
    }

    readsect(buf.as_mut_ptr(), 0);

    if (*elfh).e_magic != ELF_MAGIC {
        fprintf(
            stderr,
            b"%s: not an ELF executable file\n\0".as_ptr() as *const c_char,
            filename,
        );
        usage();
    }

    o = (*elfh).e_phoff + ((*elfh).e_phnum as off_t) * core::mem::size_of::<elf_program>() as off_t;
    if o >= (4096 - core::mem::size_of_val(&MULTIBOOT_HEADER)) as off_t {
        fprintf(
            stderr,
            b"%s: ELF header too large to accommodate multiboot header\n\0".as_ptr() as *const c_char,
            filename,
        );
        usage();
    } else if lseek(diskfd, o, SEEK_SET) != o {
        perror(b"lseek\0".as_ptr() as *const c_char);
        usage();
    }

    diskwrite(MULTIBOOT_HEADER.as_ptr(), core::mem::size_of_val(&MULTIBOOT_HEADER));
    libc::exit(0);
}
