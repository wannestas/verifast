// protect() may appear only once.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); protect(); call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
