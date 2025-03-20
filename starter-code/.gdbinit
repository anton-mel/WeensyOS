init-if-undefined $target_set = 0
if $target_set == 0
    target remote localhost:1234
    file obj/kernel.full
    add-symbol-file obj/bootsector.full 0x7c00
    add-symbol-file obj/p-allocator.full 0x1C0000
    add-symbol-file obj/p-malloc.full 0x100000
    add-symbol-file obj/p-alloctests.full 0x2C0000
    add-symbol-file obj/p-test.full 0x100000
    source build/functions.gdb
    display/5i $pc
    set $target_set = 1
end
