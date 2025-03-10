// This default_weight is manually generated for UI integration testing purpose
// This bench_marking cli need to run to complete bench marking for all functions

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for lootbox pallet.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> lootbox::WeightInfo for WeightInfo<T> {
    fn mint(i: u32) -> Weight {
        (10_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((10_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
    }
}