// The code between protect() and call() unprotects the protector made for x, uses the window (ends the fresh
// pointer, getting x's memory back), then protects x again with a new pointer. The obligation keeps that protector
// out of the heap until unprotect(), so the clause's unprotect_ref finds none.
unsafe fn set(x: &mut i32)
//@ req *x |-> ?v;
//@ ens *x |-> 5;
/*@
protection_proof {
    let x0 = x;
    protect();
    unprotect_ref(x); //~ should_fail
    end_ref(x);
    let x1 = protect_ref_mut(x0);
    x = x1;
    call();
    unprotect();
}
@*/
{
    *x = 5;
}

fn main() {}
