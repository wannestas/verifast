// A protection_proof clause on a required trait method: there is no body to check it on, so -check_protectors rejects it.
trait Tr {
    unsafe fn get(p: *mut i32) -> i32;
    //@ req *p |-> ?v;
    //@ ens *p |-> v &*& result == v;
    //@ protection_proof { protect(); let r = call(); unprotect(); }
}

fn main() {}
