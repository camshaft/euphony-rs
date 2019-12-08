```rust
async fn start(config: &mut Config, time: &Time) {
    // Setup state from config

    instrument.emit(Event {..});
    time.rest_for(Beat(1, 4)).await;
    emit(Event {..});
}
```

```rust
async fn tonic_progression<Instrument: EventContext>(instrument: &mut Instrument, tonic: Tonic) {
    let progression = [
        (tonic +  I, Measure(2, 1)),
        (tonic + IV, Measure(1, 1)),
        (tonic +  V, Measure(1, 1)),
    ];

    for (tonic, period) in progression.iter().cycle() {
        instrument.set(tonic);
        period.await;
    }
}
```
