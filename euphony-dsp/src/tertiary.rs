use crate::prelude::*;

macro_rules! tertiary {
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, | $a:ident, $b:ident, $c:ident | $value:expr) => {
        #[derive(Debug, Clone, Copy, Default, Node)]
        #[node(id = $id, module = "tertiary")]
        #[input($a)]
        #[input($b)]
        #[input($c)]
        $(#[doc = $doc])*
        pub struct $name;

        impl $name {
            fn render(&mut self, a: Input, b: Input, c: Input, output: &mut [Sample]) {
                match (a, b, c) {
                    (Input::Constant($a), Input::Constant($b), Input::Constant($c)) => {
                        let v = $value;
                        for sample in output.iter_mut() {
                            *sample = v;
                        }
                    }
                    (Input::Constant($a), Input::Constant($b), Input::Buffer(c)) => {
                        for (sample, $c) in output.iter_mut().zip(c.iter().copied()) {
                            *sample = $value;
                        }
                    }
                    (Input::Constant($a), Input::Buffer(b), Input::Constant($c)) => {
                        for (sample, $b) in output.iter_mut().zip(b.iter().copied()) {
                            *sample = $value;
                        }
                    }
                    (Input::Constant($a), Input::Buffer(b), Input::Buffer(c)) => {
                        for (sample, ($b, $c)) in output.iter_mut().zip(b.iter().copied().zip(c.iter().copied())) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Constant($b), Input::Constant($c)) => {
                        for (sample, $a) in output.iter_mut().zip(a.iter().copied()) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Constant($b), Input::Buffer(c)) => {
                        for (sample, ($a, $c)) in output.iter_mut().zip(a.iter().copied().zip(c.iter().copied())) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Buffer(b), Input::Constant($c)) => {
                        for (sample, ($a, $b)) in output.iter_mut().zip(a.iter().copied().zip(b.iter().copied())) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Buffer(b), Input::Buffer(c)) => {
                        unsafe {
                            unsafe_assert!(a.len() == b.len());
                            unsafe_assert!(a.len() == c.len());
                        }

                        let a = a.iter().copied();
                        let b = b.iter().copied();
                        let c = c.iter().copied();

                        for (sample, ($a, ($b, $c))) in output.iter_mut().zip(a.zip(b.zip(c))) {
                            *sample = $value;
                        }
                    }
                }
            }
        }
    };
}

tertiary!(
    /// Fused multiply-add. Computes `(input + add) * mul` with only one rounding
    /// error, yielding a more accurate result than an unfused add-multiply.
    75,
    AddMul,
    |input, add, mul| (input + add) * mul
);
tertiary!(
    /// Restrict a value to a certain interval unless it is NaN.
    ///
    /// Returns `max` if `input` is greater than `max`, and `min` if `input` is
    /// less than `min`. Otherwise this returns `input`.
    ///
    /// Note that this function returns NaN if the initial value was NaN as
    /// well or `min > max`
    76,
    Clamp,
    |input, min, max| {
        if min <= max {
            input.clamp(min, max)
        } else {
            f64::NAN
        }
    }
);
tertiary!(
    /// Fused multiply-add. Computes `(input * mul) + add` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    77,
    MulAdd,
    |input, mul, add| input.mul_add(mul, add)
);
tertiary!(
    /// If `cond` is positive, then `positive` is returned. Otherwise `negative`
    /// is returned.
    78,
    Select,
    |cond, positive, negative| if cond.is_sign_positive() {
        positive
    } else {
        negative
    }
);
