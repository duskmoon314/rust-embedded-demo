ENTRY(_start)
BASE_ADDRESS = 0x80000000;

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
        *(.bss .bss.*)
    }
    . = ALIGN(4K);
    /DISCARD/ : {
        *(.eh_frame)
    }
}