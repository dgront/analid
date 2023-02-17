
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::pymethods;

use ::analid as rust_analid;


#[pyclass]
#[derive(Clone)]
pub struct Point {
    inner: rust_analid::Point
}

#[pyclass]
#[derive(Clone)]
pub struct PlotBounds { inner: rust_analid::PlotBounds }

#[pymethods]
impl PlotBounds {
    #[getter(min_x)]
    fn min_x(&self) -> PyResult<f64> { Ok(self.inner.min_x) }
    #[getter(min_y)]
    fn min_y(&self) -> PyResult<f64> { Ok(self.inner.min_x) }
    #[getter(max_x)]
    fn max_x(&self) -> PyResult<f64> { Ok(self.inner.max_x) }
    #[getter(max_y)]
    fn max_y(&self) -> PyResult<f64> { Ok(self.inner.max_x) }
}


#[pymethods]
impl Point {
    /// Creates a new point from given coordinates
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Point { Point { inner: rust_analid::Point::new(x, y, z) } }

    #[staticmethod]
    pub fn from_csv(line: String) -> PyResult<Point>  {
        return match rust_analid::Point::from_csv(line.as_str()) {
            Ok(point) => {Ok(Point{inner: point})}
            Err(_) => {Err(PyValueError::new_err("can't parse the input line"))}
        }
    }

    fn __str__(&self) -> PyResult<String>   { Ok(format!("{}", self.inner)) }
}

#[pyclass]
pub struct PlotStatistics { inner: rust_analid::PlotStatistics }

#[pymethods]
impl PlotStatistics {
    fn __str__(&self) -> PyResult<String>   { Ok(format!("{}", self.inner)) }
    #[getter(key)]
    fn key(&self) -> PyResult<(i16,i16)> { Ok(self.inner.key) }
    #[getter(min)]
    fn min(&self) -> PyResult<f64> { Ok(self.inner.min) }
    #[getter(max)]
    fn max(&self) -> PyResult<f64> { Ok(self.inner.max) }
    #[getter(mode)]
    fn mode(&self) -> PyResult<f64> { Ok(self.inner.mode) }
    #[getter(avg)]
    fn avg(&self) -> PyResult<f64> { Ok(self.inner.avg) }
    #[getter(count)]
    fn count(&self) -> PyResult<usize> { Ok(self.inner.count) }
}

#[pyclass]
pub struct Grid { inner: rust_analid::Grid }

#[pymethods]
impl Grid {
    #[new]
    pub fn new(dx: f64, dy: f64, points: Vec<Point>) -> Grid {
        let mut inner_vec = vec![];
        for p in points {
            inner_vec.push(p.inner);
        }
        Grid{inner: rust_analid::Grid::new(dx, dy, inner_vec)}
    }

    pub fn hash(&self, p:&Point) -> (i16,i16) { self.inner.hash(&p.inner) }

    pub fn count_points(&self, key: (i16, i16)) -> usize { self.inner.count_points(&key) }

    pub fn bounds(&self) -> PlotBounds { PlotBounds{ inner: self.inner.bounds().clone() } }

    pub fn keys(&self) -> Vec<(i16, i16)> {
        let mut v = vec![];
        for k in self.inner.data().keys() { v.push(k.clone()); }

        return v;
    }

    pub fn plot_statistics(&self, key: (i16, i16)) -> PlotStatistics {
        PlotStatistics{inner: self.inner.plot_statistics(&key)}
    }
}

#[pyfunction]
pub fn read_points(fname: &str) -> Vec<Point> {
    let mut ret: Vec<Point> = vec![];
    for p in rust_analid::read_points(fname) {
        ret.push(Point{ inner: p});
    }

    return ret;
}

/// LIDAR analysis module written in Rust
#[pymodule]
fn pyanalid(py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add_class::<Point>()?;
    module.add_class::<Grid>()?;
    module.add_function(wrap_pyfunction!(read_points, module)?)?;
    Ok(())
}