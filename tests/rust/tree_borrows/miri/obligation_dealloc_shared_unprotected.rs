// miri: ok
// Control for obligation_dealloc_shared.rs: the same accesses, with the & a local reborrow inside the
// callee instead of a parameter, so it is not protected.
use std::alloc::{alloc, dealloc, Layout};

unsafe fn free_parent(x: *const i32, p: *mut i32) {
    let _x: &i32 = unsafe { &*x };
    unsafe { dealloc(p as *mut u8, Layout::new::<i32>()) };
}

fn main() {
    unsafe {
        let p = alloc(Layout::new::<i32>()) as *mut i32;
        assert!(!p.is_null());
        *p = 0;
        free_parent(p, p);
    }
}
