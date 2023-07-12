#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod tipos;

use frame_support::traits::{Currency, Get};
use frame_support::sp_runtime::traits::{Zero, Saturating};
use tipos::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type LargoMinimoNombreProyecto: Get<u32>;

		#[pallet::constant]
		type LargoMaximoNombreProyecto: Get<u32>;

		type Currency: Currency<Self::AccountId>; // Pueden no utilizarlo.
	}

	#[pallet::storage]
	#[pallet::getter(fn stage)]
	pub type CrowdfundingStage<T> = StorageValue<_, Stage, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub type Projects<T> =
		StorageMap<_, Blake2_128Concat, BoundedString<T>, BalanceOf<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// El proyecto fue creado exitosamente
    ProjectCreated { who: T::AccountId, name: ProjectName<T> },
		/// El proyecto fue abonado exitosamente
    ProjectSupported { name: ProjectName<T>, amount: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Etapa incorrecta
		IncorrectStage,
		/// El nombre del proyecto es muy largo
		NameTooLong,
		/// El nombre del proyecto es muy corto
		NameTooShort,
		/// El usuario quiso apoyar un proyecto con más fondos de los que dispone.
		InsufficientFunds,
		/// El usuario quiso apoyar un proyecto inexistente.
		ProjectDoesNotExist,
		/// El proyecto ya esta registrado
		ProjectAlreadyRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Crea un proyecto.
		pub fn crear_proyecto(origen: OriginFor<T>, nombre: String) -> DispatchResult {
			// Completar este método.
			let mut stage = CrowdfundingStage::<T>::get();
			ensure!(matches!(stage, Stage::NameGeneration), Error::<T>::IncorrectStage);

			let who = ensure_signed(origen)?;
			let project_name: ProjectName<T> = nombre.try_into().map_err(|_| Error::<T>::NameTooLong)?;

			ensure!(!Projects::<T>::contains_key(&project_name), Error::<T>::ProjectAlreadyRegistered);

			let initial_balance = BalanceOf::<T>::zero();
			Projects::<T>::insert(&project_name, initial_balance);
			
			stage.next();
			CrowdfundingStage::<T>::set(stage);

			Self::deposit_event(Event::ProjectCreated { who, name: project_name });
			Ok(())
		}

		pub fn apoyar_proyecto(
			origen: OriginFor<T>,
			nombre: String,
			cantidad: BalanceOf<T>,
		) -> DispatchResult {
			let stage = CrowdfundingStage::<T>::get();
			ensure!(matches!(stage, Stage::FundCollection), Error::<T>::IncorrectStage);

			let who = ensure_signed(origen)?;

			let project_name: ProjectName<T> = nombre.try_into().map_err(|_| Error::<T>::NameTooLong)?;
			ensure!(Projects::<T>::contains_key(&project_name), Error::<T>::ProjectDoesNotExist);

			let mut project_balance = Projects::<T>::get(&project_name);
			project_balance = project_balance.saturating_add(cantidad);
			Projects::<T>::insert(&project_name, project_balance);

			Self::deposit_event(Event::ProjectSupported { name: project_name, amount: cantidad });
			Ok(())
		}
	}
}
