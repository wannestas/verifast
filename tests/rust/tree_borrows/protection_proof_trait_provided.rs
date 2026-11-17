// A protection_proof clause on a provided trait method doesn't stop the check of an impl's spec against the trait's.
trait Tr {
    unsafe fn get(p: &mut i32) -> i32
    //@ req *p |-> ?v;
    //@ ens *p |-> v &*& result == v;
    //@ protection_proof { protect(); let r = call(); unprotect(); }
    {
        *p
    }
}

struct S;

impl Tr for S {
    unsafe fn get(p: &mut i32) -> i32
    //@ req *p |-> ?v &*& v == 5; //~ should_fail
    //@ ens *p |-> v &*& result == v;
    {
        *p
    }
}

fn main() {}
