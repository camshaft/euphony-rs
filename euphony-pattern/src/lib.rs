use core::{
    cmp::Reverse,
    ops::{self, Range},
};
use euphony_core::time::Beat;
pub use euphony_pattern_macros::p as p_impl;
use num_integer::lcm;
use priority_queue::PriorityQueue;
use slab::Slab;

pub mod euphony_pattern {
    pub use crate::*;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Arc {
    start: Beat,
    end: Beat,
}

impl Default for Arc {
    fn default() -> Self {
        Self {
            start: Beat(0, 1),
            end: Beat(1, 1),
        }
    }
}

impl Arc {
    pub fn cycles(&self) -> ArcCycles {
        ArcCycles(self.start, self.end)
    }

    pub fn from_cycle(cycle: usize) -> Self {
        let start = Beat(cycle as u64, 1);
        let end = Beat(cycle as u64 + 1, 1);
        (start..end).into()
    }
}

fn lcm_slice<I: Copy + num_integer::Integer>(nums: &[I]) -> Option<I> {
    // TODO can this be optimized?
    nums.iter().copied().reduce(|a, b| lcm(a, b))
}

pub struct ArcCycles(Beat, Beat);

impl Iterator for ArcCycles {
    type Item = Arc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1 {
            return None;
        }

        let end = if !self.0.is_whole() {
            self.0.ceil()
        } else {
            self.0 + 1
        }
        .min(self.1);

        let arc = (self.0..end).into();
        self.0 = end;

        Some(arc)
    }
}

#[cfg(test)]
mod arc_tests {
    use super::*;

    #[test]
    fn cycles() {
        macro_rules! t {
            ($arc:expr, $expected:expr) => {{
                let actual: Vec<_> = (Arc::from($arc)).cycles().collect();
                let expected: Vec<Arc> = ($expected).iter().map(|r| Arc::from(r.clone())).collect();
                assert_eq!(actual, expected);
            }};
        }

        t!(Beat(0, 1)..Beat(1, 1), [Beat(0, 1)..Beat(1, 1)]);
        t!(
            Beat(0, 1)..Beat(2, 1),
            [Beat(0, 1)..Beat(1, 1), Beat(1, 1)..Beat(2, 1)]
        );
        t!(
            Beat(1, 2)..Beat(3, 2),
            [Beat(1, 2)..Beat(1, 1), Beat(1, 1)..Beat(3, 2)]
        );
    }
}

impl From<Range<Beat>> for Arc {
    fn from(r: Range<Beat>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[macro_export]
macro_rules! p {
    ($($t:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::euphony_pattern;
        $crate::p_impl!($($t)*)
    }};
}

pub trait Pattern {
    type Output;

    fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
        let _ = arc;
        let _ = stream;
        todo!()
    }

    fn cycles(&self) -> usize {
        1
    }

    fn splice_len(&self, arc: &Arc) -> usize {
        let _ = arc;
        1
    }
}

macro_rules! assert_obj_safe {
    ($($xs:path),+ $(,)?) => {
        $(const _: Option<&dyn $xs> = None;)+
    };
}

assert_obj_safe!(Pattern<Output = u32>);

pub trait PatternExt: Pattern + Sized {
    fn euc<L, R>(self, lhs: L, rhs: R) -> Euclid<Self, L, R, Ident<usize>> {
        Euclid {
            pattern: self,
            lhs,
            rhs,
            offset: Ident::new(0),
        }
    }

    fn eucs<L, R, Off>(self, lhs: L, rhs: R, offset: Off) -> Euclid<Self, L, R, Off> {
        Euclid {
            pattern: self,
            lhs,
            rhs,
            offset,
        }
    }

    fn degrade(self) -> Degrade<Self> {
        Degrade(self)
    }

    fn repeat<A>(self, amount: A) -> Repeat<Self, A> {
        Repeat {
            pattern: self,
            amount,
        }
    }

    fn repl<A>(self, amount: A) -> Replicate<Self, A> {
        Replicate {
            pattern: self,
            amount,
        }
    }

    fn slow<A>(self, amount: A) -> Slow<Self, A> {
        Slow {
            pattern: self,
            amount,
        }
    }

    fn hold<A>(self, amount: A) -> Hold<Self, A> {
        Hold {
            pattern: self,
            amount,
        }
    }

