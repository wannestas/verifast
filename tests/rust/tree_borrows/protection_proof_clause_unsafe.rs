// A protection_proof clause on a function without a prototype. Without -check_protectors it is dropped.
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { protect(); call(); unprotect(); }
{
    *x = 5;
}

fn main() {}
