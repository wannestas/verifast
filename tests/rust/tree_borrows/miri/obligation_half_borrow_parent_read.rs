// miri: ub
// Twin of callee in ../protection_proof_retag_before_protect.rs, whose protection obligation fails. The caller holds x,
// a child of yp (Foo(x, yp): x's memory and half of x's ref_end_token to yp). The callee's &mut is protected at entry; it writes it
// (Active), then reads through yp, x's parent, during the call.
unsafe fn callee(p: &mut i32, yp: *const i32) {
    *p = 1;
    let _v = unsafe { *yp };
}
fn main() {
    let mut a = 0i32;
    let yp: *mut i32 = &raw mut a;
    let x: &mut i32 = unsafe { &mut *yp };
    unsafe { callee(x, yp) };
}
