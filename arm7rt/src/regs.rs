use voladdress::*;

pub const POWCNT2: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000304) };
pub const SPICNT: VolAddress<u16,Safe, Safe> = unsafe { VolAddress::new(0x040001C0) };
pub const SPIDATA: VolAddress<u8,Safe, Safe> = unsafe { VolAddress::new(0x040001C2) };
pub const VRAMSTAT: VolAddress<u8,Safe, Safe> = unsafe { VolAddress::new(0x04000240) };
pub const IME: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000208) };
pub const IPCFIFOSEND: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000188) };
pub const IPCFIFOCNT: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000184) };
pub const IPCSYNC: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000180) };
pub const IE: VolAddress<u32,Safe, Safe> = unsafe { VolAddress::new(0x04000210) };
