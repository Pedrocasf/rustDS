MEMORY {
	wram   : ORIGIN = 0x03800000, LENGTH = 64K
}

ENTRY(Reset)

__wram_start	=	ORIGIN(wram);
__wram_top	    =	ORIGIN(wram)+ LENGTH(wram);

__irq_vector	=   __wram_top - 4;
__irq_flags	    =	__irq_vector - 4;

__irq_flagsaux	=	__irq_flags - 0x34;
__sp_irq	    =	__irq_flagsaux - 0x100;

__sp_svc	    =	__sp_irq - 0x100;
__sp_usr	    =	__sp_svc - 0x100;

SECTIONS {
    _sbin = .;
    .text :
    {
        *(.text .text.*);
        . = ALIGN(4);
    } > wram
    
    .rodata :
    {
        *(.rodata .rodata.*);
        . = ALIGN(4);
    } > wram

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
        . = ALIGN(4);
    } > wram

    _sidata = LOADADDR(.data);

    .irq :
    {
        KEEP(*(.irq));
        . = ALIGN(4);
    } > wram

    _siirq = LOADADDR(.irq);

    .bss (NOLOAD) :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
        . = ALIGN(4);
    } > wram
    
    /DISCARD/ :
    {
        *(.ARM.exidx.*);
    }
}