
MEMORY {
	iwram	: ORIGIN = 0x037f8000, LENGTH = 96K	
    vram    : ORIGIN = 0x06000000, LENGTH = 256K
}

ENTRY(Reset)

__iwram_start	=	ORIGIN(iwram);
__iwram_top	=	ORIGIN(iwram)+ LENGTH(iwram);

__irq_vector	=   __iwram_top - 4;
__irq_flags	=	__irq_vector - 4;
__irq_flagsaux	=	__irq_flags - 0x34;

__sp_irq	=	__irq_flagsaux - 0x100;
__sp_svc	=	__sp_irq - 0x100;
__sp_usr	=	__sp_svc - 0x100;


SECTIONS {
    .text :
    {
        *(.text .text.*);
    } > iwram
    
    .rodata :
    {
        *(.rodata .rodata.*);
    } > iwram

    .bss :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > iwram

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > iwram

    _sidata = LOADADDR(.data);

    .irq :
    {
        KEEP(*(.irq));
    } > iwram

    _siirq = LOADADDR(.irq);

    /DISCARD/ :
    {
        *(.ARM.exidx.*);
    }
}