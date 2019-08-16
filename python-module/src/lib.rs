#[macro_use]
extern crate cpython;
extern crate rand;
extern crate wfc_image;

use cpython::{PyResult, Python, PyObject};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::cell::RefCell;
use wfc_image::*;

py_module_initializer!(
    wavefunctioncollapse,
    initwavefunctioncollapse,
    PyInit_wavefunctioncollapse,
    |py, m| {
        m.add(py, "__doc__", "WFC from Rust.")?;
        m.add(py, "Orientation", Orientation)?;
        m.add_class::<Resolver>(py)?;
        Ok(())
    }
);

py_class!(class Resolver |py| {
    data rng: RefCell<StdRng>;

    def __new__(_cls) -> PyResult<Resolver> {
        Resolver::create_instance(py, RefCell::new(StdRng::seed_from_u64(rand::thread_rng().gen())))
    }

    def set_seed(&self, s: u64) -> PyResult<PyObject> {
        self.rng(py).replace(StdRng::seed_from_u64(s));
        Ok(py.None())
    }

    def generate_image(&self, input_image: str, pattern_size: int, output_size: int, orientation:)
});
