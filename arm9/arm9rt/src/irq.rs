
use crate::regs::*;
use crate::*;
#[link_section = ".irq"]
pub static IRQ_HANDLE:extern "C" fn()= irq_handler;

extern "C" fn irq_handler(){
    read_ipc_fifo();
}

pub fn read_ipc_fifo() {
    let tam = IPCFIFORECV.read();
    let addr = IPCFIFORECV.read();
    let val = IPCFIFORECV.read();
    println!("TAM:{}, ADDR:{}, VAL:{}",tam,addr,val);
    halt();
}