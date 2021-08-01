// TODO
// * ops::{Add, Sub, Mul, Rem, Div} etc
// * filter_map
// * Generator::new(fn(context: &mut Context) -> (Option<T>, Status))
// * Divide
// * Euclidean
// * Polymetric
// * Rng

use core::marker::PhantomData;
use euphony_core::{
    ratio::Ratio,
    time::{
        beat::{Arc, Instant},
        Beat,
    },
};

pub trait Pattern: Sized {
    type Output;

    fn expansions(&self) -> usize;
    fn read(&self, context: &mut Context) -> Option<Self::Output>;
    fn status(&self, context: &mut Context) -> Status;
    fn end(&self) -> Ending;
}

pub trait HoldValue: Clone {
    fn hold(self) -> Hold<Self>;
}

impl<T> HoldValue for T
where
    T: Clone,
{
    fn hold(self) -> Hold<Self> {
        Hold::new(self)
    }
}

pub trait PatternExt: Pattern {
    fn map<F: Fn(Self::Output) -> T, T>(self, map: F) -> Map<Self, F, T> {
        Map::new(self, map)
    }

    fn filter<F: Fn(&Self::Output) -> bool>(self, filter: F) -> Filter<Self, F> {
        Filter::new(self, filter)
    }

    fn gate<P>(self, gate: P) -> Gate<Self, P>
    where
        P: Pattern,
    {
        Gate::new(self, gate)
    }

    fn cycle<P>(self, period: P) -> Cycle<Self, P>
    where
        P: Pattern<Output = Beat>,
    {
        Cycle::new(self, period)
    }

    fn scale<P>(self, amount: P) -> Scale<Self, P>
    where
        P: Pattern<Output = Ratio<u64>>,
    {
        Scale::new(self, amount)
    }

    fn translate<P>(self, amount: P) -> Translate<Self, P>
    where
        P: Pattern<Output = Beat>,
    {
        Translate::new(self, amount)
    }

    fn tick(self) -> Tick<Self>
    where
        Self: Pattern<Output = Beat>,
    {
        tick(self)
    }
}

impl<P> PatternExt for P where P: Pattern {}

pub struct Context<'a> {
    now: Instant,
    expansion: usize,
    // TODO rng
    rng: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub fn now(&self) -> Instant {
        self.now
    }

    pub fn expansion(&self) -> usize {
        self.expansion
    }
}

#[cfg(test)]
fn test_context() -> Context<'static> {
    Context {
        now: Instant(0, 1),
        expansion: 0,
        rng: PhantomData,
    }
}

#[cfg(test)]
macro_rules! combinator_test {
    ($pattern:expr, $expected:expr) => {{
        let pattern = $pattern;
        let expected = $expected;
        let mut actual = vec![];

        for expansion in 0..(pattern.expansions() + 1) {
            let mut context = test_context();
            context.expansion = expansion;
            let mut actual_expansion = vec![];
            let expected_len = expected.get(expansion).map(|v| v.len()).unwrap_or(0);

            loop {
                let output = pattern.read(&mut context);
                actual_expansion.push((context.now, output));

                match pattern.status(&mut context) {
                    Status::Finished => break,
                    Status::Pending(next) => context.now = next,
                }

                // only get enough samples for the expectation
                if actual_expansion.len() == expected_len {
                    break;
                }
            }

            actual.push(actual_expansion);
        }

        pretty_assertions::assert_eq!(&actual[..], &expected[..],);
    }};
}

