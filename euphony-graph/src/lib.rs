// #![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet, VecDeque},
    vec,
    vec::Vec,
};
use core::{cell::UnsafeCell, fmt, ops};
use slotmap::SlotMap;

slotmap::new_key_type! { struct Key; }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error<Parameter> {
    MissingNode(u64),
    InvalidParameter(u64, Parameter),
    CycleDetected,
}

type NodeMap<C> = SlotMap<Key, Node<C>>;

pub trait Config: 'static {
    type Output: 'static + Send;
    type Parameter: 'static + Send;
    type Value: 'static + Send;
    type Context: 'static + Send + Sync;
}

pub trait Processor<C: Config>: 'static + Send {
    fn set(
        &mut self,
        parameter: C::Parameter,
        key: Input<C::Value>,
    ) -> Result<Input<C::Value>, C::Parameter>;

    fn remove(&mut self, key: NodeKey);

    fn output(&self) -> &C::Output;

    fn output_mut(&mut self) -> &mut C::Output;

    fn process(&mut self, inputs: Inputs<C>, context: &C::Context);

    fn fork(&self) -> Option<Box<dyn Processor<C>>>;
}

#[derive(Debug)]
pub struct Graph<C: Config> {
    nodes: NodeMap<C>,
    ids: BTreeMap<u64, Key>,
    levels: Vec<BTreeSet<Key>>,
    dirty: BTreeMap<Key, DirtyState>,
    stack: VecDeque<Key>,
}

