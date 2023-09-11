"""Stubs for hyperdrive math."""
from __future__ import annotations

from . import types

# pylint: disable=unused-argument
# pylint: disable=too-many-arguments

class HyperdriveState:
    """A class representing the hyperdrive contract state."""

    def __new__(cls, pool_config: types.PoolConfig, pool_info: types.PoolInfo) -> HyperdriveState:
        """Create the HyperdriveState instance."""
    def __init__(self, pool_config: types.PoolConfig, pool_info: types.PoolInfo) -> None:
        """Initializes the hyperdrive state.

        Arguments
        ---------
        pool_config : PoolConfig
            Static configuration for the hyperdrive contract.  Set at deploy time.
        pool_info : PoolInfo
            Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
        """
    def get_spot_price(self) -> str:
        """Get the spot price of the bond.

        Returns
        -------
        str
            The spot price as a string representation of a solidity uint256 value.
        """
    def get_max_long(self, budget: str, checkpoint_exposure: str, maybe_max_iterations: int | None) -> str:
        """Get the max amount of bonds that can be purchased for the given budget.

        Arguments
        ---------
        budget : str
            The account budget in base for making a long.
        checkpoint_exposure : str
            The net exposure for the given checkpoint.
        maybe_max_iterations : int
            The number of iterations to use for the Newtonian method.

        Returns
        -------
        str
            The maximum long as a string representation of a solidity uint256 value.
        """
    def get_max_short(
        self, budget: str, open_share_price: str, maybe_conservative_price: str | None, maybe_max_iterations: int | None
    ) -> str:
        """Get the max amount of bonds that can be shorted for the given budget.

        Arguments
        ---------
        budget : str (FixedPoint)
            The account budget in base for making a short.
        open_share_price : str (FixedPoint)
            The share price of underlying vault.
        maybe_conservative_price : str (FixedPoint) | None
            A lower bound on the realized price that the short will pay.
        maybe_max_iterations : int | None
            The number of iterations to use for the Newtonian method.

        Returns
        -------
        str
            The maximum short as a string representation of a solidity uint256 value.
        """

def get_max_long(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    budget: str,
    checkpoint_exposure: str,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be purchased for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract. Includes things like reserve levels and share prices.
    budget : str (FixedPoint)
        The account budget in base for making a long.
    checkpoint_exposure : str
        The net exposure for the given checkpoint.
    maybe_max_iterations : int | None
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str
        The maximum long as a string representation of a solidity uint256 value.
    """

def get_max_short(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
    budget: str,
    open_share_price: str,
    maybe_conservative_price: str | None,
    maybe_max_iterations: int | None,
) -> str:
    """Get the max amount of bonds that can be shorted for the given budget.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.
    budget : str (FixedPoint)
        The account budget in base for making a short.
    open_share_price : str (FixedPoint)
        The share price of underlying vault.
    maybe_conservative_price : str (FixedPoint) | None
        A lower bound on the realized price that the short will pay.
    maybe_max_iterations : int | None
        The number of iterations to use for the Newtonian method.

    Returns
    -------
    str
        The maximum short as a string representation of a solidity uint256 value.
    """

def get_spot_price(
    pool_config: types.PoolConfig,
    pool_info: types.PoolInfo,
) -> str:
    """Get the spot price of the bond.

    Arguments
    ---------
    pool_config : PoolConfig
        Static configuration for the hyperdrive contract.  Set at deploy time.
    pool_info : PoolInfo
        Current state information of the hyperdrive contract.  Includes things like reserve levels and share prices.

    Returns
    -------
    str
        The spot price as a string representation of a solidity uint256 value.
    """
