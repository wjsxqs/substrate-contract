use codec::{Encode, Decode};

use frame_support::debug;
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;
use sp_std::prelude::*;

/// contract extension for `FetchRandom`
pub struct FetchRandomExtension;

impl ChainExtension for FetchRandomExtension {
	fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
		where
			<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{

		match func_id {
			1001 => {
				debug::info!("run 1001");
				let mut env = env.buf_in_buf_out();
				let random_seed: [u8; 32] = super::RandomnessCollectiveFlip::random_seed().0;
				let random_slice = random_seed.encode();
				debug::native::trace!(
					target: "runtime",
					"[ChainExtension]|call|func_id:{:}",
					func_id
				);
				env.write(&random_slice, false, None)
					.map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
			}

			1002 => {
				debug::info!("run 1002");
				let mut env = env.buf_in_buf_out();
				let caller = env.ext().caller(); // 调用这账户
				debug::info!("caller: {:?}", caller);
				let address = env.ext().address(); // 合约账户
				debug::info!("address: {:?}", address);

				let input = env.read(4)?;
				debug::info!("input: {}, {}, {}, {}", input[0], input[1], input[2], input[3]);

				let mut buffer = [0 as u8; 10];
				let mut buf = &mut buffer[..];
				env.read_into(&mut buf);
				debug::info!("buf: {:?}", buf);

				let claim = vec![0, 1];
				super::PoeModule::do_create_claim(claim.clone())?;
			}

			_ => {
				debug::error!("call an unregistered `func_id`, func_id:{:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"));
			}
		}
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
