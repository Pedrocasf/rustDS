use critical_section::{set_impl, Impl, RawRestoreState};

use crate::IME;

struct DSiCriticalSection;
set_impl!(DSiCriticalSection);

unsafe impl Impl for DSiCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let restore = IME.read();
        IME.write(false);
        restore
    }

    unsafe fn release(restore: RawRestoreState) {
        IME.write(restore);
    }
}