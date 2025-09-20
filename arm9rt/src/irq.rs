use crate::regs::*;
use crate::*;

#[unsafe(no_mangle)]
pub static IRQ_HANDLE: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {
    if IF.read() & 1 == 1{
        IF.write(1);
    }
    return;
}

pub fn read_ipc_fifo() {
    let tam = IPCFIFORECV.read();
    let addr = IPCFIFORECV.read();
    let val = IPCFIFORECV.read();
    println!("TAM:{}, ADDR:{}, VAL:{}", tam, addr, val);
    halt();
}
