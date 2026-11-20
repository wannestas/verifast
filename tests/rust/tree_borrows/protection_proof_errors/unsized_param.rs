// A ?Sized pointee is not yet supported: the protect lemmas take a sized T.
unsafe fn set<T: ?Sized>(x: &mut T, v: *mut T)
//@ req true;
//@ ens true;
//@ protection_proof { protect(); call(); unprotect(); }
{
}

fn main() {}
