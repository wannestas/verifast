// A projection under a raw-pointer pointee is not yet supported; decode_ty drops a raw pointer's pointee type,
// so the translated type is checked.
trait Tr {
    type A;
}

unsafe fn touch<T: Tr>(x: &mut *mut T::A)
//@ req *x |-> ?v;
//@ ens *x |-> v;
//@ protection_proof { protect(); call(); unprotect(); }
{
}

fn main() {}
