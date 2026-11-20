// call() must come after protect().
unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
//@ protection_proof { call(); protect(); unprotect(); }
{
    *x = 5;
}

fn main() {}
