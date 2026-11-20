// Assigning to a parameter before protect() doesn't change what is protected: the obligation takes the arguments
// from its own state. Here x names y's memory when protect() runs; protecting it twice would fail. After unprotect(),
// x names the argument again.
unsafe fn set(x: &mut i32, y: &mut i32)
//@ req *x |-> _ &*& *y |-> _;
//@ ens *x |-> 5 &*& *y |-> 6;
//@ protection_proof { x = y; protect(); call(); unprotect(); assert *x |-> 5; }
{
    *x = 5;
    *y = 6;
}

fn main() {}
