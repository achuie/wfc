#[macro_use]
extern crate cpython;
extern crate rand;
extern crate wfc_image;

use cpython::{PyInt, PyObject, PyResult, Python};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use wfc_image::*;

py_module_initializer!(
    wavefunctioncollapse,
    initwavefunctioncollapse,
    PyInit_wavefunctioncollapse,
    |py, m| {
        m.add(py, "__doc__", "WFC from Rust.")?;
        m.add_class::<Resolver>(py)?;
        m.add_class::<Orient>(py)?;
        Ok(())
    }
);

py_class!(class Orient |py| {
    def __new__(_cls) -> PyResult<Orient> {
        Orient::create_instance(py)
    }

    def original(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 0))
    }

    def clockwise_90(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 1))
    }

    def clockwise_180(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 2))
    }

    def clockwise_270(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 3))
    }

    def diagonally_flipped(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 4))
    }

    def diagonally_flipped_clockwise_90(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 5))
    }

    def diagonally_flipped_clockwise_180(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 6))
    }

    def diagonally_flipped_clockwise_270(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 7))
    }

    def all(&self) -> PyResult<PyInt> {
        Ok(PyInt::new(py, 8))
    }
});

fn orientFromPython(o: usize) -> Vec<Orientation> {
    let orientations = vec![
        vec![Orientation::Original],
        vec![Orientation::Clockwise90],
        vec![Orientation::Clockwise180],
        vec![Orientation::Clockwise270],
        vec![Orientation::DiagonallyFlipped],
        vec![Orientation::DiagonallyFlippedClockwise90],
        vec![Orientation::DiagonallyFlippedClockwise180],
        vec![Orientation::DiagonallyFlippedClockwise270],
        vec![
            Orientation::Original,
            Orientation::Clockwise90,
            Orientation::Clockwise180,
            Orientation::Clockwise270,
            Orientation::DiagonallyFlipped,
            Orientation::DiagonallyFlippedClockwise90,
            Orientation::DiagonallyFlippedClockwise180,
            Orientation::DiagonallyFlippedClockwise270,
        ],
    ];

    orientations[o].clone()
}

py_class!(class Resolver |py| {
    data rng: RefCell<StdRng>;

    def __new__(_cls) -> PyResult<Resolver> {
        Resolver::create_instance(py, RefCell::new(StdRng::seed_from_u64(rand::thread_rng().gen())))
    }

    def set_seed(&self, s: u64) -> PyResult<PyObject> {
        self.rng(py).replace(StdRng::seed_from_u64(s));
        Ok(py.None())
    }

    //def generate_image(&self, input_image: str, pattern_size: int, output_size: int, orientation: int)
});
