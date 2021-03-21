#![feature(const_generics)]

use euphony_sc::include_synthdef;

include_synthdef!(thing, "../euphony-sc-core/artifacts/v1.scsyndef");

#[test]
fn thing_test() {
    dbg!(thing::new().note(1));
}

macro_rules! set {
    ($receiver:expr, $($name:ident = $value:expr);* $(;)?) => {{
        $(
            $receiver.set::<{stringify!($name)}>($value);
        )*
        $receiver
    }}
}

trait Obj {
    fn set<const ID: &'static str>(&mut self, value: f32) -> &mut Self
    where
        Self: Set<ID>,
    {
        <Self as Set<ID>>::set_field(self, value)
    }
}

impl<T> Obj for T {}

trait Set<const ID: &'static str> {
    fn set_field(&mut self, value: f32) -> &mut Self;
}

impl Set<"freq"> for thing::New {
    fn set_field(&mut self, value: f32) -> &mut Self {
        // self.freq(value);
        self
    }
}

impl Set<"amp"> for thing::New {
    fn set_field(&mut self, value: f32) -> &mut Self {
        self.amp(value);
        self
    }
}

#[test]
fn generics_test() {
    let mut n = thing::new();
    dbg!(n);
    generic_obj(&mut n);
    dbg!(n);
}

#[props]
fn generic_obj<T: Prop<freq> + Prop<amp>>(n: &mut T) {
    set!(n, freq = 60.0; amp = 2.0);
}
