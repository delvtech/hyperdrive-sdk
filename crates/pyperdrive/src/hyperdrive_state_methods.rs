use ethers::core::types::{I256, U256};
use fixed_point::FixedPoint;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyErr;

pub use crate::utils::*;
use crate::HyperdriveState;
pub use crate::PyPoolConfig;
pub use crate::PyPoolInfo;
use hyperdrive_math::State;
use hyperdrive_math::YieldSpace;

#[pymethods]
impl HyperdriveState {
    #[new]
    pub fn __init__(pool_config: &PyAny, pool_info: &PyAny) -> PyResult<Self> {
        let rust_pool_config = PyPoolConfig::extract(pool_config)?.pool_config;
        let rust_pool_info = PyPoolInfo::extract(pool_info)?.pool_info;
        let state = State::new(rust_pool_config, rust_pool_info);
        Ok(HyperdriveState::new(state))
    }

    pub fn get_max_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.get_max_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_spot_price_after_long(&self, long_amount: &str) -> PyResult<String> {
        let long_amount_fp = FixedPoint::from(U256::from_dec_str(long_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert long_amount string to U256")
        })?);
        let result_fp = self.state.get_spot_price_after_long(long_amount_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_solvency(&self) -> PyResult<String> {
        let result_fp = self.state.get_solvency();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_long_amount(&self, base_amount: &str) -> PyResult<String> {
        let base_amount_fp = FixedPoint::from(U256::from_dec_str(base_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert base_amount string to U256")
        })?);
        let result_fp = self.state.get_long_amount(base_amount_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_spot_price(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_price();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_spot_rate(&self) -> PyResult<String> {
        let result_fp = self.state.get_spot_rate();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_short_deposit(
        &self,
        short_amount: &str,
        spot_price: &str,
        open_share_price: &str,
    ) -> PyResult<String> {
        let short_amount_fp = FixedPoint::from(U256::from_dec_str(short_amount).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert short_amount string to U256")
        })?);
        let spot_price_fp = FixedPoint::from(U256::from_dec_str(spot_price).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert spot_price string to U256")
        })?);
        let open_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert open_share_price string to U256")
            })?);
        let result_fp = 
            self.state.get_short_deposit(short_amount_fp, spot_price_fp, open_share_price_fp).unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_bonds_out_given_shares_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = self.state.calculate_bonds_out_given_shares_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_in_given_bonds_out_up(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = self.state.calculate_shares_in_given_bonds_out_up(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_in_given_bonds_out_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = self.state.calculate_shares_in_given_bonds_out_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_out_given_bonds_in_down(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = self.state.calculate_shares_out_given_bonds_in_down(amount_in_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_shares_out_given_bonds_in_down_safe(&self, amount_in: &str) -> PyResult<String> {
        let amount_in_fp = FixedPoint::from(U256::from_dec_str(amount_in).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let result_fp = 
            self.state.calculate_shares_out_given_bonds_in_down_safe(amount_in_fp).unwrap();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_max_buy(&self) -> PyResult<String> {
        let result_fp = self.state.calculate_max_buy();
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn calculate_max_sell(&self, minimum_share_reserves: &str) -> PyResult<String> {
        let minimum_share_reserves_fp = 
            FixedPoint::from(U256::from_dec_str(minimum_share_reserves).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
            })?);
        let result_fp = self.state.calculate_max_sell(minimum_share_reserves_fp);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn to_checkpoint(&self, time: &str) -> PyResult<String> {
        let time_int = U256::from_dec_str(time)
            .map_err(|_| PyErr::new::<PyValueError, _>("Failed to convert time string to U256"))?;
        let result_int = self.state.to_checkpoint(time_int);
        let result = result_int.to_string();
        return Ok(result);
    }

    pub fn get_max_long(
        &self,
        budget: &str,
        checkpoint_exposure: &str,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let result_fp =
            self.state
                .get_max_long(budget_fp, checkpoint_exposure_i, maybe_max_iterations);
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }

    pub fn get_max_short(
        &self,
        budget: &str,
        open_share_price: &str,
        checkpoint_exposure: &str,
        maybe_conservative_price: Option<&str>,
        maybe_max_iterations: Option<usize>,
    ) -> PyResult<String> {
        let budget_fp = FixedPoint::from(U256::from_dec_str(budget).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert budget string to U256")
        })?);
        let open_share_price_fp =
            FixedPoint::from(U256::from_dec_str(open_share_price).map_err(|_| {
                PyErr::new::<PyValueError, _>("Failed to convert open_share_price string to U256")
            })?);
        let checkpoint_exposure_i = I256::from_dec_str(checkpoint_exposure).map_err(|_| {
            PyErr::new::<PyValueError, _>("Failed to convert checkpoint_exposure string to I256")
        })?;
        let maybe_conservative_price_fp = if let Some(conservative_price) = maybe_conservative_price
        {
            Some(FixedPoint::from(
                U256::from_dec_str(conservative_price).map_err(|_| {
                    PyErr::new::<PyValueError, _>(
                        "Failed to convert maybe_conservative_price string to U256",
                    )
                })?,
            ))
        } else {
            None
        };
        let result_fp = self.state.get_max_short(
            budget_fp,
            open_share_price_fp,
            checkpoint_exposure_i,
            maybe_conservative_price_fp,
            maybe_max_iterations,
        );
        let result = U256::from(result_fp).to_string();
        return Ok(result);
    }
}
