// protect() has no result to bind.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { let a = protect(); call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
