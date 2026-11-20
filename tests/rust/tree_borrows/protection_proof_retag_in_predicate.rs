// As protection_proof_close_at_retagged.rs, but Bar says nothing about slot's contents, so Bar can be closed at the
// retagged result.
/*@

pred Bar(p: *i32, slot: **i32) = *p |-> _ &*& ref_end_token::<i32>(p, ?z) &*& ref_initialized::<i32>(p) &*& *slot |-> _;

@*/

unsafe fn pick<'a>(c: *mut i32, slot: *mut *mut i32) -> &'a mut i32
//@ req *c |-> _ &*& ref_end_token::<i32>(c, ?z) &*& ref_initialized::<i32>(c) &*& *slot |-> _;
//@ ens Bar(result, slot);
/*@
protection_proof {
    protect();
    let r = call();
    unprotect();
    open Bar(r, slot);
    let r2 = retag();
    close Bar(r2, slot);
}
@*/
{
    *slot = c;
    let r = &mut *c;
    //@ close Bar(r, slot);
    r
}

fn main() {}
