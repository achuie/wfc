#[macro_use] extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(wavefunctioncollapse, initwavefunctioncollapse, PyInit_wavefunctioncollapse, |py, m| {
    m.add(py, "__doc__", "WFC from Rust.")?;
    Ok(())
});
