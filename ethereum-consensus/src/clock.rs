//! A consensus clock
use crate::{
    configs, phase0 as presets,
    primitives::{Epoch, Slot},
};
use std::{
    ops::Deref,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const MAINNET_GENESIS_TIME: u64 = 1606824023;
pub const SEPOLIA_GENESIS_TIME: u64 = 1655733600;
pub const GOERLI_GENESIS_TIME: u64 = 1616508000;
pub const HOLESKY_GENESIS_TIME: u64 = 1694786400;

fn slot_to_nanos(slot: Slot, seconds_per_slot: u128, genesis_time: u128) -> u128 {
    u128::from(slot) * seconds_per_slot + genesis_time
}

#[inline]
fn u128_to_u64(t: u128) -> u64 {
    u64::try_from(t).expect("close enough to `UNIX_EPOCH` to fit in type")
}

/// Converts `timestamp` with **second** precision to a `Slot`.
/// Returns `None` if `timestamp` is before the `genesis_time`.
pub fn convert_timestamp_to_slot(
    timestamp: u64,
    genesis_time: u64,
    seconds_per_slot: u64,
) -> Option<Slot> {
    let delta = timestamp.checked_sub(genesis_time)?;
    Some(delta / seconds_per_slot)
}

/// Converts `timestamp` with **nanosecond** precision to a `Slot`.
/// Returns `None` if `timestamp` is before the `genesis_time`.
pub fn convert_timestamp_nanos_to_slot(
    timestamp: u128,
    genesis_time: u128,
    seconds_per_slot: u128,
) -> Option<Slot> {
    let delta = timestamp.checked_sub(genesis_time)?;
    Some(u128_to_u64(delta / seconds_per_slot))
}

pub fn get_current_unix_time_in_nanos() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("after `UNIX_EPOCH`").as_nanos()
}

pub trait TimeProvider {
    // Provide the current time to **nanosecond** precision.
    fn get_current_time(&self) -> u128;
}

#[derive(Clone)]
pub struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {
    fn get_current_time(&self) -> u128 {
        get_current_unix_time_in_nanos()
    }
}

#[derive(Clone)]
pub struct Clock<T: TimeProvider + Send + Sync>(Arc<Inner<T>>);

impl<T: TimeProvider + Send + Sync> Deref for Clock<T> {
    type Target = Inner<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Inner<T: TimeProvider> {
    genesis_time: u128,
    seconds_per_slot: u128,
    slots_per_epoch: Slot,
    time_provider: T,
}

pub fn from_system_time(
    genesis_time: u64,
    seconds_per_slot: u64,
    slots_per_epoch: Slot,
) -> Clock<SystemTimeProvider> {
    let time_provider = SystemTimeProvider;
    Clock::new(genesis_time, seconds_per_slot, slots_per_epoch, time_provider)
}

pub fn for_mainnet() -> Clock<SystemTimeProvider> {
    let genesis_time = MAINNET_GENESIS_TIME;
    let seconds_per_slot = configs::mainnet::SECONDS_PER_SLOT;
    let slots_per_epoch = presets::mainnet::SLOTS_PER_EPOCH;
    from_system_time(genesis_time, seconds_per_slot, slots_per_epoch)
}

pub fn for_sepolia() -> Clock<SystemTimeProvider> {
    let genesis_time = SEPOLIA_GENESIS_TIME;
    let seconds_per_slot = configs::sepolia::SECONDS_PER_SLOT;
    let slots_per_epoch = presets::mainnet::SLOTS_PER_EPOCH;
    from_system_time(genesis_time, seconds_per_slot, slots_per_epoch)
}

pub fn for_goerli() -> Clock<SystemTimeProvider> {
    let genesis_time = GOERLI_GENESIS_TIME;
    let seconds_per_slot = configs::goerli::SECONDS_PER_SLOT;
    let slots_per_epoch = presets::mainnet::SLOTS_PER_EPOCH;
    from_system_time(genesis_time, seconds_per_slot, slots_per_epoch)
}

pub fn for_holesky() -> Clock<SystemTimeProvider> {
    let genesis_time = HOLESKY_GENESIS_TIME;
    let seconds_per_slot = configs::holesky::SECONDS_PER_SLOT;
    let slots_per_epoch = presets::mainnet::SLOTS_PER_EPOCH;
    from_system_time(genesis_time, seconds_per_slot, slots_per_epoch)
}

impl<T: TimeProvider + Send + Sync> Clock<T> {
    pub fn new(
        genesis_time: u64,
        seconds_per_slot: u64,
        slots_per_epoch: Slot,
        time_provider: T,
    ) -> Self {
        let genesis_time = Duration::from_secs(genesis_time).as_nanos();
        let seconds_per_slot = Duration::from_secs(seconds_per_slot).as_nanos();
        let inner = Inner { genesis_time, seconds_per_slot, slots_per_epoch, time_provider };
        Self(Arc::new(inner))
    }

