// A proof nested in a protection_proof clause keeps its own call(): here the code before protect() produces a
// function-pointer chunk, whose proof calls the function. Only the clause's top-level markers are the obligation's.
/*@

fn_type Bump(p: pred(*mut u8, i32)) = unsafe fn(data: *mut u8);
    req p(data, ?todo) &*& 0 < todo;
    ens p(data, todo - 1);

pred bump_data(data: *mut u8, todo: i32) = *(data as *mut i32) |-> 3 - todo;

@*/

type Bump = unsafe fn(data: *mut u8);

unsafe fn bump(data: *mut u8)
//@ req *(data as *mut i32) |-> ?count &*& count < 0x7fffffff;
//@ ens *(data as *mut i32) |-> count + 1;
{
    *(data as *mut i32) += 1;
}

unsafe fn set(x: &mut i32)
//@ req *x |-> _;
//@ ens *x |-> 5;
/*@
protection_proof {
    produce_fn_ptr_chunk Bump(bump)(bump_data)(data) {
        open bump_data(data, ?todo);
        call();
        close bump_data(data, todo - 1);
    }
    protect();
    call();
    unprotect();
}
@*/
{
    *x = 5;
}

fn main() {}
