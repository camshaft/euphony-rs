use super::stack::Stack;
use crate::time::timestamp::Timestamp;
use core::{fmt, time::Duration};

type OccupiedBits = u64;

/// Wheel for a single level in the timer. This wheel contains 64 slots.
pub(crate) struct Level<T> {
    level: usize,

    /// Bit field tracking which slots currently contain entries.
    ///
    /// Using a bit field to track slots that contain entries allows avoiding a
    /// scan to find entries. This field is updated when entries are added or
    /// removed from a slot.
    ///
    /// The least-significant bit represents slot zero.
    occupied: OccupiedBits,

    /// Slots
    slot: [T; LEVEL_MULT],
}

/// Indicates when a slot must be processed next.
#[derive(Debug)]
pub(crate) struct Expiration {
    /// The level containing the slot.
    pub(crate) level: usize,

    /// The slot index.
    pub(crate) slot: usize,

    /// The instant at which the slot needs to be processed.
    pub(crate) deadline: Timestamp,
}

/// Level multiplier.
///
/// Being a power of 2 is very important.
const LEVEL_MULT: usize = core::mem::size_of::<OccupiedBits>() * 8;

pub(crate) const LEVEL_EXPONENT: usize = 6;

impl<T: Stack> Level<T> {
    pub(crate) fn new(level: usize) -> Level<T> {
        // Rust's derived implementations for arrays require that the value
        // contained by the array be `Copy`. So, here we have to manually
        // initialize every single slot.
        macro_rules! slot {
            ($([$($t:ident),*]),*) => {
                [$($($t::default(),)*)*]
            };
        };

        Level {
            level,
            occupied: 0,
            slot: slot!(
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T],
                [T, T, T, T, T, T, T, T]
            ),
        }
    }

    /// Finds the slot that needs to be processed next and returns the slot and
    /// `Instant` at which this slot must be processed.
    pub(crate) fn next_expiration(&self, now: Timestamp) -> Option<Expiration> {
        // Use the `occupied` bit field to get the index of the next slot that
        // needs to be processed.
        let slot = self.next_occupied_slot(now)?;

        // From the slot index, calculate the `Instant` at which it needs to be
        // processed. This value *must* be in the future with respect to `now`.

        let level_range = level_range(self.level);
        let slot_range = slot_range(self.level);

        // TODO: This can probably be simplified w/ power of 2 math
        let level_start = now.as_micros() - (now.as_micros() % level_range);
        let deadline = level_start + slot as u64 * slot_range;

        let deadline = Timestamp::default() + Duration::from_micros(deadline);

        Some(Expiration {
            level: self.level,
            slot,
            deadline,
        })
    }

    fn next_occupied_slot(&self, now: Timestamp) -> Option<usize> {
        if self.occupied == 0 {
            return None;
        }

        // Get the slot for now using Maths
        let now_slot = (now.as_micros() / slot_range(self.level)) as usize;
        let occupied = self.occupied.rotate_right(now_slot as u32);
        let zeros = occupied.trailing_zeros() as usize;
        let slot = (zeros + now_slot) % 64;

        Some(slot)
    }

    pub(crate) fn add_entry(&mut self, when: Timestamp, item: T::Value, store: &mut T::Store) {
        let slot = slot_for(when, self.level);

        self.slot[slot].push(item, store);
        self.occupied |= occupied_bit(slot);
    }

    pub(crate) fn remove_entry(&mut self, when: Timestamp, item: &T::Value, store: &mut T::Store) {
        let slot = slot_for(when, self.level);

        self.slot[slot].remove(item, store);

        if self.slot[slot].is_empty() {
            // The bit is currently set
            debug_assert!(self.occupied & occupied_bit(slot) != 0);

            // Unset the bit
            self.occupied ^= occupied_bit(slot);
        }
    }

    pub(crate) fn pop_entry_slot(&mut self, slot: usize, store: &mut T::Store) -> Option<T::Value> {
        let ret = self.slot[slot].pop(store);

        if ret.is_some() && self.slot[slot].is_empty() {
            // The bit is currently set
            debug_assert!(self.occupied & occupied_bit(slot) != 0);

            self.occupied ^= occupied_bit(slot);
        }

        ret
    }
}

impl<T: fmt::Debug> fmt::Debug for Level<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Level")
            .field("occupied", &self.occupied.count_ones())
            .finish()
    }
}

fn occupied_bit(slot: usize) -> OccupiedBits {
    (1 << slot)
}

fn slot_range(level: usize) -> OccupiedBits {
    LEVEL_MULT.pow(level as u32) as OccupiedBits
}

fn level_range(level: usize) -> OccupiedBits {
    LEVEL_MULT as OccupiedBits * slot_range(level)
}

/// Convert a timestamp and a level to a slot position
fn slot_for(timestamp: Timestamp, level: usize) -> usize {
    let level = timestamp.as_micros() >> (level * LEVEL_EXPONENT);
    (level % LEVEL_MULT as OccupiedBits) as usize
}
