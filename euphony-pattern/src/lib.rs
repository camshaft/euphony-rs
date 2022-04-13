// TODO
// * ops::{Add, Sub, Mul, Rem, Div} etc
// * Generator::new(fn(context: &mut Context) -> (Option<T>, Status))
// * Euclidean
// * Polymetric

use core::marker::PhantomData;
use euphony_core::{
    ratio::Ratio,
    time::{beat::Instant, Beat},
};

mod rng;

pub type Result<T> = core::result::Result<(T, Status), Status>;

pub trait ResultExt<T> {
    fn map_status<F: FnOnce(Status) -> Status>(self, f: F) -> Self;
    fn or_status(self, status: Status) -> Self;
    fn split_status(self) -> (Option<T>, Status);
}

impl<T> ResultExt<T> for Result<T> {
    fn map_status<F: FnOnce(Status) -> Status>(self, f: F) -> Self {
        match self {
            Ok((value, status)) => Ok((value, f(status))),
            Err(status) => Err(f(status)),
        }
    }

    fn or_status(self, status: Status) -> Self {
        self.map_status(|mut s| {
            s.or(status);
            s
        })
    }

    fn split_status(self) -> (Option<T>, Status) {
        match self {
            Ok((value, status)) => (Some(value), status),
            Err(status) => (None, status),
        }
    }
}

pub trait HoldValue: Clone {
    fn hold(self) -> Hold<Self>;
}

pub trait HoldGroup {
    type Pattern: Pattern;

    fn hold_group(self) -> Self::Pattern;
}

pub trait Pattern: Sized {
    type Output;

    fn expansions(&self) -> usize;
    fn poll(&self, context: &Context) -> Result<Self::Output>;
    fn period(&self, context: &Context) -> Beat;
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

    fn poll(&self, idx: usize, context: &Context) -> Result<Self::Output>;

    fn period(&self, idx: usize, context: &Context) -> Beat;

    fn period_total(&self, context: &Context) -> Beat {
        let mut v = Beat(0, 1);
        for idx in 0..self.len() {
            v += self.period(idx, context);
        }
        v
    }
}

pub trait Join: Sized {
    type Pattern: Pattern;

    fn join(self) -> Self::Pattern;
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

    fn filter_map<F: Fn(Self::Output) -> Option<T>, T>(self, map: F) -> FilterMap<Self, F, T> {
        FilterMap::new(self, map)
    }

    fn gate<P>(self, gate: P) -> Gate<Self, P>
    where
        P: Pattern,
    {
        Gate::new(self, gate)
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

    fn cycle<P>(self, period: P) -> Cycle<Self, P>
    where
        P: Pattern<Output = Beat>,
    {
        Cycle::new(self, period)
    }

    fn splice<P>(self, amount: P) -> Splice<Self, P>
    where
        P: Pattern<Output = Ratio<u64>>,
    {
        Splice::new(self, amount)
    }

    fn tick(self) -> Tick<Self>
    where
        Self: Pattern<Output = Beat>,
    {
        tick(self)
    }
}

impl<P> PatternExt for P where P: Pattern {}

pub trait GroupExt: Group {
    fn select<P>(self, selector: P) -> Select<Self, P>
    where
        P: Pattern<Output = u64>,
    {
        Select::new(self, selector)
    }

    fn divide(self) -> Divide<Self> {
        Divide::new(self)
    }
}

impl<G> GroupExt for G where G: Group {}

pub struct Context {
    // Scaled time
    now: Instant,
    // Actual time
    real: Instant,
    expansion: usize,
}

impl Context {
    pub fn new(now: Instant, expansion: usize) -> Self {
        Self {
            now,
            real: now,
            expansion,
        }
    }

    pub fn now(&self) -> Instant {
        self.now
    }

    pub fn expansion(&self) -> usize {
        self.expansion
    }

