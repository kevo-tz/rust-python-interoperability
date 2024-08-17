use pyo3::{exceptions::PyValueError, prelude::*, types::PyLong};

// TODO: Add a `__new__` constructor to the `ShoppingOrder` class that takes the following arguments:
//  - `name` (non-empty string)
//  - `price` (non-zero integer)
//  - `quantity` (non-zero integer)
//  The constructor should raise a `ValueError` if any of the arguments are invalid.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    price: u64,
    #[pyo3(get, set)]
    quantity: u64,
}

#[pymethods]
impl ShoppingOrder {
    #[new]
    fn new(name: String, price: Bound<'_, PyLong>, quantity:  Bound<'_, PyLong>) -> PyResult<Self> {
        if name.is_empty() {
            return Err(PyValueError::new_err("Name can not be empty"));
        }
        let price: u64 = price.extract().map_err(|_| PyValueError::new_err("Value must be non-negative"))?;
        let quantity: u64 = quantity.extract().map_err(|_| PyValueError::new_err("Value must be non-negative"))?;

        Ok(Self {
            name,
            price,
            quantity,
        })
    }
}

#[pymodule]
fn constructors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
