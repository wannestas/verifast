// A clause on a function without references in its parameter and result types is an error: there is nothing to
// prove.
unsafe fn get(p: *mut i32, n: i32) -> i32
//@ req *p |-> ?v;
//@ ens *p |-> v &*& result == v;
//@ protection_proof { protect(); let r = call(); unprotect(); }
{
    *p
}

fn main() {}
