// marks One and Two as states of State
pub trait State {}
struct One {}
struct Two {}
impl State for One {}
impl State for Two {}

// to transition to State = Two
trait ToTwo where Self: StateHolder<One> {
    fn two(self) -> impl StateHolder<Two>;
} 

// to transition to State = One
trait ToOne where Self: StateHolder<Two> {
    fn one(self) -> impl StateHolder<One>;
} 

// trait that marks something as typestate-able over State
pub trait StateHolder<S: State> {}

// our experiment struct
struct Bar<S: State>(S);

// Bar is now typestate-enforced
impl<S: State> StateHolder<S> for Bar<S> {}

// Bar<One> can transition to Two
impl ToOne for Bar<Two> {
    fn one(self) -> Bar<One> {
        Bar(One{})
    }
}

// and Bar<Two> can transition to One
impl ToTwo for Bar<One> {
    fn two(self) -> Bar<Two> {
        Bar(Two{})
    }
}

// TODO: i return `impl StateHolder<...>` but this means they can return something that isn't Self<...>, which is not ideal.

pub fn testing() {
    let bar = Bar(One{});
    bar.two().one().two();
}

