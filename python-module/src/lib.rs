#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(
    wavefunctioncollapse,
    initwavefunctioncollapse,
    PyInit_wavefunctioncollapse,
    |py, m| {
        m.add(py, "__doc__", "WFC from Rust.")?;
        m.add_class::<Resolver>(py)?;
        Ok(())
    }
);

py_class!(class Resolver |py| {
    def __new__(_cls) -> PyResult<Resolver> {
        Resolver::create_instance(py)
    }
});
