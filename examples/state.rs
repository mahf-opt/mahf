use better_any::{Tid, TidAble};
use derive_more::{Deref, DerefMut};
use mahf::{CustomState, State};

#[derive(Debug, Deref, DerefMut, Tid)]
pub struct A(usize);
impl CustomState<'_> for A {}

#[derive(Debug, Deref, DerefMut, Tid)]
pub struct B<'a>(&'a mut A);
impl<'a> CustomState<'a> for B<'a> {}

fn main() {
    let mut b_source = A(10);

    let mut state: State<A> = State::new();

    state.insert(A(0));
    state.insert(B(&mut b_source));

    let a = state.borrow::<A>();
    let _a2 = state.borrow::<A>();
    let mut b = state.borrow_mut::<B>();

    assert!(state.try_borrow_mut::<A>().is_err());

    println!("{a:?}");
    println!("{b:?}");

    b.0 .0 += 1;

    assert!(state.try_borrow_mut::<B>().is_err());

    println!("{b:?}");

    drop(a);
    drop(_a2);
    drop(b);

    let (a, b) = state.get_multiple_mut::<(A, B)>();
    a.0 += 1;
    b.0 .0 += 1;

    println!("{a:?}");
    println!("{b:?}");

    let a = state.entry::<A>().or_insert(A(0));
    println!("{a:?}");
    drop(a);

    assert!(state.try_get_multiple_mut::<(A, A)>().is_err());
}
