// protect(), call() and unprotect() are required; a missing one is reported before the order.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
