extern crate alloc;

use alloc::vec::Vec;
use frame_support::{traits::Currency, BoundedVec};
use codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo, MaxEncodedLen)]
pub enum Stage {
	NameGeneration,
  FundCollection,
  Fulfillment,
}

impl Default for Stage {
	fn default() -> Self {
		Self::NameGeneration
	}
}

impl Stage {
	pub fn next(&mut self) {
		use Stage::*;

		*self = match *self {
			NameGeneration => FundCollection,
			FundCollection => Fulfillment,
      Fulfillment => Fulfillment,
		};
	}
}

pub type CuentaDe<T> = <T as frame_system::Config>::AccountId;
pub type BalanceOf<T> = <<T as crate::Config>::Currency as Currency<CuentaDe<T>>>::Balance;

pub type String = Vec<u8>;
pub type BoundedString<T> = BoundedVec<u8, <T as crate::Config>::LargoMaximoNombreProyecto>;

pub type ProjectName<T> = BoundedString<T>;
