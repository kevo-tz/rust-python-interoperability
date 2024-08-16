// TODO: Expose a function named `max_k` that takes a list of unsigned integers and return as output
//   a list containing the `k` largest numbers in the list, in descending order.
//
// Hint: you can use the `num_bigint` crate if you think it'd be useful.
use num_bigint::BigInt;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

#[pyfunction]
fn max_k(mut list: Vec<BigInt>, k: usize) -> PyResult<Vec<BigInt>> {
    if k > list.len() {
        return Err(PyValueError::new_err("more elements requested"));
    }

    if list.iter().any(|n| n < &BigInt::from(0)) {
        return Err(PyTypeError::new_err(
            "element in the list must be non-negative",
        ));
    }

    list.sort();
    list.reverse();
    list.truncate(k);

    Ok(list)
}

#[pymodule]
fn outro1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_k, m)?)?;
    Ok(())
}
