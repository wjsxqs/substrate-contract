use codec::{Encode, Decode};

use frame_support::debug;
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::{DispatchError, AccountId32, RuntimeDebug};
use pallet_primitives::Balance;
use sp_std::prelude::*;
// use core::convert::TryInto;
use sp_std::convert::TryFrom;

// pub fn to_account_id<E: Ext>(account: <E::T as SysConfig>::AccountId) -> Result<AccountId32, DispatchError>
// where
// 	<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
// {
// 	let account_buf: [u8; 32] = account.as_ref().try_into().unwrap();
// 	Ok(AccountId32::from(account_buf))
// }

// pub fn to_account_id<T: Ext>(account: T::AccountId) -> Result<AccountId32, DispatchError>
// 	where
// 		T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
// {
// 	let account_buf: [u8; 32] = account.as_ref().try_into().unwrap();
// 	Ok(AccountId32::from(account_buf))
// }

pub fn to_account_id(account: &[u8]) -> AccountId32 {
	AccountId32::try_from(account).unwrap()
}

/// contract extension for `FetchRandom`
pub struct FetchRandomExtension;

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct TransferInput {
	to: AccountId32,
	amount: Balance,
	claim: Vec<u8>,
}

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
				let caller = env.ext().caller().clone(); // 调用这账户
				debug::info!("caller: {:?}", caller);
				let address = env.ext().address(); // 合约账户
				debug::info!("address: {:?}", address);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				let input = env.read(4)?;
				debug::info!("input with len: {}, {}, {}, {}", input[0], input[1], input[2], input[3]);

				let input: Vec<u8> = env.read_as()?;
				debug::info!("input: {:?}", input);

				let who = to_account_id(caller.as_ref());
				super::PoeModule::do_create_claim(who, input)?;
			}
			1003 => {
				debug::info!("run 1003");
				let mut env = env.buf_in_buf_out();
				// let caller = env.ext().caller().clone(); // 调用这账户
				let caller = to_account_id(env.ext().caller().clone().as_ref());
				debug::info!("caller: {:?}", caller);

				let in_len = env.in_len();
				debug::info!("in_len: {}", in_len);

				// let mut buffer = [0 as u8; in_len];
				let mut buffer = vec![0u8; in_len as usize];
				// let mut buf = &mut buffer[..];
				env.read_into(&mut &mut buffer[..])?;
				debug::info!("buffer: {:?}", buffer);
				// debug::info!("buf: {:?}", buf);
				//
				// let mut to = [0u8; 32];
				// to[..].copy_from_slice(&buffer[0..32]);
				//
				// let mut amount = [0u8; 16];
				// amount[..].copy_from_slice(&buffer[32..48]);
				//
				// let mut claim = [0u8; 5];
				// claim[..].copy_from_slice(&buffer[48..]);
				//
				// debug::info!("to: {:?}, amount: {:?}, claim: {:?}", to, amount, claim);

				// let input: Vec<u8> = env.read_as()?;
				let input: TransferInput = env.read_as()?;
				debug::info!("input: {:?}", input);

				let weight = 100_000;
				env.charge_weight(weight)?;

				// let who = to_account_id(caller.as_ref());
				// let to_account = AccountId32::from(to);
				// super::PoeModule::do_transfer_claim(&who, &to_account, claim.to_vec());

				super::PoeModule::do_transfer_claim(caller, input.to, input.claim)?;
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