macro_rules! impl_tuple {
    ($call:ident) => {
        impl_tuple!(
            @call
            $call;
            A(0),
            B(1),
            C(2),
            D(3),
            E(4),
            F(5),
            G(6),
            H(7),
            I(8),
            J(9),
            K(10),
            L(11),
            M(12),
            N(13),
            O(14),
            P(15),
            Q(16),
            R(17),
            S(18),
            T(19),
            U(20),
            V(21),
            W(22),
            X(23),
            Y(24),
            Z(25),
            AA(26),
            AB(27),
            AC(28),
            AD(29),
            AE(30),
            AF(31),
            AG(32),
            []
        );
    };
    (@call $call:ident; [$($acc:ident($a_value:tt),)*]) => {
        // done
    };
    (@call $call:ident; $head:ident($h_value:tt), $($tail:ident($t_value:tt), )* [$($acc:ident($a_value:tt),)*]) => {
        $call!([$($acc, )* $head], [$($a_value, )* $h_value]);
        impl_tuple!(@call $call; $($tail($t_value),)* [$($acc($a_value),)* $head($h_value),]);
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Finished,
    Pending(Instant),
}

impl Status {
    pub fn or(&mut self, other: Self) {
        match (self, other) {
            (Self::Pending(a), Self::Pending(b)) => *a = core::cmp::min(*a, b),
            (Self::Finished, Self::Finished) => {}
            (a, Self::Pending(_)) => *a = other,
            (Self::Pending(_), Self::Finished) => {}
        }
    }

    pub fn and(&mut self, other: Self) {
        match (self, other) {
            (Self::Pending(a), Self::Pending(b)) => *a = core::cmp::min(*a, b),
            (Self::Finished, Self::Finished) => {}
            (Self::Finished, Self::Pending(_)) => {}
            (Self::Pending(_), Self::Finished) => {}
        }
    }

    pub fn map(self, map: impl Fn(Instant) -> Instant) -> Self {
        match self {
            Self::Pending(v) => Self::Pending(map(v)),
            Self::Finished => Self::Finished,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ending {
    Indefinite,
    Definite(Instant),
}

impl Ending {
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Self::Definite(a), Self::Definite(b)) => Self::Definite(core::cmp::max(a, b)),
            (Self::Indefinite, Self::Indefinite) => Self::Indefinite,
            (Self::Indefinite, Self::Definite(b)) => Self::Definite(b),
            (Self::Definite(a), Self::Indefinite) => Self::Definite(a),
        }
    }

    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Self::Definite(a), Self::Definite(b)) => Self::Definite(core::cmp::min(a, b)),
            (Self::Indefinite, Self::Indefinite) => Self::Indefinite,
            (Self::Indefinite, Self::Definite(b)) => Self::Definite(b),
            (Self::Definite(a), Self::Indefinite) => Self::Definite(a),
        }
    }

    pub fn map(self, map: impl Fn(Instant) -> Instant) -> Self {
        match self {
            Self::Definite(v) => Self::Definite(map(v)),
            Self::Indefinite => Self::Indefinite,
        }
    }
}

/// Constantly emits a value
#[derive(Clone, Debug)]
pub struct Hold<T>
where
    T: Clone,
{
    value: T,
}

impl<T> Hold<T>
where
    T: Clone,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Pattern for Hold<T>
where
    T: Clone,
{
    type Output = T;

    fn expansions(&self) -> usize {
        0
    }

    fn read(&self, _context: &mut Context) -> Option<Self::Output> {
        Some(self.value.clone())
    }

    fn status(&self, _context: &mut Context) -> Status {
        Status::Finished
    }

    fn end(&self) -> Ending {
        Ending::Indefinite
    }
}

/// Returns a pattern that never emits a value
pub fn rest<T>() -> Rest<T> {
    Rest::new()
}

/// Never emits a value
#[derive(Clone, Debug)]
pub struct Rest<T> {
    value: PhantomData<T>,
}

impl<T> Rest<T> {
    pub fn new() -> Self {
        Self { value: PhantomData }
    }
}

impl<T> Pattern for Rest<T> {
    type Output = T;

    fn expansions(&self) -> usize {
        0
    }

    fn read(&self, _context: &mut Context) -> Option<Self::Output> {
        None
    }

    fn status(&self, _context: &mut Context) -> Status {
        Status::Finished
    }

