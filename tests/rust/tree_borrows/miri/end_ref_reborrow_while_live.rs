// miri: ok
// Twin of `reborrow_with_remaining_ri` in ../end_ref_ri.rs.

fn main() {
    let mut v = 0;
    let x = &raw mut v;
    let r: &i32 = unsafe { &*x };
    let r2: &i32 = &*r;
    let _x = *r2;
    unsafe { *x = 5; }
}