    pub fn child(&self, now: Instant) -> Self {
        Self {
            now,
            real: self.real,
            expansion: self.expansion,
        }
    }
}

#[cfg(test)]
fn test_context() -> Context {
    Context::new(Instant(0, 1), 0)
}

#[cfg(test)]
macro_rules! combinator_test {
    ($pattern:expr, $expected:expr) => {{
        let pattern = $pattern;
        let mut expected = $expected;

        // simplify the expected terms
        for exp in expected.iter_mut() {
            for (time, _) in exp.iter_mut() {
                *time = time.reduce();
            }
        }

        let mut actual = vec![];

        for expansion in 0..(pattern.expansions() + 1) {
            let mut context = test_context();
            context.expansion = expansion;
            let mut actual_expansion = vec![];
            let expected_len = expected.get(expansion).map(|v| v.len()).unwrap_or(0);

            loop {
                let (output, status) = pattern.poll(&context).split_status();
                actual_expansion.push((context.now, output));

                match status {
                    Status::Continuous => break,
                    Status::Pending(next) => {
                        assert!(next > context.now, "combinator going back in time");
                        context.now = next;
                        context.real = next;
                    }
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
    Continuous,
    Pending(Instant),
}

impl Status {
    pub fn or(&mut self, other: Self) {
        match (self, other) {
            (Self::Pending(a), Self::Pending(b)) => *a = core::cmp::min(*a, b),
            (Self::Continuous, Self::Continuous) => {}
            (a, Self::Pending(_)) => *a = other,
            (Self::Pending(_), Self::Continuous) => {}
        }
    }

    pub fn and(&mut self, other: Self) {
        match (self, other) {
            (Self::Pending(a), Self::Pending(b)) => *a = core::cmp::min(*a, b),
            (Self::Continuous, Self::Continuous) => {}
            (Self::Continuous, Self::Pending(_)) => {}
            (Self::Pending(_), Self::Continuous) => {}
        }
    }

    pub fn map(self, map: impl Fn(Instant) -> Instant) -> Self {
        match self {
            Self::Pending(v) => Self::Pending(map(v)),
            Self::Continuous => Self::Continuous,
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

    fn poll(&self, _context: &Context) -> Result<Self::Output> {
        let value = self.value.clone();
        let status = Status::Continuous;
        Ok((value, status))
    }

    fn period(&self, _context: &Context) -> Beat {
        Beat(1, 1)
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

    fn poll(&self, _context: &Context) -> Result<Self::Output> {
        Err(Status::Continuous)
    }

    fn period(&self, _context: &Context) -> Beat {
        Beat(1, 1)
    }
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (period, mut status) = self.pattern.poll(context)?;

        if period.0 == 0 {
            return Err(status);
        }

        let count = (context.now() / period).whole();

        let next_cycle: Instant = (period * (count + 1)).as_ratio().into();

        status.or(Status::Pending(next_cycle));

        Ok((count, status))
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

pub fn random<P>(seed: P) -> Random<P>
where
    P: Pattern<Output = u64>,
{
    Random::new(seed)
}

/// Generates a random u64 with a given seed
///
/// The output is completely determined from the current time and seed
#[derive(Copy, Clone, Debug)]
pub struct Random<P>
where
    P: Pattern<Output = u64>,
{
    seed: P,
}

impl<P> Random<P>
where
    P: Pattern<Output = u64>,
{
    pub fn new(seed: P) -> Self {
        Self { seed }
    }
}

impl<P> Pattern for Random<P>
where
    P: Pattern<Output = u64>,
{
    type Output = u64;

    fn expansions(&self) -> usize {
        0
    }

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (seed, status) = self.seed.poll(context)?;

        let value = rng::get(context.now(), context.expansion() as u64, seed);

        Ok((value, status))
    }

    fn period(&self, context: &Context) -> Beat {
        self.seed.period(context)
    }
}

#[test]
fn random_test() {
    combinator_test!(
        random(123.hold())
            .gate(Beat(1, 1).hold().tick())
            .map(|v| v % 4),
        [[
            (Instant(0, 1), Some(3)),
            (Instant(1, 1), Some(0)),
            (Instant(2, 1), Some(2)),
            (Instant(3, 1), Some(2)),
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (value, status) = self.pattern.poll(context)?;
        let value = (self.map)(value);
        Ok((value, status))
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (value, status) = self.pattern.poll(context)?;
        if (self.filter)(&value) {
            Ok((value, status))
        } else {
            Err(status)
        }
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

/// Filters the output from a pattern
#[derive(Clone, Debug)]
pub struct FilterMap<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> Option<T>,
{
    pattern: P,
    filter_map: F,
    output: PhantomData<T>,
}

impl<P, F, T> FilterMap<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> Option<T>,
{
    pub fn new(pattern: P, filter_map: F) -> Self {
        Self {
            pattern,
            filter_map,
            output: PhantomData,
        }
    }
}

impl<P, F, T> Pattern for FilterMap<P, F, T>
where
    P: Pattern,
    F: Fn(P::Output) -> Option<T>,
{
    type Output = T;

    fn expansions(&self) -> usize {
        self.pattern.expansions()
    }

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (value, status) = self.pattern.poll(context)?;
        if let Some(value) = (self.filter_map)(value) {
            Ok((value, status))
        } else {
            Err(status)
        }
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
    }
}

#[test]
fn filter_map_test() {
    combinator_test!(
        Beat(1, 1)
            .hold()
            .tick()
            .filter_map(|v| if v % 2 == 0 { Some(v + 1) } else { None }),
        [[
            (Instant(0, 1), Some(1)),
            (Instant(1, 1), None),
            (Instant(2, 1), Some(3)),
            (Instant(3, 1), None),
        ]]
    );
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (_, status) = self.condition.poll(context)?;

        self.pattern.poll(context).or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (amount, status) = self.amount.poll(context)?;

        if amount.0 == 0 {
            return Err(status);
        }

        let now = context.now();
        let child = context.child(now * amount);

        self.pattern
            .poll(&child)
            .map_status(|s| s.map(|v| v / amount))
            .or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        // TODO do we need to scale the period?
        self.pattern.period(context)
    }
}

#[test]
fn scale_test() {
    combinator_test!(
        Beat(1, 1).hold().tick().scale(Ratio(2, 1).hold()),
        [[
            (Instant(0, 1), Some(0)),
            (Instant(1, 2), Some(1)),
            (Instant(2, 2), Some(2)),
            (Instant(3, 2), Some(3)),
            (Instant(4, 2), Some(4)),
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
            (Instant(0, 2), Some(0)),
            (Instant(2, 2), Some(1)),
            (Instant(4, 2), Some(4)),
            (Instant(5, 2), Some(5)),
            (Instant(6, 2), Some(6)),
            (Instant(7, 2), Some(7)),
            (Instant(8, 2), Some(12)),
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (amount, status) = self.amount.poll(context)?;

        let now = context.now();
        let child = context.child(now + amount);

        self.pattern
            .poll(&child)
            .map_status(|s| s.map(|v| v - amount))
            .or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (period, mut status) = self.period.poll(context)?;

        if period.0 == 0 {
            return Err(status);
        }

        let now = context.now();
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
        let result = self.pattern.poll(&context.child(cycle_now));

        // we need to offset the returned time with the cycle start
        result
            .map_status(|s| s.map(|v| v + cycle_start))
            .or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        self.pattern.period(context)
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

/// Scales group member time by another pattern
#[derive(Clone, Debug)]
pub struct Splice<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    pattern: P,
    amount: A,
}

impl<P, A> Splice<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    pub fn new(pattern: P, amount: A) -> Self {
        Self { pattern, amount }
    }
}

impl<P, A> Pattern for Splice<P, A>
where
    P: Pattern,
    A: Pattern<Output = Ratio<u64>>,
{
    type Output = P::Output;

    fn expansions(&self) -> usize {
        self.pattern.expansions().max(self.amount.expansions())
    }

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (_amount, status) = self.amount.poll(context)?;
        self.pattern.poll(context).or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        let period = self.pattern.period(context);

        if let Ok((amount, _)) = self.amount.poll(context) {
            period * amount
        } else {
            Beat(1, 1)
        }
    }
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

    fn idx(&self, context: &Context) -> Result<usize> {
        let (idx, status) = self.selector.poll(context)?;
        let idx = idx as usize;
        let len = self.group.len();
        let idx = idx % len;
        Ok((idx, status))
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

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let (idx, status) = self.idx(context)?;
        let result = self.group.poll(idx, context);
        result.or_status(status)
    }

    fn period(&self, context: &Context) -> Beat {
        if let Ok((idx, _)) = self.idx(context) {
            self.group.period(idx, context)
        } else {
            Beat(1, 1)
        }
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
            (Instant(0, 4), Some(1)),
            (Instant(1, 4), Some(2)),
            (Instant(2, 4), Some(3)),
            (Instant(3, 4), Some(4)),
            (Instant(4, 4), Some(1)),
        ]]
    );
    combinator_test!(
        (1.hold(), 2.hold(), 3.hold(), 4.hold())
            .select(Beat(1, 1).hold().tick())
            .translate(Beat(2, 1).hold())
            .scale(Ratio(4, 1).hold()),
        [[
            (Instant(0, 4), Some(3)),
            (Instant(1, 4), Some(4)),
            (Instant(2, 4), Some(1)),
            (Instant(3, 4), Some(2)),
            (Instant(4, 4), Some(3)),
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
            (Instant(0, 4), Some(0)),
            (Instant(4, 4), Some(2)),
            (Instant(6, 4), Some(3)),
            (Instant(8, 4), Some(8)),
            (Instant(9, 4), Some(9)),
        ]]
    );
}

/// Equally divides a period between members of a group based on their period
#[derive(Clone, Debug)]
pub struct Divide<G>
where
    G: Group,
{
    group: G,
}

impl<G> Divide<G>
where
    G: Group,
{
    pub fn new(group: G) -> Self {
        Self { group }
    }
}

impl<G> Pattern for Divide<G>
where
    G: Group,
{
    type Output = G::Output;

    fn expansions(&self) -> usize {
        self.group.max_expansions()
    }

    fn poll(&self, context: &Context) -> Result<Self::Output> {
        let period = self.group.period_total(context).as_ratio();

        if period.0 == 0 {
            return Err(Status::Continuous);
        }

        let now = context.now();
        // figure out the start of the current cycle
        let cycle_start: Instant = now.whole().into();
        // figure out where we are in the cycle
        let cycle_cursor: Instant = (now - cycle_start) * period;
        let cycle_cursor: Beat = cycle_cursor.as_ratio().into();

        // keep track of each of the starting points
        let mut child_start = Beat(0, 1);

        for idx in 0..self.group.len() {
            let child_period = self.group.period(idx, context);

            // skip over empty periods
            if child_period.0 == 0 {
                continue;
            }

            let child_end = child_start + child_period;

            // keep searching
            if cycle_cursor >= child_end {
                child_start = child_end;
                continue;
            }

            let mut child_cursor = cycle_cursor;
            // shift the cursor back to the start of the child
            child_cursor -= child_start;
            // scale the cursor by the amount of time it occupies in the period
            child_cursor *= child_period.as_ratio();

            let child_now = cycle_start + child_cursor;

            let child = context.child(child_now);
            let result = self.group.poll(idx, &child);

            return result.map_status(|status| {
                let mut status = status.map(|time| {
                    // invert the time scaling
                    let mut child_cursor = time - child_now;
                    child_cursor /= period;
                    child_cursor += now;
                    child_cursor
                });

                // status should change on the next group member
                status.or(Status::Pending(cycle_start + (child_end / period)));

                status
            });
        }

        unreachable!()
    }

    fn period(&self, context: &Context) -> Beat {
        self.group.period_total(context)
    }
}

#[test]
fn divide_test() {
    combinator_test!(
        (
            Beat(1, 1).hold().tick().map(|_| 1),
            Beat(1, 2).hold().tick().map(|_| 2),
        )
            .divide(),
        [[
            (Instant(0, 4), Some(1)),
            (Instant(2, 4), Some(2)),
            (Instant(3, 4), Some(2)),
            (Instant(4, 4), Some(1)),
            (Instant(6, 4), Some(2)),
            (Instant(7, 4), Some(2)),
            (Instant(8, 4), Some(1)),
            (Instant(10, 4), Some(2)),
            (Instant(11, 4), Some(2)),
        ]]
    );
    combinator_test!(
        (
            Beat(1, 1).hold().tick().map(|_| 1),
            Beat(1, 2).hold().tick().map(|_| 2),
            Beat(1, 4).hold().tick().map(|_| 3),
        )
            .divide(),
        [[
            (Instant(0, 1), Some(1)),
            (Instant(2, 6), Some(2)),
            (Instant(3, 6), Some(2)),
            (Instant(8, 12), Some(3)),
            (Instant(9, 12), Some(3)),
            (Instant(10, 12), Some(3)),
            (Instant(11, 12), Some(3)),
        ]]
    );
    combinator_test!(
        (
            Beat(1, 1)
                .hold()
                .tick()
                .map(|_| 1)
                .splice(Ratio(2, 1).hold()),
            Beat(1, 2).hold().tick().map(|_| 2),
            Beat(1, 4).hold().tick().map(|_| 3),
        )
            .divide(),
        [[
            (Instant(0, 16), Some(1)),
            (Instant(4, 16), Some(1)),
            (Instant(8, 16), Some(2)),
            (Instant(10, 16), Some(2)),
            (Instant(12, 16), Some(3)),
            (Instant(13, 16), Some(3)),
            (Instant(14, 16), Some(3)),
            (Instant(15, 16), Some(3)),
            (Instant(16, 16), Some(1)),
        ]]
    );
}

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

    fn poll(&self, idx: usize, context: &Context) -> Result<Self::Output> {
        self[idx].poll(context)
    }

    fn period(&self, idx: usize, context: &Context) -> Beat {
        self[idx].period(context)
    }
}

impl<P, const N: usize> Group for [P; N]
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

    fn poll(&self, idx: usize, context: &Context) -> Result<Self::Output> {
        self[idx].poll(context)
    }

    fn period(&self, idx: usize, context: &Context) -> Beat {
        self[idx].period(context)
    }
}

/*
impl<V: Clone, const N: usize> HoldGroup for [V; N] {
    type Pattern = [Hold<V>; N];

    fn hold_group(self) -> Self::Pattern {
        todo!()
    }
}
*/

macro_rules! impl_group {
    ([$($t:ident),*],[$($value:tt),*]) => {
        impl<$($t),*> Pattern for ($($t,)*)
        where
            $(
                $t: Pattern,
            )*
        {
            type Output = (
                $(
                    Option<$t::Output>,
                )*
            );

            fn expansions(&self) -> usize {
                0 $(
                  +  (self.$value).expansions()
                )*
            }

            fn poll(&self, context: &Context) -> Result<Self::Output> {
                let mut status = Status::Continuous;
                let mut has_value = false;

                let value = (
                    $({
                        let (value, s) = (self.$value).poll(context).split_status();
                        status.or(s);
                        has_value |= value.is_some();
                        value
                    },)*
                );

                if has_value {
                    Ok((value, status))
                } else {
                    Err(status)
                }
            }

            fn period(&self, _context: &Context) -> Beat {
                // TODO does this even make sense to do?
                Beat(1, 1)
            }
        }

        impl<$($t),*> HoldGroup for ($($t,)*)
        where
            $(
                $t: HoldValue,
            )*
        {
            type Pattern = (
                $(
                    Hold<$t>,
                )*
            );

            fn hold_group(self) -> Self::Pattern {
                (
                    $(
                        (self.$value).hold(),
                    )*
                )
            }
        }

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

            fn poll(&self, idx: usize, context: &Context) -> Result<Self::Output> {
                match idx {
                    $(
                        $value => {
                            (self.$value).poll(context)
                        }
                    )*
                    _ => panic!("invalid index"),
                }
            }

            fn period(&self, idx: usize, context: &Context) -> Beat {
                match idx {
                    $(
                        $value => {
                            (self.$value).period(context)
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
        }
    };
}

impl_tuple!(impl_group);