    fn end(&self) -> Ending {
        Ending::Indefinite
    }
}

/// Emits a pattern only while another pattern returns a value
#[derive(Clone, Debug)]
pub struct Gate<P, C>
where
    P: Pattern,
    C: Pattern,
{
    pattern: P,
    condition: C,
}

impl<P, C> Gate<P, C>
where
    P: Pattern,
    C: Pattern,
{
    pub fn new(pattern: P, condition: C) -> Self {
        Self { pattern, condition }
    }
}

impl<P, C> Pattern for Gate<P, C>
where
    P: Pattern,
    C: Pattern,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions().max(self.condition.expansions())
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let _ = self.condition.read(context)?;
        self.pattern.read(context)
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.condition.status(context);
        status.or(self.pattern.status(context));
        status
    }

    fn end(&self) -> Ending {
        self.pattern.end().min(self.condition.end())
    }
}

#[test]
fn gate_test() {
    combinator_test!(123.hold().gate(().hold()), [[(Instant(0, 1), Some(123))]]);
    combinator_test!(
        123.hold()
            .gate(Beat(1, 1).hold().tick().filter(|b| b % 2 == 0)),
        [[
            (Instant(0, 1), Some(123)),
            (Instant(1, 1), None),
            (Instant(2, 1), Some(123)),
            (Instant(3, 1), None),
        ]]
    );
}

/// Scales time of one pattern by another
#[derive(Clone, Debug)]
pub struct Scale<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    pattern: P,
    amount: A,
}

impl<P, A> Scale<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    pub fn new(pattern: P, amount: A) -> Self {
        Self { pattern, amount }
    }
}

impl<P, A> Pattern for Scale<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions().max(self.amount.expansions())
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let amount = self.amount.read(context)?;
        // an empty period doesn't make sense
        if amount.0 == 0 {
            return None;
        }

        let now = context.now();
        context.now *= amount;
        let value = self.pattern.read(context);
        context.now = now;

        value
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.amount.status(context);

        if let Some(amount) = self.amount.read(context) {
            // an empty period doesn't make sense
            if amount.0 == 0 {
                return status;
            }

            let now = context.now();
            context.now *= amount;
            let p_status = self.pattern.status(context);
            context.now = now;
            status.or(p_status.map(|v| v / amount));
        }

        status
    }

    fn end(&self) -> Ending {
        self.pattern.end().min(self.amount.end())
    }
}

#[test]
fn scale_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().scale(Ratio(2, 1).hold()),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 2), Some(1)),
            (Instant(1, 1), Some(2)),
            (Instant(3, 2), Some(3)),
            (Instant(2, 1), Some(4)),
            (Instant(5, 2), Some(5)),
        ]]
    );
    combinator_test!(
        Beat(1, 1).hold().tick().scale(Hold::new(Ratio(1, 2))),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(2, 1), Some(1)),
            (Instant(4, 1), Some(2)),
            (Instant(6, 1), Some(3)),
            (Instant(8, 1), Some(4)),
            (Instant(10, 1), Some(5)),
        ]]
    );
    combinator_test!(
        Beat(1, 1)
            .hold()
            .tick()
            .scale(Beat(2, 1).hold().tick().map(|v| (v as u64 + 1).into())),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 1), Some(1)),
            (Instant(2, 1), Some(4)),
            (Instant(5, 2), Some(5)),
            (Instant(3, 1), Some(6)),
            (Instant(7, 2), Some(7)),
            (Instant(4, 1), Some(12)),
        ]]
    );
}

/// Translates time of one pattern by another
#[derive(Clone, Debug)]
pub struct Translate<P, A>
where
    P: Pattern,
    A: Pattern<Output = Beat>,
{
    pattern: P,
    amount: A,
}

impl<P, A> Translate<P, A>
where
    P: Pattern,
    A: Pattern<Output = Beat>,
{
    pub fn new(pattern: P, amount: A) -> Self {
        Self { pattern, amount }
    }
}

