// A clause on an assume_correct function: its body is trusted, so there is nothing to check.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); call(); unprotect(); }
//@ assume_correct
{
    *x = 5;
}

fn main() {}