impl<C: Config> Default for Graph<C> {
    #[inline]
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            ids: Default::default(),
            levels: vec![Default::default()],
            dirty: Default::default(),
            stack: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum DirtyState {
    Initial,
    Pending,
    Done(u16),
}

impl Default for DirtyState {
    #[inline]
    fn default() -> Self {
        Self::Initial
    }
}

impl<C: Config> Graph<C> {
    #[inline]
    pub fn process(&mut self, context: &C::Context) {
        debug_assert!(
            self.dirty.is_empty(),
            "need to call `update` before `process`"
        );

        for level in &self.levels {
            let nodes = &self.nodes;

            #[cfg(any(test, feature = "rayon"))]
            {
                use rayon::prelude::*;
                level.par_iter().for_each(|key| {
                    nodes[*key].render(nodes, context);
                });
            }

            #[cfg(not(any(test, feature = "rayon")))]
            {
                level.iter().for_each(|key| {
                    nodes[*key].render(nodes, context);
                });
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, id: u64, processor: Box<dyn Processor<C>>) {
        let node = Node::new(id, processor);
        let key = self.nodes.insert(node);
        self.ids.insert(id, key);
        self.levels[0].insert(key);

        self.ensure_consistency();
    }

    #[inline]
    pub fn set(
        &mut self,
        target: u64,
        param: C::Parameter,
        value: C::Value,
    ) -> Result<(), Error<C::Parameter>> {
        let idx = *self.ids.get(&target).ok_or(Error::MissingNode(target))?;

        let node = unsafe { self.nodes.get_unchecked_mut(idx) };

        let prev = node
            .set(param, Input::Value(value))
            .map_err(|param| Error::InvalidParameter(target, param))?;

        if let Input::Node(prev) = prev {
            // if we went from a node input to a constant, we need to recalc
            self.dirty.insert(idx, Default::default());
            node.parents.remove(prev.0);

            // tell the parent we are no longer a child
            let prev = unsafe { self.nodes.get_unchecked_mut(prev.0) };
            prev.children.remove(idx);
        }

        self.ensure_consistency();

        Ok(())
    }

    #[inline]
    pub fn connect(
        &mut self,
        target: u64,
        param: C::Parameter,
        source: u64,
    ) -> Result<(), Error<C::Parameter>> {
        if target == source {
            return Err(Error::CycleDetected);
        }

        let idx = *self.ids.get(&target).ok_or(Error::MissingNode(target))?;

        let source_key = *self.ids.get(&source).ok_or(Error::MissingNode(source))?;
        let source = unsafe { self.nodes.get_unchecked_mut(source_key) };
        source.children.insert(idx);
        let source_level = source.level;

        let node = unsafe { self.nodes.get_unchecked_mut(idx) };
        let prev = node
            .set(param, Input::Node(NodeKey(source_key)))
            .map_err(|param| Error::InvalidParameter(target, param))?;
        node.parents.insert(source_key);

        if let Input::Node(prev) = prev {
            node.parents.remove(prev.0);

            let prev = unsafe { self.nodes.get_unchecked_mut(prev.0) };
            prev.children.remove(idx);
            let prev_level = prev.level;

            // the node is only dirty if the levels have changed
            if source_level != prev_level {
                self.dirty.insert(idx, Default::default());
            }
        } else {
            // going from a constant to a node will require recalc
            self.dirty.insert(idx, Default::default());
        }

        self.ensure_consistency();

        Ok(())
    }

    #[inline]
    pub fn remove(&mut self, id: u64) -> Result<Box<dyn Processor<C>>, Error<C::Parameter>> {
        let key = self.ids.remove(&id).ok_or(Error::MissingNode(id))?;
        let node = self.nodes.remove(key).unwrap();

        // the node is no longer part of the levels
        self.levels[node.level as usize].remove(&key);
        self.dirty.remove(&key);

        // notify our children that we're finished
        for child_key in node.children.iter() {
            let child = unsafe { self.nodes.get_unchecked_mut(child_key) };
            child.clear_parent(key);

            // if the child's level matches the node's, it needs to be recalculated
            if child.level == node.level + 1 {
                self.dirty.insert(child_key, Default::default());
            }
        }

        // notify our parents that we're finished
        for parent_key in node.parents.iter() {
            let parent = unsafe { self.nodes.get_unchecked_mut(parent_key) };
            parent.children.clear(key);
        }

        self.ensure_consistency();

        Ok(node.processor.into_inner())
    }

    #[inline]
    pub fn get_node(&self, id: u64) -> Result<&dyn Processor<C>, Error<C::Parameter>> {
        let key = self.ids.get(&id).ok_or(Error::MissingNode(id))?;
        let node = unsafe { self.nodes.get_unchecked(*key) };
        let out = unsafe { (*node.processor.get()).as_ref() };
        Ok(out)
    }

    #[inline]
    pub fn get(&self, id: u64) -> Result<&C::Output, Error<C::Parameter>> {
        let key = self.ids.get(&id).ok_or(Error::MissingNode(id))?;
        let node = unsafe { self.nodes.get_unchecked(*key) };
        let output = node.output();
        Ok(output)
    }

    #[inline]
    pub fn get_mut(&mut self, id: u64) -> Result<&mut C::Output, Error<C::Parameter>> {
        let key = self.ids.get(&id).ok_or(Error::MissingNode(id))?;
        let node = unsafe { self.nodes.get_unchecked_mut(*key) };
        let output = node.output_mut();
        Ok(output)
    }

    #[inline]
    pub fn update(&mut self) -> Result<(), Error<C::Parameter>> {
        if self.dirty.is_empty() {
            return Ok(());
        }

        // queue up all of the updates
        self.stack.extend(self.dirty.keys().copied());

        while let Some(key) = self.stack.pop_front() {
            let node = unsafe { self.nodes.get_unchecked(key) };
            let mut was_repushed = false;
            let mut new_level = 0u16;

            for parent in node.parents.iter() {
                if let Some(parent_state) = self.dirty.get(&parent).copied() {
                    match parent_state {
                        DirtyState::Initial => {
                            if !core::mem::replace(&mut was_repushed, true) {
                                self.dirty.insert(key, DirtyState::Pending);
                                self.stack.push_front(key);
                            }

                            self.stack.push_front(parent);
                        }
                        DirtyState::Pending => {
                            return Err(Error::CycleDetected);
                        }
                        DirtyState::Done(parent_level) => {
                            new_level = new_level.max(parent_level + 1);
                        }
                    }
                } else if !was_repushed {
                    let parent = unsafe { self.nodes.get_unchecked(parent) };
                    new_level = new_level.max(parent.level + 1);
                }
            }

            if was_repushed {
                continue;
            }

            if let Some(DirtyState::Done(prev_level)) =
                self.dirty.insert(key, DirtyState::Done(new_level))
            {
                if prev_level != new_level {
                    return Err(Error::CycleDetected);
                }

                continue;
            }

            if node.level == new_level {
                continue;
            }

            self.levels[node.level as usize].remove(&key);

            // the children need to be updated now
            for child in node.children.iter() {
                self.stack.push_back(child);
            }

            let node = unsafe { self.nodes.get_unchecked_mut(key) };
            node.level = new_level;

            let new_level = new_level as usize;
            if self.levels.len() <= new_level {
                self.levels.resize_with(new_level + 1, Default::default);
            }
            self.levels[new_level].insert(key);
        }

        self.dirty.clear();

        self.ensure_consistency();

        Ok(())
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn ensure_consistency(&self) {}

    #[inline]
    #[cfg(debug_assertions)]
    fn ensure_consistency(&self) {
        // ensure the ids aren't referencing a freed node
        for (id, key) in self.ids.iter() {
            let node = self.nodes.get(*key).unwrap();
            assert_eq!(*id, node.id);
        }

        // ensure the nodes match the expected id
        for (key, node) in self.nodes.iter() {
            let actual = *self.ids.get(&node.id).unwrap();
            assert_eq!(actual, key);
        }

        // ensure the levels don't have freed nodes
        for level in &self.levels {
            for key in level {
                assert!(self.nodes.contains_key(*key));
            }
        }

        for key in self.nodes.keys() {
            let node = &self.nodes[key];

            for child_key in node.children.iter() {
                let child = &self.nodes[child_key];
                assert!(child.parents.0.contains_key(&key));
            }

            for parent_key in node.parents.iter() {
                let parent = &self.nodes[parent_key];
                assert!(parent.children.0.contains_key(&key));
            }

            assert!(self.levels[node.level as usize].contains(&key));

            // the following checks require the node to be clean
            if self.dirty.contains_key(&key) {
                continue;
            }

            let mut expected = 0;

            for parent in node.parents.iter() {
                let parent = self.nodes[parent].level;
                expected = expected.max(parent + 1);
            }

            assert_eq!(node.level, expected, "level mismatch");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeKey(Key);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input<Value> {
    Value(Value),
    Node(NodeKey),
}

pub struct Inputs<'a, C: Config> {
    nodes: &'a NodeMap<C>,
    #[cfg(debug_assertions)]
    parents: &'a Relationship,
}

impl<'a, C: Config> ops::Index<NodeKey> for Inputs<'a, C> {
    type Output = C::Output;

    #[inline]
    fn index(&self, key: NodeKey) -> &Self::Output {
        debug_assert!(self.nodes.contains_key(key.0));

        #[cfg(debug_assertions)]
        {
            assert!(
                self.parents.0.contains_key(&key.0),
                "node should only access its configured parents"
            );
        }

        unsafe { self.nodes.get_unchecked(key.0).output() }
    }
}

struct Node<C: Config> {
    #[cfg(debug_assertions)]
    id: u64,
    processor: UnsafeCell<Box<dyn Processor<C>>>,
    level: u16,
    parents: Relationship,
    children: Relationship,
}

impl<C: Config> fmt::Debug for Node<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Node");

        #[cfg(debug_assertions)]
        s.field("id", &self.id);

        s.field("level", &self.level)
            .field("parents", &self.parents)
            .field("children", &self.children)
            .finish()
    }
}

/// Safety: Mutual exclusion is ensured by level organization
unsafe impl<C: Config> Sync for Node<C> {}

impl<C: Config> Node<C> {
    #[inline]
    fn new(id: u64, processor: Box<dyn Processor<C>>) -> Self {
        let _ = id;
        Self {
            #[cfg(debug_assertions)]
            id,
            processor: UnsafeCell::new(processor),
            level: 0,
            parents: Default::default(),
            children: Default::default(),
        }
    }

    #[inline]
    fn set(
        &mut self,
        param: C::Parameter,
        value: Input<C::Value>,
    ) -> Result<Input<C::Value>, C::Parameter> {
        let processor = unsafe { &mut *self.processor.get() };
        processor.set(param, value)
    }

    #[inline]
    fn clear_parent(&mut self, key: Key) {
        let processor = unsafe { &mut *self.processor.get() };
        processor.remove(NodeKey(key));
        self.parents.clear(key);
    }

    #[inline]
    fn render(&self, nodes: &NodeMap<C>, context: &C::Context) {
        let inputs = Inputs {
            nodes,
            #[cfg(debug_assertions)]
            parents: &self.parents,
        };

        let processor = unsafe { &mut *self.processor.get() };

        processor.process(inputs, context);
    }

    #[inline]
    fn output(&self) -> &C::Output {
        let processor = unsafe { &*self.processor.get() };
        processor.output()
    }

    #[inline]
    fn output_mut(&mut self) -> &mut C::Output {
        let processor = unsafe { &mut *self.processor.get() };
        processor.output_mut()
    }
}

#[derive(Clone, Debug, Default)]
struct Relationship(BTreeMap<Key, u16>);

impl Relationship {
    #[inline]
    pub fn insert(&mut self, key: Key) {
        *self.0.entry(key).or_default() += 1;
    }

    #[inline]
    pub fn remove(&mut self, key: Key) {
        let new_count = self.0.remove(&key).unwrap_or(1) - 1;
        if new_count != 0 {
            self.0.insert(key, new_count);
        }
    }

    #[inline]
    pub fn clear(&mut self, key: Key) {
        self.0.remove(&key);
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Key> + '_ {
        self.0.keys().copied()
    }
}

#[cfg(test)]
mod tests;
