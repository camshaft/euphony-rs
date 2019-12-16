// pub trait EventContext: Sized {
//     type ChildEvent: EventContext;

//     fn start<E>(&mut self, event: E) -> Self::ChildEvent
//     where
//         E: ToEvent<Self>;

//     fn stop(self);
// }

// pub trait EventParameter<V> {
//     fn with(self, value: V) -> Self;
//     fn set(&mut self, value: V);
//     fn get(&self) -> Option<V>;
// }

// pub trait ToEvent<Parent: EventContext> {
//     type Event: EventContext;

//     fn to_event(self, system: &Parent) -> Self::Event;
// }

pub trait Context<Value> {
    fn get(&self) -> Value;
}

pub trait ContextMut<Value>: Context<Value> {
    fn set(&self, value: Value) {
        self.update(|_| value)
    }

    fn update<F: FnOnce(Value) -> Value>(&self, update: F);
}

pub trait Rest<Duration> {
    fn rest(&self, value: Duration);
}