    fn polym<A>(self, amount: A) -> Polym<Self, A> {
        Polym {
            pattern: self,
            amount,
        }
    }

    fn ap<F: FnOnce(Self) -> U, U>(self, f: F) -> U {
        f(self)
    }
}

impl<T: Pattern> PatternExt for T {}

pub trait StreamT {
    type Output;

    fn emit(&mut self, arc: Arc, scale_ttl: usize, value: Self::Output);
    /*
    fn start(&self) -> Beat;
    fn end(&self) -> Beat;
    fn arc(&self) -> Arc {
        (self.start()..self.end()).into()
    }
    */
}

assert_obj_safe!(StreamT<Output = u32>);

#[derive(Clone, Debug, Default)]
pub struct Stream<T> {
    values: Slab<T>,
    times: PriorityQueue<(usize, Beat), Reverse<Beat>>,
    last: Beat,
}

impl<T: Clone> Stream<T> {
    pub fn drain(mut self) -> Vec<(Arc, T)> {
        let mut events = vec![];
        while let Some(((idx, end), start)) = self.times.pop() {
            let arc = (start.0..end).into();
            let value = self.values[idx].clone();
            events.push((arc, value));
        }
        events
    }
}

impl<T> StreamT for Stream<T> {
    type Output = T;

    fn emit(&mut self, arc: Arc, _scale_ttl: usize, value: Self::Output) {
        let id = self.values.insert(value);
        self.times.push((id, arc.end), Reverse(arc.start));
        self.last = self.last.max(arc.end);
    }

    /*
    fn start(&self) -> Beat {
        self.times.peek().map(|(_, b)| b.0).unwrap_or(Beat(0, 1))
    }

    fn end(&self) -> Beat {
        self.last
    }
    */
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Event<T> {
    Start(T),
    Rest,
}

#[derive(Clone, Copy, Debug)]
pub struct Rest<T>(core::marker::PhantomData<T>);

impl<T> Default for Rest<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Rest<T> {
    pub const fn new() -> Self {
        Self(core::marker::PhantomData)
    }
}

impl<T> Pattern for Rest<T> {
    type Output = T;

