// pick returns c and keeps a raw alias of it in slot; its postcondition Bar(result, slot) says slot holds the result
// itself. Opening Bar, retagging and closing Bar at the retagged result is the right proof, but the close fails: the
// retag gives the result a new tag, and slot still holds the old one
// (Miri twin miri/obligation_alias_in_predicate.rs: UB).
/*@

pred Bar(p: *i32, slot: **i32) = *p |-> _ &*& [?k]ref_end_token::<i32>(p, ?z) &*& [k]ref_initialized::<i32>(p) &*& *slot |-> p;

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
    close Bar(r2, slot); //~ should_fail
}
@*/
{
    *slot = c;
    let r = &mut *c;
    //@ close Bar(r, slot);
    r
}

fn main() {}
