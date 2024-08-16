use pyo3::{exceptions::PyTypeError, prelude::*, types::PyLong};

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(n: Bound<'_, PyLong>) -> PyResult<Vec<i64>> {
    let number = n.extract::<usize>().map_err(|_| PyTypeError::new_err("n must be an integer"))?;
    let list = match number {
        0 => vec![],
        1 => vec![0],
        2 => vec![0, 1],
        _ => {
            let mut fib: Vec<i64> = vec![0,1];
            while number > fib.len(){
                let fib_len = fib.len();
                let next_val = fib[fib_len-2..].iter().sum();
                fib.push(next_val)
            }
            fib
        }
    };
    Ok(list)
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