    fn emit(&self, _arc: &Arc, _stream: &mut dyn StreamT<Output = Self::Output>) {
        // nothing to do
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ident<T>(pub T);

impl<T> Ident<T> {
    pub const fn new(v: T) -> Self {
        Self(v)
    }
}

impl<T: Clone> Pattern for Ident<T> {
    type Output = T;

    fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
        for cycle in arc.cycles() {
            let start = cycle.start;
            if !start.is_whole() {
                continue;
            }

            // always occupy 1 cycle
            let end = start + 1;
            let arc = (start..end).into();
            stream.emit(arc, 0, self.0.clone());
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Euclid<T, L, R, Off> {
    pattern: T,
    lhs: L,
    rhs: R,
    offset: Off,
}

impl<T, L, R, Off> Pattern for Euclid<T, L, R, Off>
where
    T: Pattern,
{
    type Output = T::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Degrade<T>(T);

impl<T> Pattern for Degrade<T>
where
    T: Pattern,
{
    type Output = T::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Repeat<T, A> {
    pattern: T,
    amount: A,
}

impl<T, A> Pattern for Repeat<T, A>
where
    T: Pattern,
    A: Pattern<Output = usize>,
{
    type Output = T::Output;

    fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
        for arc in arc.cycles() {
            let mut amount_stream = FirstValueStream::default();
            self.amount.emit(&arc, &mut amount_stream);
            let len = amount_stream.unwrap_or(1);

            let mut group_stream = GroupStream::new(stream, len, arc.start);

            for _ in 0..len {
                self.pattern.emit(&arc, &mut group_stream);
                group_stream.advance();
            }
        }
    }

    fn cycles(&self) -> usize {
        lcm(self.pattern.cycles(), self.amount.cycles())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Replicate<T, A> {
    pattern: T,
    amount: A,
}

impl<T, A> Pattern for Replicate<T, A>
where
    T: Pattern,
    A: Pattern<Output = usize>,
{
    type Output = T::Output;

    fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
        for arc in arc.cycles() {
            let len = self.splice_len(&arc) as u64;

            for offset in 0..len {
                let mut alt_stream = AltStream::new(stream, offset);
                self.pattern.emit(&arc, &mut alt_stream);
            }
        }
    }

    fn cycles(&self) -> usize {
        lcm(self.pattern.cycles(), self.amount.cycles())
    }

    fn splice_len(&self, arc: &Arc) -> usize {
        let mut stream = FirstValueStream::default();
        self.amount.emit(arc, &mut stream);
        stream.value.unwrap_or(1)
    }
}

#[derive(Default)]
struct FirstValueStream<T> {
    value: Option<T>,
}

impl<T> ops::Deref for FirstValueStream<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> StreamT for FirstValueStream<T> {
    type Output = T;

    fn emit(&mut self, _arc: Arc, _scale_ttl: usize, value: Self::Output) {
        if self.value.is_none() {
            self.value = Some(value);
        }
    }
}

#[derive(Default)]
struct LastValueStream<T> {
    value: Option<T>,
}

impl<T> ops::Deref for LastValueStream<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> StreamT for LastValueStream<T> {
    type Output = T;

    fn emit(&mut self, _arc: Arc, _scale_ttl: usize, value: Self::Output) {
        self.value = Some(value);
    }
}

#[derive(Default)]
struct TotalValueStream<T> {
    value: T,
}

impl<T> ops::Deref for TotalValueStream<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: ops::AddAssign> StreamT for TotalValueStream<T> {
    type Output = T;

    fn emit(&mut self, _arc: Arc, _scale_ttl: usize, value: Self::Output) {
        self.value += value;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Slow<T, A> {
    pattern: T,
    amount: A,
}

impl<T, A> Slow<T, A>
where
    T: Pattern,
    A: Pattern<Output = usize>,
{
    fn for_each<F: FnMut(Arc, usize)>(&self, arc: &Arc, mut f: F) {
        dbg!(self.cycles());
        let amount_cycles = self.amount.cycles() as u64;

        for arc in arc.cycles() {
            let start = Beat(arc.start.0 % amount_cycles, 1);
            let end = start + 1;
            let subarc: Arc = (start..end).into();

            let mut amount_stream = FirstValueStream::default();
            self.amount.emit(&subarc, &mut amount_stream);
            let amount = amount_stream.unwrap_or(1);

            f(arc, amount);
        }
    }
}

impl<T, A> Pattern for Slow<T, A>
where
    T: Pattern,
    A: Pattern<Output = usize>,
{
    type Output = T::Output;

    fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
        self.for_each(arc, |arc, _amount| {
            self.pattern.emit(&arc, stream);
            // let slow_stream = SlowStream::new(stream, amount as _);
            todo!()
        });
    }

    fn cycles(&self) -> usize {
        let amount_cycles = self.amount.cycles();
        let cycles = lcm(self.pattern.cycles(), amount_cycles);

        let mut total_stream = TotalValueStream::default();
        let arc = (Beat(0, 1)..Beat(amount_cycles as _, 1)).into();
        self.amount.emit(&arc, &mut total_stream);
        let total = *total_stream;

        lcm(total, cycles)
    }
}

struct SlowStream<'a, Output> {
    stream: &'a mut dyn StreamT<Output = Output>,
    amount: u64,
}

impl<'a, Output> SlowStream<'a, Output> {
    #[allow(dead_code)]
    fn new(stream: &'a mut dyn StreamT<Output = Output>, amount: u64) -> Self {
        Self { stream, amount }
    }
}

impl<'a, Output> StreamT for SlowStream<'a, Output> {
    type Output = Output;

    fn emit(&mut self, arc: Arc, scale_ttl: usize, value: Self::Output) {
        let start = arc.start * self.amount;

        let mut end = arc.end;
        if scale_ttl == 0 {
            end *= self.amount;
        }

        let arc = (start..end).into();
        self.stream.emit(arc, scale_ttl.saturating_sub(1), value);
    }

    /*

    fn start(&self) -> Beat {
        self.stream.start()
    }

    fn end(&self) -> Beat {
        self.stream.end()
    }
    */
}

#[derive(Clone, Copy, Debug)]
pub struct Hold<T, A> {
    pattern: T,
    amount: A,
}

impl<T, A> Pattern for Hold<T, A>
where
    T: Pattern,
{
    type Output = T::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Polym<T, A> {
    pattern: T,
    amount: A,
}

impl<T, A> Pattern for Polym<T, A>
where
    T: Pattern,
{
    type Output = T::Output;
}

#[derive(Clone, Copy, Debug)]
pub struct Group<T>(T);

impl<T> Group<T> {
    pub const fn new(v: T) -> Self {
        Self(v)
    }
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

macro_rules! impl_group {
    ([$($t:ident),*],[$($value:tt),*]) => {
        impl<$($t),*, Output> Pattern for Group<($($t,)*)>
        where
            $(
                $t: Pattern<Output = Output>,
            )*
        {
            type Output = Output;

            fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
                for arc in arc.cycles() {
                    let lengths = (
                        $(
                            (self.0).$value.splice_len(&arc),
                        )*
                    );
                    let total = 0
                        $(
                            + lengths.$value
                        )*;

                    let mut group_stream = GroupStream::new(stream, total, arc.start);
                    $(
                        (self.0).$value.emit(&arc, &mut group_stream);
                        group_stream.advance();
                    )*
                }
            }

            fn cycles(&self) -> usize {
                lcm_slice(&[
                    $(
                        (self.0).$value.cycles(),
                    )*
                ]).unwrap_or(1)
            }
        }
    };
}

impl_tuple!(impl_group);

struct GroupStream<'a, Output> {
    stream: &'a mut dyn StreamT<Output = Output>,
    total: usize,
    offset: usize,
    start: Beat,
}

impl<'a, Output> GroupStream<'a, Output> {
    fn new(stream: &'a mut dyn StreamT<Output = Output>, total: usize, start: Beat) -> Self {
        Self {
            stream,
            total,
            offset: 0,
            start,
        }
    }

    fn scale(&self) -> (u64, u64) {
        (1, self.total as _)
    }

    fn offset(&self) -> Beat {
        Beat(self.offset as _, self.total as _)
    }

    fn advance(&mut self) {
        self.offset += 1;
    }
}

impl<'a, Output> StreamT for GroupStream<'a, Output> {
    type Output = Output;

    fn emit(&mut self, arc: Arc, scale_ttl: usize, value: Self::Output) {
        let offset = self.offset() + self.start;
        let scale = self.scale();

        let start = (arc.start - self.start) * scale + offset;

        let mut end = arc.end - self.start;
        if scale_ttl == 0 {
            end *= scale
        }
        end += offset;

        let arc = (start..end).into();
        self.stream.emit(arc, scale_ttl.saturating_sub(1), value);
    }

    /*

    fn start(&self) -> Beat {
        self.stream.start()
    }

    fn end(&self) -> Beat {
        self.stream.end()
    }
    */
}

#[derive(Clone, Copy, Debug)]
pub struct Alternate<T>(T);

impl<T> Alternate<T> {
    pub const fn new(v: T) -> Self {
        Self(v)
    }
}

macro_rules! impl_alt {
    ([$($t:ident),*],[$($value:tt),*]) => {
        impl<$($t),*, Output> Pattern for Alternate<($($t,)*)>
        where
            $(
                $t: Pattern<Output = Output>,
            )*
        {
            type Output = Output;

            fn emit(&self, arc: &Arc, stream: &mut dyn StreamT<Output = Self::Output>) {
                for arc in arc.cycles() {
                    let lengths = (
                        $(
                            (self.0).$value.splice_len(&arc) as u64,
                        )*
                    );
                    let total = 0
                        $(
                            + lengths.$value
                        )*;

                    if total == 0 {
                        continue;
                    }

                    let start = arc.start.truncate().whole();
                    let mut idx = start % total;

                    $(
                        if let Some(n) = idx.checked_sub(lengths.$value) {
                            idx = n;
                        } else {
                            let new_start = start / total;
                            let diff = start - new_start;
                            let mut alt_stream = AltStream::new(stream, diff);
                            let start = Beat(new_start, 1);
                            let end = start + 1;
                            let arc = (start..end).into();
                            (self.0).$value.emit(&arc, &mut alt_stream);
                            continue;
                        }
                    )*

                    let _ = idx;
                }
            }

            fn cycles(&self) -> usize {
                let cycles = lcm_slice(&[
                    $(
                        (self.0).$value.cycles(),
                    )*
                ]).unwrap_or(1);

                let mut total = 0;

                for cycle in 0..cycles {
                    let arc = Arc::from_cycle(cycle);
                    $(
                        total += (self.0).$value.splice_len(&arc);
                    )*
                }

                lcm(total, cycles)
            }
        }
    };
}

struct AltStream<'a, Output> {
    stream: &'a mut dyn StreamT<Output = Output>,
    offset: u64,
}

impl<'a, Output> AltStream<'a, Output> {
    fn new(stream: &'a mut dyn StreamT<Output = Output>, offset: u64) -> Self {
        Self { stream, offset }
    }

    fn offset(&self) -> Beat {
        Beat(self.offset, 1)
    }
}

impl<'a, Output> StreamT for AltStream<'a, Output> {
    type Output = Output;

    fn emit(&mut self, mut arc: Arc, scale_ttl: usize, value: Self::Output) {
        let offset = self.offset();
        arc.start += offset;
        arc.end += offset;
        self.stream.emit(arc, scale_ttl, value);
    }

    /*

    fn start(&self) -> Beat {
        self.stream.start()
    }

    fn end(&self) -> Beat {
        self.stream.end()
    }
    */
}

impl_tuple!(impl_alt);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! pt {
        ($($t:tt)*) => {{
            let f = $crate::p!($($t)*);
            let mut stream = Stream::default();
            let cycles = f.cycles() as u64;
            let arc = (Beat(0, 1)..Beat(cycles, 1)).into();
            f.emit(&arc, &mut stream);
            let value = format!("{:#?}", stream.drain());
            insta::assert_snapshot!(
                insta::_macro_support::AutoName,
                value,
                stringify!($($t)*)
            );
            f
        }};
    }

    #[allow(non_upper_case_globals)]
    const bd: &str = "bd";
    #[allow(non_upper_case_globals)]
    const sd: &str = "sd";
    #[allow(non_upper_case_globals)]
    const hh: &str = "hh";
    #[allow(non_upper_case_globals)]
    const cp: &str = "cp";

    #[test]
    fn grouping() {
        // "[bd sd] hh"
        pt![[bd, sd], hh];
    }

    #[test]
    fn replicate() {
        // "hh bd!2"
        pt![hh, bd.repl(2)];
    }

    #[test]
    fn replicate_alternate() {
        // "hh bd!<2 3>"
        pt![hh, bd.repl((2, 3))];
    }

    #[test]
    fn alternate() {
        // "bd <sd hh cp>"
        pt![bd, (sd, hh, cp)];
    }

    #[test]
    fn alternate_nested() {
        // "<sd <hh <cp bd>>>"
        pt![(sd, (hh, (cp, bd)))];
    }

    #[test]
    fn repeat() {
        // "bd*2 sd"
        pt![bd * 2, sd];
        pt![bd * 2];
    }

    #[test]
    fn repeat_alternate() {
        // "bd*<2 3> sd"
        pt![bd * (2, 3), sd];
    }

    #[test]
    #[ignore] // TODO
    fn slow() {
        // "bd/2"
        pt![bd / 2, sd];
        pt![bd / 2];
    }

    #[test]
    #[ignore] // TODO
    fn slow_alternate() {
        // "bd/2"
        pt![bd / (1, 2), sd];
    }

    #[test]
    fn examples() {
        /*
        // shorthand
        // "bd sd . hh hh hh"
        // p![bd sd | hh hh hh];

        // slow down a pattern
        // "bd/2"
        pt![bd / 2, sd];
        pt![bd / 2];

        // rest
        // "bd ~ sd"
        //pt![bd, _];
        //pt![bd, _, sd];

        // elongate
        // "bd@3 bd"
        pt![bd + 3];
        pt![bd + 3, bd];

        // degrade
        // "bd? sd"
        pt![bd?];
        pt![bd?, sd];

        // euclidean sequences
        // "bd(3,8)"
        pt![bd.euc(3, 8)];
        pt![bd.eucs(3, 8, 1)];
        pt![bd.eucs([3, 4], 8 * 3, 1)];

        // polymetric sequences
        // "{bd bd bd bd, cp cp hh}"
        // pt![bd, bd, bd, bd & cp, cp, hh];

        // subdivision
        // "{bd cp hh}%8"
        pt![[bd, cp, hh] % 8];

        // other
        let _: Rest<()> = pt![];
        pt![1];
        pt![1,];
        pt![1, 2];
        pt![1, [2, 3]];

        let test = Ident::new(4);
        pt![&test];

        // escape
        pt![{ p![1] }];

        // chaining
        pt![1.ap(|f| f.repeat(1)),];
        */
    }
}
