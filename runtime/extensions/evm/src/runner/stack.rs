// Copyright 2019-2020 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

use crate::executor::stack::TraceExecutor as TraceExecutorT;
use ethereum_types::{H160, U256};
use evm::{executor::StackExecutor, Config as EvmConfig};
use moonbeam_rpc_primitives_debug::TraceExecutorResponse;
use pallet_evm::{
	runner::stack::{Backend, Runner},
	Config, ExitError, PrecompileSet, Vicinity,
};
use sp_std::vec::Vec;

pub trait TraceRunner<T: Config> {
	fn trace_execute<F>(
		source: H160,
		gas_limit: u32,
		config: &EvmConfig,
		f: F,
	) -> Result<TraceExecutorResponse, ExitError>
	where
		F: FnOnce(&mut StackExecutor<Backend<T>>) -> Result<TraceExecutorResponse, ExitError>;

	fn trace_call(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u32,
		config: &EvmConfig,
	) -> Result<TraceExecutorResponse, ExitError>;

	fn trace_create(
		source: H160,
		init: Vec<u8>,
		value: U256,
		gas_limit: u32,
		config: &EvmConfig,
	) -> Result<TraceExecutorResponse, ExitError>;
}

impl<T: Config> TraceRunner<T> for Runner<T> {
	fn trace_execute<F>(
		source: H160,
		gas_limit: u32,
		config: &EvmConfig,
		f: F,
	) -> Result<TraceExecutorResponse, ExitError>
	where
		F: FnOnce(&mut StackExecutor<Backend<T>>) -> Result<TraceExecutorResponse, ExitError>,
	{
		let vicinity = Vicinity {
			gas_price: U256::zero(),
			origin: source,
		};

		let backend = Backend::<T>::new(&vicinity);
		let mut executor = StackExecutor::new_with_precompile(
			&backend,
			gas_limit as u64,
			config,
			T::Precompiles::execute,
		);

		f(&mut executor)
	}

	fn trace_call(
		source: H160,
		target: H160,
		input: Vec<u8>,
		value: U256,
		gas_limit: u32,
		config: &EvmConfig,
	) -> Result<TraceExecutorResponse, ExitError> {
		Self::trace_execute(source, gas_limit, config, |executor| {
			executor.trace_call(source, target, value, input, gas_limit as u64)
		})
	}

	fn trace_create(
		source: H160,
		init: Vec<u8>,
		value: U256,
		gas_limit: u32,
		config: &EvmConfig,
	) -> Result<TraceExecutorResponse, ExitError> {
		Self::trace_execute(source, gas_limit, config, |executor| {
			executor.trace_create(source, value, init, gas_limit as u64)
		})
	}
}