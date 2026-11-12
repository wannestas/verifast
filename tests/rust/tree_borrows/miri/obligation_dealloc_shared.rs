// miri: ub
// Twin of free_parent and caller in protection_obligation_fails.rs, whose protection obligation
// fails: the callee deallocates, through the parent p, the memory behind its protected & argument.
use std::alloc::{alloc, dealloc, Layout};

unsafe fn free_parent(_x: &i32, p: *mut i32) {
    unsafe { dealloc(p as *mut u8, Layout::new::<i32>()) };
}

fn main() {
    unsafe {
        let p = alloc(Layout::new::<i32>()) as *mut i32;
        assert!(!p.is_null());
        *p = 0;
        free_parent(&*p, p);
    }
}
