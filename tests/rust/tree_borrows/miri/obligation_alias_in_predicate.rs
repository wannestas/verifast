// miri: ub
// Twin of pick in ../protection_proof_close_at_retagged.rs, whose protection obligation fails. pick stores a raw alias of c
// in slot and returns &mut *c. The caller's retag of the result makes it a child of c's tag; reading through the
// alias (the parent) and then writing through the result again is UB.
unsafe fn pick<'a>(c: *mut i32, slot: *mut *mut i32) -> &'a mut i32 {
    unsafe { *slot = c; &mut *c }
}
fn main() {
    let mut v = 0i32;
    let c: *mut i32 = &raw mut v;
    let mut s: *mut i32 = std::ptr::null_mut();
    let slot: *mut *mut i32 = &raw mut s;
    unsafe {
        let r = pick(c, slot);
        *r = 1;
        let a = *slot;
        let _v = *a;
        *r = 2;
    }
}