impl<P, A> Pattern for Translate<P, A>
where
    P: Pattern,
    A: Pattern<Output = Beat>,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions().max(self.amount.expansions())
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let amount = self.amount.read(context)?;

        let now = context.now();
        context.now += amount;
        let value = self.pattern.read(context);
        context.now = now;

        value
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.amount.status(context);

        if let Some(amount) = self.amount.read(context) {
            // an empty period doesn't make sense
            if amount.0 == 0 {
                return status;
            }

            let now = context.now();
            context.now += amount;
            let p_status = self.pattern.status(context);
            context.now = now;
            status.or(p_status.map(|v| v - amount));
        };

        status
    }

    fn end(&self) -> Ending {
        self.pattern.end().min(self.amount.end())
    }
}

#[test]
fn translate_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().translate(Beat(2, 1).hold()),
        [[
            (Instant(0, 1), Some(2)),
            (Instant(1, 1), Some(3)),
            (Instant(2, 1), Some(4)),
            (Instant(3, 1), Some(5)),
        ]]
    );
}

/// Emits a `()` when the Arc emitted by the pattern is active
#[derive(Clone, Debug)]
pub struct Window<P>
where
    P: Pattern<Output = Arc>,
{
    pattern: P,
}

impl<P> Window<P>
where
    P: Pattern<Output = Arc>,
{
    pub fn new(pattern: P) -> Self {
        Self { pattern }
    }
}

impl<P> Pattern for Window<P>
where
    P: Pattern<Output = Arc>,
{
    type Output = ();

    fn expansions(&self) -> usize {
        self.pattern.expansions()
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let arc = self.pattern.read(context)?;

        if !arc.contains(context.now()) {
            return None;
        }

        Some(())
    }

    fn status(&self, context: &mut Context) -> Status {
        // TODO use the arc to determine the next time
        self.pattern.status(context)
    }

    fn end(&self) -> Ending {
        self.pattern.end()
    }
}

/// Cycles after a number of beats
#[derive(Clone, Debug)]
pub struct Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    pattern: P,
    period: Period,
}

impl<P, Period> Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    pub fn new(pattern: P, period: Period) -> Self {
        Self { pattern, period }
    }
}

impl<P, Period> Pattern for Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions().max(self.period.expansions())
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let now = context.now();

        let period = self.period.read(context)?;

        // modulate the current time by the period
        let cycle_now: Instant = (now % period).into();

        // change what `now` the child pattern sees
        context.now = cycle_now;
        let value = self.pattern.read(context);
        context.now = now;

        value
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.period.status(context);

        let now = context.now();

        if let Some(period) = self.period.read(context) {
            // count how many cycles we've done
            let cycle_count = now / period;
            // modulate the current time by the period
            let cycle_now: Instant = (now % period).into();
            // compute when the current cycle would have started
            let cycle_start: Instant = (period * cycle_count.whole()).as_ratio().into();

            // let the caller know we change every cycle
            let next_cycle = cycle_start + period;
            status.or(Status::Pending(next_cycle));

            // change what `now` the child pattern sees
            context.now = cycle_now;
            let p_status = self.pattern.status(context);
            context.now = now;

            // we need to offset the returned time with the cycle start
            status.or(p_status.map(|v| v + cycle_start));
        }

        status
    }

    fn end(&self) -> Ending {
        self.pattern.end().min(self.period.end())
    }
}

#[test]
fn cycle_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().cycle(Beat(3, 1).hold()),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 1), Some(1)),
            (Instant(2, 1), Some(2)),
            (Instant(3, 1), Some(0)),
            (Instant(4, 1), Some(1)),
            (Instant(5, 1), Some(2)),
        ]]
    );
}

/// Creates a tick pattern
pub fn tick<P>(pattern: P) -> Tick<P>
where
    P: Pattern<Output = Beat>,
{
    Tick::new(pattern)
}

