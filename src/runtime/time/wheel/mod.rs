use crate::time::timestamp::Timestamp;
use core::time::Duration;

pub mod level;
pub(crate) use self::level::Expiration;
use self::level::Level;

pub mod stack;
pub(crate) use self::stack::Stack;

#[derive(Debug)]
pub(crate) struct Wheel<T> {
    timestamp: Timestamp,
    levels: [Level<T>; NUM_LEVELS],
}

const NUM_LEVELS: usize = 6;

/// The maximum duration of a delay
const MAX_DURATION: Duration = Duration::from_micros(1 << (level::LEVEL_EXPONENT * NUM_LEVELS));

#[derive(Debug)]
pub(crate) enum InsertError {
    Elapsed,
    Invalid,
}

/// Poll expirations from the wheel
#[derive(Debug, Default)]
pub(crate) struct Poll {
    now: Timestamp,
    expiration: Option<Expiration>,
}

impl Poll {
    pub fn new(now: Timestamp) -> Self {
        Self {
            now,
            expiration: None,
        }
    }
}

impl<T> Wheel<T>
where
    T: Stack,
{
    /// Create a new timing wheel
    pub(crate) fn new() -> Wheel<T> {
        Wheel {
            timestamp: Default::default(),
            levels: [
                Level::new(0),
                Level::new(1),
                Level::new(2),
                Level::new(3),
                Level::new(4),
                Level::new(5),
            ],
        }
    }

    /// Return the time that have elapsed since the timing
    /// wheel's creation.
    pub(crate) fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Insert an entry into the timing wheel.
    ///
    /// # Arguments
    ///
    /// * `when`: is the instant at which the entry should be fired. It is
    ///           represented as the number of milliseconds since the creation
    ///           of the timing wheel.
    ///
    /// * `item`: The item to insert into the wheel.
    ///
    /// * `store`: The slab or `()` when using heap storage.
    ///
    /// # Return
    ///
    /// Returns `Ok` when the item is successfully inserted, `Err` otherwise.
    ///
    /// `Err(Elapsed)` indicates that `when` represents an instant that has
    /// already passed. In this case, the caller should fire the timeout
    /// immediately.
    ///
    /// `Err(Invalid)` indicates an invalid `when` argument as been supplied.
    pub(crate) fn insert(
        &mut self,
        when: Timestamp,
        item: T::Value,
        store: &mut T::Store,
    ) -> Result<(), (T::Value, InsertError)> {
        if when <= self.timestamp {
            return Err((item, InsertError::Elapsed));
        } else if when - self.timestamp > MAX_DURATION {
            return Err((item, InsertError::Invalid));
        }

        // Get the level at which the entry should be stored
        let level = self.level_for(when);

        self.levels[level].add_entry(when, item, store);

        debug_assert!({
            self.levels[level]
                .next_expiration(self.timestamp)
                .map(|e| e.deadline >= self.timestamp)
                .unwrap_or(true)
        });

        Ok(())
    }

    /// Remove `item` from the timing wheel.
    pub(crate) fn remove(&mut self, item: &T::Value, store: &mut T::Store) {
        let when = T::when(item, store);
        let level = self.level_for(when);

        self.levels[level].remove_entry(when, item, store);
    }

    pub(crate) fn prepare_park(&mut self, store: &mut T::Store) -> Option<Timestamp> {
        // loop {
        //     let expiration = self.next_expiration()?;

        //     if !self.sift_up(&expiration, store) {
        //         return Some(expiration.deadline);
        //     }

        //     self.set_timestamp(expiration.deadline);
        // }

        self.next_expiration().map(|expiration| expiration.deadline)
    }

    pub(crate) fn poll(&mut self, poll: &mut Poll, store: &mut T::Store) -> Option<T::Value> {
        loop {
            if poll.expiration.is_none() {
                poll.expiration = self.next_expiration().and_then(|expiration| {
                    if expiration.deadline > poll.now {
                        None
                    } else {
                        Some(expiration)
                    }
                });
            }

            match poll.expiration.take() {
                Some(ref expiration) => {
                    if let Some(item) = self.poll_expiration(expiration, store) {
                        return Some(item);
                    }

                    self.set_timestamp(expiration.deadline);
                }
                None => {
                    self.set_timestamp(poll.now);
                    return None;
                }
            }
        }
    }

    pub(crate) fn take(&mut self, store: &mut T::Store) -> Option<T::Value> {
        let expiration = self.next_expiration()?;
        let now = expiration.deadline;
        self.poll(
            &mut Poll {
                expiration: Some(expiration),
                now,
            },
            store,
        )
    }

    /// Returns the instant at which the next timeout expires.
    fn next_expiration(&self) -> Option<Expiration> {
        self.levels
            .iter()
            .find_map(|level| level.next_expiration(self.timestamp))
    }

    pub(crate) fn poll_expiration(
        &mut self,
        expiration: &Expiration,
        store: &mut T::Store,
    ) -> Option<T::Value> {
        if !self.sift_up(expiration, store) {
            return self.pop_entry(expiration, store);
        }

        None
    }

    fn sift_up(&mut self, expiration: &Expiration, store: &mut T::Store) -> bool {
        let mut sifted = false;

        if expiration.level == 0 {
            return sifted;
        }

        while let Some(item) = self.pop_entry(expiration, store) {
            let when = T::when(&item, store);

            let next_level = expiration.level - 1;

            self.levels[next_level].add_entry(when, item, store);

            sifted = true;
        }

        sifted
    }

    fn set_timestamp(&mut self, when: Timestamp) {
        debug_assert!(
            self.timestamp <= when,
            "timestamp={:?}; when={:?}",
            self.timestamp,
            when
        );

        self.timestamp = when;
    }

    fn pop_entry(&mut self, expiration: &Expiration, store: &mut T::Store) -> Option<T::Value> {
        self.levels[expiration.level].pop_entry_slot(expiration.slot, store)
    }

    fn level_for(&self, when: Timestamp) -> usize {
        level_for(self.timestamp, when)
    }
}

fn level_for(elapsed: Timestamp, when: Timestamp) -> usize {
    let elapsed = elapsed.as_micros();
    let when = when.as_micros();
    let masked = elapsed ^ when;

    assert!(
        masked != 0,
        "masked should not be 0; elapsed={}; when={}",
        elapsed,
        when
    );

    let leading_zeros = masked.leading_zeros() as usize;
    let significant = 63 - leading_zeros;
    significant / level::LEVEL_EXPONENT
}
