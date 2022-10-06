#[cfg(any(test, feature = "rayon"))]
pub use rayon::prelude::*;

#[cfg(not(any(test, feature = "rayon")))]
mod polyfill {
    use std::collections::{btree_map, btree_set, hash_map, BTreeMap, BTreeSet, HashMap};

    pub trait ParIter<'a> {
        type ParIter;
        fn par_iter(&'a self) -> Self::ParIter;
    }

    pub trait ParIterMut<'a> {
        type ParIterMut;
        fn par_iter_mut(&'a mut self) -> Self::ParIterMut;
    }

    pub trait ParExtend<Item> {
        fn par_extend<I: IntoIterator<Item = Item>>(&mut self, iter: I);
    }

    impl<Item, T: core::iter::Extend<Item>> ParExtend<Item> for T {
        fn par_extend<I: IntoIterator<Item = Item>>(&mut self, iter: I) {
            self.extend(iter)
        }
    }

    impl<'a, T: 'a> ParIter<'a> for BTreeSet<T> {
        type ParIter = btree_set::Iter<'a, T>;

        fn par_iter(&'a self) -> Self::ParIter {
            self.iter()
        }
    }

    impl<'a, K: 'a, V: 'a> ParIterMut<'a> for BTreeMap<K, V> {
        type ParIterMut = btree_map::IterMut<'a, K, V>;

        fn par_iter_mut(&'a mut self) -> Self::ParIterMut {
            self.iter_mut()
        }
    }

    impl<'a, K: 'a, V: 'a> ParIter<'a> for HashMap<K, V> {
        type ParIter = hash_map::Iter<'a, K, V>;

        fn par_iter(&'a self) -> Self::ParIter {
            self.iter()
        }
    }

    impl<'a, K: 'a, V: 'a> ParIterMut<'a> for HashMap<K, V> {
        type ParIterMut = hash_map::IterMut<'a, K, V>;

        fn par_iter_mut(&'a mut self) -> Self::ParIterMut {
            self.iter_mut()
        }
    }
}

#[cfg(not(any(test, feature = "rayon")))]
pub use polyfill::*;
