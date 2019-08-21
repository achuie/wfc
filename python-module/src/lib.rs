#[macro_use]
extern crate cpython;
extern crate coord_2d;
extern crate image;
extern crate rand;
extern crate wfc_image;

use coord_2d::Size;
use cpython::{PyErr, PyInt, PyObject, PyResult, Python, exc};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::num::NonZeroU32;
use wfc_image::*;
use std::error::Error;

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
    data seed: u64;

    def __new__(_cls, seed: u64) -> PyResult<Resolver> {
        Resolver::create_instance(
            py,
            seed
        )
    }

    def generate_image(
        &self,
        input_path: &str,
        pattern_size: u32,
        output_size: Vec<u32>,
        orientation: i32,
        retry: i32,
        output_path: &str
    ) -> PyResult<PyObject> {
        let input_image = image::open(input_path).unwrap();
        let out_size = Size::new(output_size[0], output_size[1]);
        let patt_size =
            NonZeroU32::new(pattern_size).expect("*** Pattern size must not be zero. ***");
        let orient = orientFromPython(orientation as usize);
        let mut rng = StdRng::seed_from_u64(*self.seed(py));

        match generate_image_with_rng(
            &input_image,
            patt_size,
            out_size,
            &orient,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(retry as usize),
            &mut rng,
        ) {
            Err(_) => {
                eprintln!("*** Too many contradictions. ***");
                Err(PyErr::new::<exc::TypeError, _>(py, "*** Too many contradictions. ***"))
            }
            Ok(output_image) => {
                println!("*** Saving output image. ***");
                output_image.save(output_path);
                Ok(py.None())
            }
        }
    }
});
