use pyo3::prelude::*;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
fn fibonacci(n: i64) -> Vec<i64> {
    match n {
        0 => vec![],
        1 => vec![0],
        2 => vec![0, 1],
        _ => {
            let mut fib: Vec<i64> = vec![0,1];
            while n > fib.len() as i64{
                let fib_len = fib.len();
                let next_val = fib[fib_len-2..].iter().sum();
                fib.push(next_val)
            }
            fib
        }
    }
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
