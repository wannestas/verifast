// miri: ub
// Twin of id_alias in ../protection_proof_skip_retag.rs and ../protection_proof_skip_retag_placed.rs, whose
// protection obligations fail. The result is the caller's retag of the callee's return value, a child of the alias
// stored in slot. Write via the result, read via the alias (foreign read: the result becomes Frozen), write via the
// result again: UB.
unsafe fn id_alias(x: &mut i32, slot: *mut *mut i32) -> &mut i32 { unsafe { *slot = &raw mut *x; } x }
fn main() {
    let mut a = 0i32;
    let mut s: *mut i32 = std::ptr::null_mut();
    let r = unsafe { id_alias(&mut a, &raw mut s) };
    *r = 1;
    let _v = unsafe { *s };
    *r = 2;
}
