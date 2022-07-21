use futures::{ready, FutureExt};

use crate::{
    ext::DelayExt,
    node::Node,
    prelude::{env, Beat, DurationInput, TargetInput, ValueInput},
    processor::Processor,
    sink::Sink,
    time::Timer,
    value::{Parameter, Trigger},
};
use core::{
    future::Future,
    ops,
    pin::Pin,
    task::{Context, Poll},
};

#[macro_export]
macro_rules! envgen {
    ([$value_a:expr, $value_b:expr $(, $values:expr)* $(,)?], [$($durations:tt)*]) => {
        $crate::env::EnvGen::new(
            vec![$value_a.into(), $value_b.into(), $( $values.into() ),*],
            $crate::envgen!(@durs; []; $($durations)*),
            vec![0.into()]
        )
    };
    (@durs; [$value_a:expr, $($acc:tt)*]; ) => {
        vec![$value_a, $($acc)*]
    };
    (@durs; [$($acc:tt)*]; $num:literal / $den:literal, $($rest:tt)*) => {
        $crate::envgen!(@durs; [$($acc)* $crate::prelude::Beat($num, $den),]; $($rest)*)
    };
    (@durs; [$($acc:tt)*]; $num:literal / $den:literal) => {
        $crate::envgen!(@durs; [$($acc)* $crate::prelude::Beat($num, $den),];)
    };
    (@durs; [$($acc:tt)*]; $num:literal, $($rest:tt)*) => {
        $crate::envgen!(@durs; [$($acc)* $crate::prelude::Beat($num, 1),]; $($rest)*)
    };
    (@durs; [$($acc:tt)*]; $num:literal) => {
        $crate::envgen!(@durs; [$($acc)* $crate::prelude::Beat($num, 1),];)
    };
    (@durs; [$($acc:tt)*]; $num:expr, $($rest:tt)*) => {
        $crate::envgen!(@durs; [$($acc)* $num.into(),]; $($rest)*)
    };
    (@durs; [$($acc:tt)*]; $num:expr) => {
        $crate::envgen!(@durs; [$($acc)* $num.into(),]; )
    };
}

#[test]
fn macro_test() {
    crate::runtime::Runtime::new(0).block_on(async {
        let _ = envgen!([0, 1, 0], [1, 2]);
        let _ = envgen!([0, 1, 0], [1 / 2, 1 / 4]);
        let _ = envgen!([0, 1, 0], [Beat(1, 2), Beat(3, 4)]);
    });
}

#[must_use = "nodes do nothing unless routed to a Sink"]
pub struct EnvGen {
    values: Vec<Trigger>,
    durations: Vec<Beat>,
    curves: Vec<Parameter>,
    position: usize,
    timer: Timer,
    env: env::Linear,
}

impl EnvGen {
    pub fn new(values: Vec<Trigger>, durations: Vec<Beat>, curves: Vec<Parameter>) -> Self {
        let env = env::linear();

        let delay = durations.first().unwrap();
        env.set_duration(*delay);
        let timer = delay.delay();

        let value = &values[0];
        env.set_value(value.clone());

        let target = &values[1];
        env.set_target(target.clone());

        Self {
            values,
            durations,
            curves,
            position: 0,
            timer,
            env,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn update(&mut self) {
        let delay = self.durations[self.position % self.durations.len()];
        self.env.set_duration(delay);
        self.timer = delay.delay();

        // TODO set on env
        let curve = &self.curves[self.position % self.curves.len()];
        let _ = curve;

        let value = &self.values[self.position + 1];
        self.env.set_target(value.clone());
    }
}

impl ops::Deref for EnvGen {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        self.env.deref()
    }
}

impl Processor for EnvGen {
    fn sink(&self) -> Sink {
        self.env.sink()
    }

    fn node(&self) -> Node {
        self.env.node()
    }
}

impl From<EnvGen> for Parameter {
    fn from(env: EnvGen) -> Self {
        env.env.into()
    }
}

impl From<&EnvGen> for Parameter {
    fn from(env: &EnvGen) -> Self {
        (&env.env).into()
    }
}

impl Future for EnvGen {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            ready!(self.timer.poll_unpin(cx));

            let position = self.position + 1;

            // we're done
            if position == self.len() - 1 {
                return Poll::Ready(());
            }

            self.position = position;

            self.update();
        }
    }
}