    fn get_current_time(&self) -> u128 {
        self.time_provider.get_current_time()
    }

    pub fn before_genesis(&self) -> bool {
        self.before_genesis_at(self.get_current_time())
    }

    #[inline]
    fn before_genesis_at(&self, current_time: u128) -> bool {
        current_time < self.genesis_time
    }

    // Return the current slot, or `None` if before genesis.
    pub fn current_slot(&self) -> Option<Slot> {
        self.slot_at_time(self.get_current_time())
    }

    #[inline]
    pub fn slot_at_time(&self, current_time: u128) -> Option<Slot> {
        convert_timestamp_nanos_to_slot(current_time, self.genesis_time, self.seconds_per_slot)
    }

    // Return the current epoch, or `None` if before genesis.
    pub fn current_epoch(&self) -> Option<Epoch> {
        let current_slot = self.current_slot()?;
        Some(self.epoch_for(current_slot))
    }

    #[inline]
    pub fn epoch_for(&self, slot: Slot) -> Epoch {
        slot / self.slots_per_epoch
    }

    /// Return a `Duration` until the provided `slot` relative to the current time as determined
    /// by the clock. If `slot` is in the past, return a `Duration` of 0.
    pub fn duration_until_slot(&self, slot: Slot) -> Duration {
        let current_time = self.get_current_time();
        let target_slot_in_nanos = slot_to_nanos(slot, self.seconds_per_slot, self.genesis_time);
        target_slot_in_nanos
            .checked_sub(current_time)
            .map(|t| Duration::from_nanos(u128_to_u64(t)))
            .unwrap_or_default()
    }