/// Ticks every period, returning how many periods have elapsed
#[derive(Clone, Debug)]
pub struct Tick<P>
where
    P: Pattern<Output = Beat>,
{
    pattern: P,
}

impl<P> Tick<P>
where
    P: Pattern<Output = Beat>,
{
    pub fn new(pattern: P) -> Self {
        Self { pattern }
    }
}

impl<P> Pattern for Tick<P>
where
    P: Pattern<Output = Beat>,
{
    type Output = u64;

    fn expansions(&self) -> usize {
        self.pattern.expansions()
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let period = self.pattern.read(context)?;

        let count = (context.now() / period).whole();
        Some(count)
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.pattern.status(context);

        if let Some(period) = self.pattern.read(context) {
            let count = (context.now() / period).whole();
            let next_cycle: Instant = (period * (count + 1)).as_ratio().into();
            status.or(Status::Pending(next_cycle));
        }

        status
    }

    fn end(&self) -> Ending {
        self.pattern.end()
    }
}

#[test]
fn tick_test() {
    combinator_test!(
        Beat(1, 1).hold().tick(),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 1), Some(1)),
            (Instant(2, 1), Some(2)),
            (Instant(3, 1), Some(3)),
        ]]
    );
}

/// Maps from one pattern value to another
#[derive(Clone, Debug)]
pub struct Map<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> T,
{
    pattern: P,
    map: F,
    output: PhantomData<T>,
}

impl<P, F, T> Map<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> T,
{
    pub fn new(pattern: P, map: F) -> Self {
        Self {
            pattern,
            map,
            output: PhantomData,
        }
    }
}

impl<P, F, T> Pattern for Map<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> T,
{
    type Output = T;

    fn expansions(&self) -> usize {
        self.pattern.expansions()
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        self.pattern.read(context).map(|v| (self.map)(v))
    }

    fn status(&self, context: &mut Context) -> Status {
        self.pattern.status(context)
    }

    fn end(&self) -> Ending {
        self.pattern.end()
    }
}

#[test]
fn map_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().map(|v| v * 2),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 1), Some(2)),
            (Instant(2, 1), Some(4)),
            (Instant(3, 1), Some(6)),
        ]]
    );
}

/// Filters the output from a pattern
#[derive(Clone, Debug)]
pub struct Filter<P, F>
where
    P: Pattern,
    F: Fn(&P::Output) -> bool,
{
    pattern: P,
    filter: F,
}

impl<P, F> Filter<P, F>
where
    P: Pattern,
    F: Fn(&P::Output) -> bool,
{
    pub fn new(pattern: P, filter: F) -> Self {
        Self { pattern, filter }
    }
}

impl<P, F> Pattern for Filter<P, F>
where
    P: Pattern,
    F: Fn(&P::Output) -> bool,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions()
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        self.pattern.read(context).filter(|v| (self.filter)(v))
    }

    fn status(&self, context: &mut Context) -> Status {
        self.pattern.status(context)
    }

    fn end(&self) -> Ending {
        self.pattern.end()
    }
}

#[test]
fn filter_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().filter(|v| v % 2 == 1),
        [[
            (Instant(0, 1), None),
            (Instant(1, 1), Some(1)),
            (Instant(2, 1), None),
            (Instant(3, 1), Some(3)),
        ]]
    );
}

#[derive(Clone, Debug)]
pub struct Select<G, P>
where
    G: Group,
    P: Pattern<Output = u64>,
{
    group: G,
    selector: P,
}

impl<G, P> Select<G, P>
where
    G: Group,
    P: Pattern<Output = u64>,
{
    pub fn new(group: G, selector: P) -> Self {
        Self { group, selector }
    }
}

