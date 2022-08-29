ENTRY(_start)
BASE_ADDRESS = 0x40010000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    .data : {
        *(.data .data.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .bss : {
        *(.bss.stack)
        BOOT_STACK = .;
        . = . + 0x10000;
        BOOT_STACK_TOP = .;
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    /DISCARD/ : {
        *(.eh_frame)
    }
}

/* ENTRY(_start)
SECTIONS
{
    . = 0x40010000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(8);
    . = . + 0x4000;
    LD_STACK_PTR = .;
} */