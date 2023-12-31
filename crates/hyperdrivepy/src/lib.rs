mod hyperdrive_state;
mod hyperdrive_state_methods;
mod hyperdrive_utils;
mod pool_config;
mod pool_info;
mod utils;

use pyo3::prelude::*;

use hyperdrive_state::HyperdriveState;
pub use hyperdrive_state_methods::*;
pub use hyperdrive_utils::{
    calculate_bonds_given_shares_and_rate, get_effective_share_reserves, get_time_stretch,
};
pub use pool_config::PyPoolConfig;
pub use pool_info::PyPoolInfo;
pub use utils::*;

/// Get the share reserves after subtracting the adjustment used for
/// A pyO3 wrapper for the hyperdrie_math crate.
#[pymodule]
#[pyo3(name = "hyperdrivepy")]
fn hyperdrivepy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperdriveState>()?;
    m.add_function(wrap_pyfunction!(calculate_bonds_given_shares_and_rate, m)?)?;
    m.add_function(wrap_pyfunction!(get_effective_share_reserves, m)?)?;
    m.add_function(wrap_pyfunction!(get_time_stretch, m)?)?;
    Ok(())
}