impl<G, P> Pattern for Select<G, P>
where
    G: Group,
    P: Pattern<Output = u64>,
{
    type Output = G::Output;

    fn expansions(&self) -> usize {
        self.group.max_expansions().max(self.selector.expansions())
    }

    fn read(&self, context: &mut Context) -> Option<Self::Output> {
        let idx = self.selector.read(context)? as usize;
        let len = self.group.len();
        let idx = idx % len;
        self.group.read(idx, context)
    }

    fn status(&self, context: &mut Context) -> Status {
        let mut status = self.selector.status(context);

        if let Some(idx) = self.selector.read(context) {
            let idx = idx as usize;
            let len = self.group.len();
            let idx = idx % len;
            status.or(self.group.status(idx, context));
        }

        status
    }

    fn end(&self) -> Ending {
        todo!()
    }
}

#[test]
fn select_test() {
    combinator_test!(
        (1.hold(), 2.hold(), 3.hold(), 4.hold()).select(Beat(1, 1).hold().tick()),
        [[
            (Instant(0, 1), Some(1)),
            (Instant(1, 1), Some(2)),
            (Instant(2, 1), Some(3)),
            (Instant(3, 1), Some(4)),
            (Instant(4, 1), Some(1)),
            (Instant(5, 1), Some(2)),
            (Instant(6, 1), Some(3)),
            (Instant(7, 1), Some(4)),
        ]]
    );
    combinator_test!(
        (1.hold(), 2.hold(), 3.hold(), 4.hold())
            .select(Beat(1, 1).hold().tick())
            .scale(Ratio(4, 1).hold()),
        [[
            (Instant(0, 1), Some(1)),
            (Instant(1, 4), Some(2)),
            (Instant(2, 4), Some(3)),
            (Instant(3, 4), Some(4)),
            (Instant(1, 1), Some(1)),
        ]]
    );
    combinator_test!(
        (1.hold(), 2.hold(), 3.hold(), 4.hold())
            .select(Beat(1, 1).hold().tick())
            .translate(Beat(2, 1).hold())
            .scale(Ratio(4, 1).hold()),
        [[
            (Instant(0, 1), Some(3)),
            (Instant(1, 4), Some(4)),
            (Instant(2, 4), Some(1)),
            (Instant(3, 4), Some(2)),
            (Instant(1, 1), Some(3)),
        ]]
    );
    combinator_test!(
        (
            Beat(1, 1).hold().tick(),
            Beat(1, 2).hold().tick(),
            Beat(1, 4).hold().tick()
        )
            .select(Beat(1, 1).hold().tick()),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 1), Some(2)),
            (Instant(3, 2), Some(3)),
            (Instant(2, 1), Some(8)),
            (Instant(9, 4), Some(9)),
        ]]
    );
}

pub trait Group: Sized {
    type Output;

    fn len(&self) -> usize;
    fn expansions(&self, idx: usize) -> usize;
    fn max_expansions(&self) -> usize {
        let mut v = 0;
        for idx in 0..self.len() {
            v = v.max(self.expansions(idx));
        }
        v
    }
    fn read(&self, idx: usize, context: &mut Context) -> Option<Self::Output>;
    fn status(&self, idx: usize, context: &mut Context) -> Status;
    fn end(&self, idx: usize) -> Ending;
}

pub trait GroupExt: Group {
    fn select<P>(self, selector: P) -> Select<Self, P>
    where
        P: Pattern<Output = u64>,
    {
        Select::new(self, selector)
    }
}

impl<G> GroupExt for G where G: Group {}

impl<P> Group for &[P]
where
    P: Pattern,
{
    type Output = P::Output;

    fn len(&self) -> usize {
        <[P]>::len(self)
    }

    fn expansions(&self, idx: usize) -> usize {
        self[idx].expansions()
    }

    fn read(&self, idx: usize, context: &mut Context) -> Option<Self::Output> {
        self[idx].read(context)
    }

    fn status(&self, idx: usize, context: &mut Context) -> Status {
        self[idx].status(context)
    }

    fn end(&self, idx: usize) -> Ending {
        self[idx].end()
    }
}

