// miri: ok
// Twin of protection_obligation.rs: the positive callees, called with protected arguments.
use std::cell::UnsafeCell;

struct Pair { a: i32, b: i32 }
struct S { a: i32, c: UnsafeCell<i32> }

fn set(x: &mut i32) { *x = 5; }
fn get(x: &i32) -> i32 { *x }
fn first(x: &Pair) -> i32 { x.a }
fn bump(x: &S) { unsafe { *x.c.get() = x.a; } }
fn id_shared<'a>(x: &'a i32) -> &'a i32 { let raw = x as *const i32; unsafe { &*raw } }

fn main() {
    let mut v = 0;
    set(&mut v);
    assert!(get(&v) == 5);
    let pair = Pair { a: 1, b: 2 };
    assert!(first(&pair) == 1 && pair.b == 2);
    let s = S { a: 3, c: UnsafeCell::new(0) };
    bump(&s);
    assert!(unsafe { *s.c.get() } == 3);
    let r = id_shared(&v);
    assert!(*r == 5 && v == 5);
}
