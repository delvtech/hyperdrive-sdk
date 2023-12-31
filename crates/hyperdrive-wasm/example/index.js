import * as hyperdriveWasm from '@delvtech/hyperdrive-wasm';

const ZERO_ADDRESS = '0x'.padEnd(42, '0');
// const MAX_U256 = '0x'.padEnd(66, 'F');

// const MAX_BUDGET = BigInt(MAX_U256).toString();
const mockPoolInfo = {
  shareReserves: '10000000000000000000000000',
  shareAdjustment: '0',
  zombieShareReserves: '0',
  bondReserves: '10217899519533796120000000',
  lpTotalSupply: '9999990000000000000000000',
  sharePrice: '1000000000000000000',
  longsOutstanding: '0',
  longAverageMaturityTime: '0',
  shortsOutstanding: '0',
  shortAverageMaturityTime: '0',
  withdrawalSharesReadyToWithdraw: '0',
  withdrawalSharesProceeds: '0',
  lpSharePrice: '1000000000000000000',
  longExposure: '0',
};
const mockPoolConfig = {
  baseToken: ZERO_ADDRESS,
  governance: ZERO_ADDRESS,
  feeCollector: ZERO_ADDRESS,
  fees: {
    curve: '100000000000000000',
    flat: '500000000000000',
    governanceLP: '10000000000000000',
    governanceZombie: '100000000000000000',
  },
  initialSharePrice: '1000000000000000000',
  minimumShareReserves: '10000000000000000000',
  minimumTransactionAmount: '1000000000000000',
  timeStretch: '44463125629060298',
  positionDuration: '604800',
  checkpointDuration: '3600',
  linkerCodeHash: '0x'.padEnd(66, '0'),
  linkerFactory: ZERO_ADDRESS,
};

async function main() {
  hyperdriveWasm.initSync(hyperdriveWasm.wasmBuffer);

  const spotRate = hyperdriveWasm.getSpotRate(mockPoolInfo, mockPoolConfig);
  console.log('spotRate:', spotRate);

  const spotPrice = hyperdriveWasm.getSpotPrice(mockPoolInfo, mockPoolConfig);
  console.log('spotPrice:', spotPrice);

  const baseAmountRequired = hyperdriveWasm.calcOpenShort(
    mockPoolInfo,
    mockPoolConfig,
    (10e18).toString()
  );
  console.log('baseAmountRequired:', baseAmountRequired);
}

main();
