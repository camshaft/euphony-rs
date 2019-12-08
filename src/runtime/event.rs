pub trait EventContext: Sized {
    type ChildEvent: EventContext;

    fn start<E>(&mut self, event: E) -> Self::ChildEvent
    where
        E: ToEvent<Self>;

    fn stop(self);
}

pub trait EventParameter<V> {
    fn with(self, value: V) -> Self;
    fn set(&mut self, value: V);
    fn get(&self) -> Option<V>;
}

pub trait ToEvent<Parent: EventContext> {
    type Event: EventContext;

    fn to_event(self, system: &Parent) -> Self::Event;
}
