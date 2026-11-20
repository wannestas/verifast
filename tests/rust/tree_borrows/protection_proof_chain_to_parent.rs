// The code between protect() and call() unprotects the protector made for x, chains half of the fresh pointer's
// ref_end_token to x's parent p, and locks the rest again, so that the callee could end half of its own protected
// borrow and read p mid-call (Miri twin miri/obligation_parent_read.rs: UB). The obligation keeps that protector out
// of the heap until unprotect(), so the clause's unprotect_ref finds none.
unsafe fn read_parent(x: &mut i32, p: *const i32) -> i32
//@ req *x |-> _ &*& [1/2]ref_end_token(x, p as *i32) &*& ref_initialized(x);
//@ ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*(p as *i32) |-> w) &*& result == w;
/*@
protection_proof {
    let x0 = x;
    protect();
    unprotect_ref(x); //~ should_fail
    chain_half(x);
    lock_half_to(x, x0);
    let r = call();
    unprotect();
}
@*/
{
    *x = 1;
    //@ end_ref(x);
    *p
}

/*@

lem chain_half(r: *i32)
    req [1/2]ref_end_token::<i32>(r, ?x) &*& [1/2]ref_end_token::<i32>(x, ?p) &*& [1/2]ref_initialized::<i32>(x);
    ens [1/2]ref_end_token::<i32>(r, p);
{ ref_end_token_trans(r); }

lem lock_half_to(r: *i32, x: *i32)
    req [1/2]ref_end_token::<i32>(r, x);
    ens ref_protector::<i32>(r, x, 1/2);
{ lock_ref(r); }

@*/

fn main() {}
