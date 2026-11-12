
unsafe fn write_parent(x: &mut i32, p: *mut i32)
//@ req *x |-> _ &*& ref_end_token(x, p) &*& ref_initialized(x);
//@ ens *p |-> 1;
{
    *x = 0;
    //@ end_ref(x);
    *p = 1;
}

unsafe fn read_parent(x: &mut i32, p: *const i32) -> i32
//@ req *x |-> _ &*& [1/2]ref_end_token(x, p as *i32) &*& ref_initialized(x);
//@ ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*(p as *i32) |-> w) &*& result == w;
{
    *x = 1;
    //@ end_ref(x);
    *p
}

unsafe fn free_parent(x: &i32, p: *mut i32)
//@ req [?f](*x |-> ?v) &*& [f]ref_end_token(x, p) &*& [f]ref_initialized(x) &*& [1 - f](*p |-> v) &*& std::alloc::alloc_block(p as *u8, std::alloc::Layout::new::<i32>());
//@ ens true;
{
    //@ end_ref(x);
    //@ to_u8s_(p);
    std::alloc::dealloc(p as *mut u8, std::alloc::Layout::new::<i32>());
}

unsafe fn caller()
//@ req true;
//@ ens true;
{
    let layout = std::alloc::Layout::new::<i32>();
    let p = std::alloc::alloc(layout) as *mut i32;
    if p.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    //@ from_u8s_(p);
    *p = 0;
    //@ let c = precreate_ref(p);
    //@ init_ref_readonly(c, 1/2);
    free_parent(&*p, p);
}

/*@

// Each f_spec is f's contract, which f's body satisfies at every argument.

pred half_token(r: *i32, x: *i32) = [1/2]ref_end_token::<i32>(r, x);

lem write_parent_spec(x: *i32, p: *i32)
    req *x |-> _ &*& ref_end_token(x, p) &*& ref_initialized(x);
    ens *p |-> 1;
{ assume(false); }

lem write_parent_obligation(x: *i32, p: *i32)
    req *x |-> _ &*& ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens *p |-> 1;
{
    let r = protect_ref_mut(x);
    assert pointer_within_limits(r) == true;
    write_parent_spec(r, p); //~ should_fail
    unprotect_ref(r);
    end_ref(r);
}

lem write_parent_chain_first(x: *i32, p: *i32)
    req *x |-> _ &*& ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens *p |-> 1;
{
    let r = protect_ref_mut(x);
    ref_end_token_trans(r); //~ should_fail
    write_parent_spec(r, p);
}

lem read_parent_spec(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x);
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{ assume(false); }

lem read_parent_obligation(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{
    let r = protect_ref_mut(x);
    assert pointer_within_limits(r) == true;
    let res = read_parent_spec(r, p); //~ should_fail
    unprotect_ref(r);
    end_ref(r);
    return res;
}

lem read_parent_chain_first(x: *i32, p: *i32) -> i32
    req *x |-> _ &*& [1/2]ref_end_token(x, p) &*& ref_initialized(x) &*& pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens [1/2](*x |-> ?w) &*& [1/2]ref_initialized(x) &*& [1/2](*p |-> w) &*& result == w;
{
    let r = protect_ref_mut(x);
    close half_token(r, x); //~ should_fail
    ref_end_token_trans(r);
    let res = read_parent_spec(r, p);
    open half_token(r, x);
    end_ref(r);
    return res;
}

lem free_parent_spec(x: *i32, p: *i32)
    req [?f](*x |-> ?v) &*& [f]ref_end_token(x, p) &*& [f]ref_initialized(x) &*& [1 - f](*p |-> v) &*& std::alloc::alloc_block(p as *u8, std::alloc::Layout::new::<i32>());
    ens true;
{ assume(false); }

lem free_parent_obligation(x: *i32, p: *i32)
    req [?f](*x |-> ?v) &*& [f]ref_end_token(x, p) &*& [f]ref_initialized(x) &*& [1 - f](*p |-> v) &*& std::alloc::alloc_block(p as *u8, std::alloc::Layout::new::<i32>()) &*&
        pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens true;
{
    let r = protect_ref(x, 1/2);
    assert pointer_within_limits(r) == true;
    free_parent_spec(r, p); //~ should_fail
    unprotect_ref(r);
    end_ref(r);
}

lem free_parent_chain_first(x: *i32, p: *i32)
    req [?f](*x |-> ?v) &*& [f]ref_end_token(x, p) &*& [f]ref_initialized(x) &*& [1 - f](*p |-> v) &*& std::alloc::alloc_block(p as *u8, std::alloc::Layout::new::<i32>()) &*&
        pointer_within_limits(x) == true &*& pointer_within_limits(p) == true;
    ens true;
{
    let r = protect_ref(x, 1/2);
    ref_end_token_trans(r); //~ should_fail
    end_ref(x);
    free_parent_spec(r, p);
}

@*/

fn main() {}
