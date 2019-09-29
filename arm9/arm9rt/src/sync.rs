use crate::regs::*;
pub fn sync(){    
    IPCSYNC.write(0x9<<8);
    while IPCSYNC.read() & 0xF != 0x7 {}
}