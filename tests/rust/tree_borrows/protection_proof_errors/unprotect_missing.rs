// unprotect() is required.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); call(); }
{
    *x = 5;
}

fn main() {}
