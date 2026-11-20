// The markers must be at the top level of the clause.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { if true { protect(); } call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
