MEMORY {
	ewram	: ORIGIN = 0x02000000, LENGTH = 16M
	dtcm	: ORIGIN = 0x0B000000, LENGTH = 16K
	vectors	: ORIGIN = 0x01000000, LENGTH = 256
	itcm	: ORIGIN = 0x01000100, LENGTH = 32K - 256
}

ENTRY(Reset)

__ewram_end	=	ORIGIN(ewram) + LENGTH(ewram);
__eheap_end	=	ORIGIN(ewram) + LENGTH(ewram);

__itcm_start	= ORIGIN(itcm);

__dtcm_start	= ORIGIN(dtcm);
__dtcm_top	=	ORIGIN(dtcm) + LENGTH(dtcm);
__irq_flags	=	__dtcm_top - 0x08;
__irq_vector	=	__dtcm_top - 0x04;

__sp_svc	=	__dtcm_top - 0x100;
__sp_irq	=	__sp_svc - 0x100;
__sp_usr	=	__sp_irq - 0x100;

SECTIONS {
    .secure:{
        KEEP(*(.secure))
    } > ewram

    .text :
    {
        *(.text .text.*);
    } > ewram
    
    .rodata :
    {
        *(.rodata .rodata.*);
    } > ewram

    .bss :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > dtcm

    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > dtcm

    _sidata = LOADADDR(.data);
    
    .irq :
    {
        KEEP(*(.irq));
    } > dtcm

    _siirq = LOADADDR(.irq);
    
    /DISCARD/ :
    {
        *(*);
    }
}