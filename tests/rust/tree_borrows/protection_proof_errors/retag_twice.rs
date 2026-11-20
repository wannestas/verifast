// retag() may appear only once.
unsafe fn id(x: &mut i32) -> &mut i32
//@ req *x |-> ?v;
//@ ens *result |-> v &*& ref_end_token(result, x) &*& ref_initialized(result);
//@ protection_proof { protect(); let r = call(); unprotect(); retag(); retag(); }
{
    //@ assume(false);
    x
}

fn main() {}
