// miri: ok
// Twin of `chain_then_end` and `partial_chain` in ../ref_end_token_trans.rs.

fn main() {
    let mut v = 0;
    let g = &raw mut v;
    let rp: &i32 = unsafe { &*g };
    let rawp = rp as *const i32;
    let ra: &i32 = unsafe { &*rawp };
    let _result = *ra;
    unsafe { *g = 5; }
}
