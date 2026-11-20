// retag() on a function whose result is not retagged.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); call(); unprotect(); retag(); }
{
    *x = 5;
}

fn main() {}
