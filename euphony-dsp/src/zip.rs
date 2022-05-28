pub trait Zip {
    type Iter;

    fn zip(self) -> Self::Iter;
}

pub struct ZipIter<T>(T);

macro_rules! zip {
    () => {};
    ($H:ident $(,$T:ident)* $(,)?) => {
        impl<$H: IntoIterator, $($T: IntoIterator,)*> Zip for ($H, $($T,)*) {
            type Iter = ZipIter<($H::IntoIter, $($T::IntoIter,)*)>;

            #[inline]
            #[allow(non_snake_case)]
            fn zip(self) -> Self::Iter {
                let (
                    $H,
                    $(
                        $T,
                    )*
                ) = self;
                ZipIter((
                    $H.into_iter(),
                    $(
                        $T.into_iter(),
                    )*
                ))
            }
        }

        impl<$H: Iterator, $($T: Iterator,)*> Iterator for ZipIter<($H, $($T,)*)> {
            type Item = ($H::Item, $($T::Item,)*);

            #[inline]
            #[allow(non_snake_case)]
            fn next(&mut self) -> Option<Self::Item> {
                let (
                    $H,
                    $(
                        $T,
                    )*
                ) = &mut self.0;
                Some((
                    $H.next()?,
                    $(
                        $T.next()?,
                    )*
                ))
            }
        }

        zip!($($T),*);
    };
}

zip!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
