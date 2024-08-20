// TODO: Define a base class named `Discount`, with a `percentage` attribute.
//  It should be possible to access the `percentage` attribute of a `Discount`.
//  It should also be possible to modify the `percentage` attribute of a `Discount`.
//  It must be enforced that the `percentage` attribute is a float between 0. and 1.
//  Then define two subclasses:
//  - `SeasonalDiscount` that inherits from `Discount` with two additional attributes, `to` and `from_`.
//    `from_` is a datetime object that represents the start of the discount period.
//    `to` is a datetime object that represents the end of the discount period.
//     Both `from_` and `to` should be accessible and modifiable.
//     The class should enforce that `from` is before `to`.
//  - `CappedDiscount` that inherits from `Discount` with an additional attribute `cap`.
//    `cap` is a float that represents the maximum discount (in absolute value) that can be applied.
//    It should be possible to access and modify the `cap` attribute.
//    The class should enforce that `cap` is a non-zero positive float.
//
// All classes should have a method named `apply` that takes a price (float) as input and
// returns the discounted price.
// `SeasonalDiscount` should raise an `ExpiredDiscount` exception if `apply` is called but
// the current date is outside the discount period.
use chrono::{DateTime, Utc};
use pyo3::create_exception;
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(subclass)]
struct Discount {
    #[pyo3(get)]
    percentage: f64,
}

#[pymethods]
impl Discount {
    #[new]
    fn new(percentage: f64) -> PyResult<Self> {
        Discount::discount_check(percentage)?;
        Ok(Self { percentage })
    }
    #[setter]
    fn set_percentage(&mut self, percentage: f64) -> PyResult<()> {
        Discount::discount_check(percentage)?;
        self.percentage = percentage;
        Ok(())
    }
    fn apply(&self, price: f64) -> PyResult<f64> {
        let disconted_price: f64 = price * (1.0 - self.percentage);
        Ok(disconted_price)
    }
}

impl Discount {
    fn discount_check(value: f64) -> PyResult<()> {
        if (value >= 0.0) & (value <= 1.0) {
            return Ok(());
        }
        Err(PyValueError::new_err("Percentage must be between 0 and 1"))
    }
}

#[pyclass(extends=Discount)]
struct SeasonalDiscount {
    #[pyo3(get, set)]
    from_: DateTime<Utc>,
    #[pyo3(get, set)]
    to: DateTime<Utc>,
}

#[pymethods]
impl SeasonalDiscount {
    #[new]
    fn new(
        percentage: f64,
        from_: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> PyResult<PyClassInitializer<Self>> {
        if from_ >= to {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "`from_` date must be before `to` date",
            ));
        }
        let discount = Discount::new(percentage)?;
        let seasonal = SeasonalDiscount { from_, to };
        Ok(PyClassInitializer::from(discount).add_subclass(seasonal))
    }

    fn apply(self_: PyRef<'_, Self>, price: f64) -> PyResult<f64> {
        let now = Utc::now();
        if now < self_.from_ || now > self_.to {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Discount is expired.",
            ));
        }
        self_.as_super().apply(price)
    }
}

create_exception!(outro2, ExpiredDiscount, pyo3::exceptions::PyException);

#[pyclass(extends=Discount)]
struct CappedDiscount {
    #[pyo3(get, set)]
    cap: f64,
}

#[pymethods]
impl CappedDiscount {
    #[new]
    fn new(percentage: f64, cap: f64) -> PyResult<PyClassInitializer<Self>> {
        let discount = Discount::new(percentage)?;
        CappedDiscount::check_cap(cap)?;
        let cap = Self { cap };
        Ok(PyClassInitializer::from(discount).add_subclass(cap))
    }
    fn apply(self_: PyRef<'_, Self>, price: f64) -> PyResult<f64> {
        let discount_price = self_.as_super().percentage as f64 * price;
        if discount_price < self_.cap {
            return self_.as_super().apply(price);
        }
        Ok(self_.cap)
    }
}

impl CappedDiscount {
    fn check_cap(cap: f64) -> PyResult<()> {
        if cap < 0.0 {
            return Err(PyValueError::new_err("Cap must be a positive number"));
        }
        Ok(())
    }
}

#[pymodule]
fn outro2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Discount>()?;
    m.add_class::<SeasonalDiscount>()?;
    m.add_class::<CappedDiscount>()?;
    m.add(
        "ExpiredDiscount",
        m.py().get_type_bound::<ExpiredDiscount>(),
    )?;
    Ok(())
}
