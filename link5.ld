MEMORY {
    ewram	: ORIGIN = 0x02000000, LENGTH = 16M
	dtcm	: ORIGIN = 0x02400000, LENGTH = 16K
	vectors	: ORIGIN = 0x01000000, LENGTH = 256
	itcm	: ORIGIN = 0x01000100, LENGTH = 32K - 256
}

ENTRY(Reset)

PROVIDE(_heap_size = 8M);

__ewram_end	    = ORIGIN(ewram) + LENGTH(ewram);
__sheap_end	    = ORIGIN(ewram) + LENGTH(ewram);

__itcm_start    = ORIGIN(itcm);

__dtcm_start	= ORIGIN(dtcm);
__dtcm_top	    = ORIGIN(dtcm) + LENGTH(dtcm);
__irq_flags	    = __dtcm_top - 0x08;
__irq_vector	= __dtcm_top - 0x04;

__sp_svc	=	__dtcm_top - 0x100;
__sp_irq	=	__sp_svc - 0x100;
__sp_usr	=	__sp_irq - 0x100;

SECTIONS {
    
    .secure : 
    {
        KEEP(*(.secure));
        . = ALIGN(4);
    } > ewram

    .text :
    {
        *(.text .text.*);
        . = ALIGN(4);
    } > ewram
    
    .rodata :
    {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } > ewram

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
        . = ALIGN(4);
    } > ewram

    _sidata = LOADADDR(.data);
    
    .bss (NOLOAD):
    {
        _sbss = .;
        *(.sbss .sbss.* .bss .bss.*);
        _ebss = .;
        . = ALIGN(4);
    } > ewram

    .heap (NOLOAD) :
    {
        _sheap = .;
        . += _heap_size;
        . = ALIGN(4);
        _eheap = .;
    } > ewram

    /DISCARD/ :
    {
        *(*);
    }
}