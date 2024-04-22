pub mod beacon_block;
pub mod beacon_state;
pub mod block_processing;
pub mod constants;
<<<<<<< HEAD
pub mod execution_engine;
=======
pub mod epoch_processing;
>>>>>>> 3b086684 (Create `epoch_processing` module and `process_registry_updates()`)
pub mod execution_payload;
pub mod fork;
pub mod genesis;
pub mod helpers;
pub mod operations;
pub mod presets;
pub mod spec;

pub use spec::*;

pub use presets::{mainnet, minimal, Preset};
