#[cfg_attr(test, macro_use)]
extern crate ndarray;
#[macro_use]
extern crate abomonation_derive;
extern crate abomonation;

pub mod itembased;
pub mod lsh;
pub mod ridge;
pub mod mnb;

pub mod io_utils;

pub mod differential;

pub trait IncrementalDecrementalModel<T, I, O> {
    fn partial_fit(self: &mut Self, data: &[T]);
    fn forget(self: &mut Self, data: &T);

    fn predict(self: &Self, data: &I) -> O;
}