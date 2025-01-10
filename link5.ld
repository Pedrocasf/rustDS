MEMORY {
    ewram	: ORIGIN = 0x02000000, LENGTH = 16M
	dtcm	: ORIGIN = 0x02400000, LENGTH = 16K
	vectors	: ORIGIN = 0x01000000, LENGTH = 256
	itcm	: ORIGIN = 0x01000100, LENGTH = 32K - 256
}

ENTRY(Reset)

PROVIDE(_heap_size = 4M - 18816);

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


  /* rust-lld demands we keep the `section header string table` */
    .shstrtab        0 : { *(.shstrtab) }

    /* debugging sections */
    /* Stabs */
    .stab            0 : { *(.stab) }
    .stabstr         0 : { *(.stabstr) }
    .stab.excl       0 : { *(.stab.excl) }
    .stab.exclstr    0 : { *(.stab.exclstr) }
    .stab.index      0 : { *(.stab.index) }
    .stab.indexstr   0 : { *(.stab.indexstr) }
    .comment         0 : { *(.comment) }
    /* DWARF 1 */
    .debug           0 : { *(.debug) }
    .line            0 : { *(.line) }
    /* GNU DWARF 1 extensions */
    .debug_srcinfo   0 : { *(.debug_srcinfo) }
    .debug_sfnames   0 : { *(.debug_sfnames) }
    /* DWARF 1.1 and DWARF 2 */
    .debug_aranges   0 : { *(.debug_aranges) }
    .debug_pubnames  0 : { *(.debug_pubnames) }
    /* DWARF 2 */
    .debug_info      0 : { *(.debug_info) }
    .debug_abbrev    0 : { *(.debug_abbrev) }
    .debug_line      0 : { *(.debug_line) }
    .debug_frame     0 : { *(.debug_frame) }
    .debug_str       0 : { *(.debug_str) }
    .debug_loc       0 : { *(.debug_loc) }
    .debug_macinfo   0 : { *(.debug_macinfo) }
    /* SGI/MIPS DWARF 2 extensions */
    .debug_weaknames 0 : { *(.debug_weaknames) }
    .debug_funcnames 0 : { *(.debug_funcnames) }
    .debug_typenames 0 : { *(.debug_typenames) }
    .debug_varnames  0 : { *(.debug_varnames) }

    /* discard anything not already mentioned */
    /DISCARD/ : { *(*) }
    /DISCARD/ :
    {
        *(*);
    }
}