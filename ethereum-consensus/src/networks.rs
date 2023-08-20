/// This module contains support for various Ethereum netowrks.
use crate::state_transition::Context;

// NOTE: the default genesis time here is usually seen on testnets
// where we have control over the genesis details
pub fn typical_genesis_time(context: &Context) -> u64 {
    context.min_genesis_time + context.genesis_delay
}