    /// Return a `Duration` until the next `slot` relative to the
    /// current time as determined by the clock.
    pub fn duration_until_next_slot(&self) -> Duration {
        let current_time = self.get_current_time();
        if self.before_genesis_at(current_time) {
            Duration::from_nanos(u128_to_u64(self.genesis_time - current_time))
        } else {
            let current_slot = self.slot_at_time(current_time).expect("is after genesis");
            let next_slot = current_slot + 1;
            let target_slot_in_nanos =
                slot_to_nanos(next_slot, self.seconds_per_slot, self.genesis_time);
            // safety: `target_slot_in_seconds` >= `current_time` always
            Duration::from_nanos(u128_to_u64(target_slot_in_nanos - current_time))
        }
    }
}

pub type SystemClock = Clock<SystemTimeProvider>;

#[cfg(feature = "async")]
use tokio_stream::Stream;

#[cfg(feature = "async")]
impl<T: TimeProvider + Send + Sync> Clock<T> {
    pub fn stream_slots(&self) -> impl Stream<Item = Slot> + '_ {
        async_stream::stream! {
            loop {
                let slot = self.current_slot().expect("after genesis");
                yield slot;
                let duration_until_next_slot = self.duration_until_slot(slot + 1);
                tokio::time::sleep(duration_until_next_slot).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct Ticker {
        tick: Mutex<u128>,
        seconds_per_slot: u128,
    }

    impl Ticker {
        fn tick(&self) {
            let mut tick = self.tick.lock().unwrap();
            *tick += Duration::from_secs(1).as_nanos();
        }

        fn tick_slot(&self) {
            let mut tick = self.tick.lock().unwrap();
            *tick += self.seconds_per_slot;
        }
    }

    impl TimeProvider for Arc<Ticker> {
        fn get_current_time(&self) -> u128 {
            *self.tick.lock().unwrap()
        }
    }

    fn new_ticker(seconds_per_slot: u64) -> Arc<Ticker> {
        let seconds_per_slot = Duration::from_secs(seconds_per_slot).as_nanos();
        Arc::new(Ticker { tick: Mutex::new(0), seconds_per_slot })
    }

    #[test]
    fn test_custom_time_provider() {
        let seconds_per_slot: u64 = 12;
        let time_provider = new_ticker(seconds_per_slot);
        let clock = Clock::new(0, seconds_per_slot, 32, time_provider.clone());
        assert_eq!(clock.duration_until_next_slot().as_secs(), 12);
        assert_eq!(clock.current_slot().unwrap(), 0);
        time_provider.tick();
        assert_eq!(clock.duration_until_next_slot().as_secs(), 11);
        assert_eq!(clock.current_slot().unwrap(), 0);
        for _ in 0..12 {
            time_provider.tick();
        }
        assert_eq!(clock.duration_until_next_slot().as_secs(), 11);
        let current_slot = clock.current_slot().unwrap();
        assert_eq!(current_slot, 1);
        let duration_until_previous_slot = clock.duration_until_slot(current_slot - 1);
        assert_eq!(duration_until_previous_slot, Duration::default());
        let duration_until_now = clock.duration_until_slot(current_slot);
        assert_eq!(duration_until_now, Duration::default());
        let next_slot = current_slot + 1;
        let duration_until_next_slot = clock.duration_until_slot(next_slot);
        assert_eq!(duration_until_next_slot, Duration::from_secs(11));
        assert_eq!(duration_until_next_slot, clock.duration_until_next_slot());
    }

    #[test]
    fn test_before_genesis() {
        let seconds_per_slot: u64 = 12;
        let time_provider = new_ticker(seconds_per_slot);
        let clock = Clock::new(10, seconds_per_slot, 32, time_provider.clone());
        assert_eq!(clock.duration_until_next_slot().as_secs(), 10);
        time_provider.tick();
        assert_eq!(clock.duration_until_next_slot().as_secs(), 9);

        assert!(clock.current_slot().is_none());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_slot_stream() {
        use tokio_stream::StreamExt;

        // note: make this very large so it is clear if the `TimeProvider` mocking is broken
        let seconds_per_slot: u64 = 1200000000;
        let time_provider = new_ticker(seconds_per_slot);
        let clock = Clock::new(0, seconds_per_slot, 12, time_provider.clone());
        let slot_stream = clock.stream_slots();

        tokio::pin!(slot_stream);

        let current_slot = clock.current_slot().unwrap();
        let target_slot = current_slot + 3;
        let mut slots = vec![];
        while let Some(slot) = slot_stream.next().await {
            if slot >= target_slot {
                break
            }
            slots.push(slot);
            time_provider.tick_slot();
        }
        assert_eq!(slots, (current_slot..target_slot).collect::<Vec<_>>());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    #[ignore = "uses wall clock time for mainnet params"]
    async fn test_slot_stream_mainnet() {
        use tokio_stream::StreamExt;

        let clock = for_mainnet();
        let slot_stream = clock.stream_slots();

        tokio::pin!(slot_stream);

        let current_slot = clock.current_slot().unwrap();
        let target_slot = current_slot + 3;
        let mut slots = vec![];
        while let Some(slot) = slot_stream.next().await {
            if slot >= target_slot {
                break
            }
            slots.push(slot);
        }
        assert_eq!(slots, (current_slot..target_slot).collect::<Vec<_>>());
    }
}
