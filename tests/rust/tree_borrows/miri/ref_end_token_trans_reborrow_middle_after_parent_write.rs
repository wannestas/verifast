// miri: ub
// Twin of `reborrow_middle_after_parent_write` in ../ref_end_token_trans.rs.

fn main() {
    let mut v = 0;
    let g = &raw mut v;
    let rp: &i32 = unsafe { &*g };
    let rawp = rp as *const i32;
    let _ra: &i32 = unsafe { &*rawp };
    unsafe { *g = 5; }
    let _r2: &i32 = &*rp;
}