macro_rules! impl_group {
    ([$($t:ident),*],[$($value:tt),*]) => {
        impl<$($t),*, Output> Group for ($($t,)*)
        where
            $(
                $t: Pattern<Output = Output>,
            )*
        {
            type Output = Output;

            fn expansions(&self, idx: usize) -> usize {
                match idx {
                    $(
                        $value => {
                            (self.$value).expansions()
                        }
                    )*
                    _ => panic!("invalid index"),
                }
            }

            fn read(&self, idx: usize, context: &mut Context) -> Option<Self::Output> {
                match idx {
                    $(
                        $value => {
                            (self.$value).read(context)
                        }
                    )*
                    _ => panic!("invalid index"),
                }
            }

            fn status(&self, idx: usize, context: &mut Context) -> Status {
                match idx {
                    $(
                        $value => {
                            (self.$value).status(context)
                        }
                    )*
                    _ => panic!("invalid index"),
                }
            }

            fn len(&self) -> usize {
                $(
                    let _idx = $value;
                )*
                _idx + 1
            }

            fn end(&self, idx: usize) -> Ending {
                match idx {
                    $(
                        $value => {
                            (self.$value).end()
                        }
                    )*
                    _ => panic!("invalid index"),
                }
            }
        }
    };
}

impl_tuple!(impl_group);

/*
/// Equally divides time between members of a group
#[derive(Clone, Debug)]
pub struct Divide<G>
where
    G: Group,
{
    pattern: P,
    filter: F,
}
*/

/*

macro_rules! tuple {
    ($(($T:ident, $idx:tt)),*) => {
        impl<A, $($T),*> Pattern for (A, $($T,)*)
        where
            A: Pattern,
            $(
                $T: Pattern<Output = A::Output>,
            )*
        {
            type Output = A::Output;

            fn poll(&self, context: &mut impl Context<Self::Output>) -> Status {
                let now = context.now();

                let mut status = Status::Finished;

                let mut sub_context = CycleContext::new(now, path![0], context);
                status.any(self.0.poll(&mut sub_context));

                $(
                    {
                        let mut sub_context = CycleContext::new(now, path![$idx], context);
                        status.any(self.$idx.poll(&mut sub_context));
                    }
                )*

                status
            }

            fn end(&self) -> Instant {
                let mut t = self.0.end();
                $(
                    t = t.max(self.$idx.end());
                )*
                t
            }
        }
    };
}

tuple!();
tuple!((B, 1));
tuple!((B, 1), (C, 2));

/// Emits a value every specified beat
#[derive(Clone, Debug)]
pub struct Periodic<T, P, D>
where
    T: Clone,
    P: Pattern<Output = Beat>,
    D: Pattern<Output = Beat>,
{
    value: T,
    period: P,
    duration: D,
}

impl<T, P, D> Periodic<T, P, D>
where
    T: Clone,
    P: Pattern<Output = Beat>,
    D: Pattern<Output = Beat>,
{
    pub fn new(value: T, period: P, duration: D) -> Self {
        Self {
            value,
            period,
            duration,
        }
    }
}

impl<T, P, D> Pattern for Periodic<T, P, D>
where
    T: Clone,
    P: Pattern<Output = Beat>,
    D: Pattern<Output = Beat>,
{
    type Output = T;

    fn poll(&mut self, context: &mut impl Context<Self::Output>) -> Status {
        let now = context.now();
        let divisions = now / self.period;

        if divisions.is_whole() {
            context.emit(self.value.clone(), self.duration);
        }

        let next = self.period * (divisions.whole() + 1);

        Status::Pending(next)
    }
}
*/
/*
#[test]
fn periodic_test() {
    let mut context = VecContext::default();

    let mut pattern = Periodic::new(1, Beat(1, 1), Beat(1, 2));

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(1, 1)));
    assert_eq!(context.tick(Beat(1, 2)), vec![(1, Instant(1, 2))]);

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(1, 1)));
    assert!(context.tick(Beat(1, 2)).is_empty());

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(2, 1)));
    assert_eq!(context.tick(Beat(1, 1)), vec![(1, Instant(3, 2))]);
}

/// Cycles after a number of beats
#[derive(Clone, Debug)]
struct Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    pattern: P,
    period: Period,
}

impl<P, Period> Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    pub fn new(pattern: P, period: Period) -> Self {
        Self { pattern, period }
    }
}

impl<P, Period> Pattern for Cycle<P, Period>
where
    P: Pattern,
    Period: Pattern<Output = Beat>,
{
    type Output = P::Output;

    fn poll(&self, context: &mut impl Context<Self::Output>) -> Status {
        let now = context.now();

        let mut period_context = VecContext::new(now);
        let mut status = self.period.poll(&mut period_context);

        for (index, (path, period)) in period_context.values.drain(..).enumerate() {
            let cycle_count = now / period;
            let cycle_now = now % period;
            let cycle_start: Instant = (period * cycle_count.whole()).as_ratio().into();

            let next_cycle = cycle_start + period;
            status.any(Status::Pending(next_cycle));

            let mut cycle_context =
                CycleContext::new(cycle_now.into(), path.with(path![index as u8]), context);

            let mut cycle_status = self.pattern.poll(&mut cycle_context);

            // shift the reported cycle by the start time
            cycle_status = cycle_status.map(|v| v + cycle_start);

            status.any(cycle_status);
        }

        status
    }

    fn end(&self) -> Instant {
        self.period.end()
    }
}

struct CycleContext<'a, T, C: Context<T>> {
    context: &'a mut C,
    value: PhantomData<T>,
    path: Path,
    now: Instant,
}

impl<'a, T, C: Context<T>> CycleContext<'a, T, C> {
    pub fn new(now: Instant, path: Path, context: &'a mut C) -> Self {
        Self {
            context,
            value: PhantomData,
            path,
            now,
        }
    }
}

impl<'a, T, C: Context<T>> Context<T> for CycleContext<'a, T, C> {
    fn now(&self) -> Instant {
        self.now
    }

    fn emit(&mut self, path: Path, value: T) {
        self.context.emit(self.path.with(path), value)
    }
}

#[test]
fn cycle_single_test() {
    let mut context = VecContext::default();

    let pattern = Cycle::new(Constant::new(123), Constant::new(Beat(2, 1)));

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(2, 1)));
    assert_eq!(&context.tick(Beat(2, 1))[..], &[(path![0], 123)][..]);

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(4, 1)));
    assert_eq!(&context.tick(Beat(2, 1))[..], &[(path![0], 123)][..]);
}

#[test]
fn cycle_multi_test() {
    let mut context = VecContext::default();

    let pattern = Cycle::new(
        (Constant::new(123), Constant::new(456)),
        (Constant::new(Beat(2, 1)), Constant::new(Beat(4, 1))),
    );

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(2, 1)));
    assert_eq!(&context.tick(Beat(2, 1))[..], &[(path![0], 123)][..]);

    assert_eq!(pattern.poll(&mut context), Status::Pending(Instant(4, 1)));
    assert_eq!(&context.tick(Beat(2, 1))[..], &[(path![0], 123)][..]);
}

/// Scales the sub-pattern's periods by a specified amount
pub struct Scale<P: Pattern, A: Pattern<Output = Ratio<u64>>> {
    pattern: P,
    amount: A,
}

impl<P: Pattern, A: Pattern<Output = Ratio<u64>>> Pattern for Scale<P, A> {
    type Output = P::Output;

    fn poll(&mut self, context: &mut impl Context<Self::Output>) -> Option<Beat> {
        let mut amount_context = VecContext::default();
        amount_context.now = context.now();

        self.amount.poll(&mut amount_context);

        todo!()
    }
}
*/
