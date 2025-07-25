use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyFloat;

use crate::PyExpr;
use crate::conversion::Wrap;
use crate::error::PyPolarsErr;
use crate::map::lazy::{ToSeries, call_lambda_with_series};
use crate::py_modules::polars;

#[pymethods]
impl PyExpr {
    #[pyo3(signature = (window_size, weights, min_periods, center))]
    fn rolling_sum(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            ..Default::default()
        };
        self.inner.clone().rolling_sum(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed))]
    fn rolling_sum_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };
        Ok(self.inner.clone().rolling_sum_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center))]
    fn rolling_min(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            ..Default::default()
        };
        self.inner.clone().rolling_min(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed))]
    fn rolling_min_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };
        Ok(self.inner.clone().rolling_min_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center))]
    fn rolling_max(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            ..Default::default()
        };
        self.inner.clone().rolling_max(options).into()
    }
    #[pyo3(signature = (by, window_size, min_periods, closed))]
    fn rolling_max_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };
        Ok(self.inner.clone().rolling_max_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center))]
    fn rolling_mean(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            ..Default::default()
        };

        self.inner.clone().rolling_mean(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed))]
    fn rolling_mean_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };

        Ok(self.inner.clone().rolling_mean_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center, ddof))]
    fn rolling_std(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
        ddof: u8,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            fn_params: Some(RollingFnParams::Var(RollingVarParams { ddof })),
        };

        self.inner.clone().rolling_std(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed, ddof))]
    fn rolling_std_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
        ddof: u8,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: Some(RollingFnParams::Var(RollingVarParams { ddof })),
        };

        Ok(self.inner.clone().rolling_std_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center, ddof))]
    fn rolling_var(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
        ddof: u8,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            fn_params: Some(RollingFnParams::Var(RollingVarParams { ddof })),
        };

        self.inner.clone().rolling_var(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed, ddof))]
    fn rolling_var_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
        ddof: u8,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: Some(RollingFnParams::Var(RollingVarParams { ddof })),
        };

        Ok(self.inner.clone().rolling_var_by(by.inner, options).into())
    }

    #[pyo3(signature = (window_size, weights, min_periods, center))]
    fn rolling_median(
        &self,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            min_periods,
            weights,
            center,
            fn_params: None,
        };
        self.inner.clone().rolling_median(options).into()
    }

    #[pyo3(signature = (by, window_size, min_periods, closed))]
    fn rolling_median_by(
        &self,
        by: PyExpr,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };
        Ok(self
            .inner
            .clone()
            .rolling_median_by(by.inner, options)
            .into())
    }

    #[pyo3(signature = (quantile, interpolation, window_size, weights, min_periods, center))]
    fn rolling_quantile(
        &self,
        quantile: f64,
        interpolation: Wrap<QuantileMethod>,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            fn_params: None,
        };

        self.inner
            .clone()
            .rolling_quantile(interpolation.0, quantile, options)
            .into()
    }

    #[pyo3(signature = (by, quantile, interpolation, window_size, min_periods, closed))]
    fn rolling_quantile_by(
        &self,
        by: PyExpr,
        quantile: f64,
        interpolation: Wrap<QuantileMethod>,
        window_size: &str,
        min_periods: usize,
        closed: Wrap<ClosedWindow>,
    ) -> PyResult<Self> {
        let options = RollingOptionsDynamicWindow {
            window_size: Duration::try_parse(window_size).map_err(PyPolarsErr::from)?,
            min_periods,
            closed_window: closed.0,
            fn_params: None,
        };

        Ok(self
            .inner
            .clone()
            .rolling_quantile_by(by.inner, interpolation.0, quantile, options)
            .into())
    }

    #[pyo3(signature = (window_size, bias, min_periods, center))]
    fn rolling_skew(
        &self,
        window_size: usize,
        bias: bool,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights: None,
            min_periods,
            center,
            fn_params: Some(RollingFnParams::Skew { bias }),
        };

        self.inner.clone().rolling_skew(options).into()
    }

    #[pyo3(signature = (window_size, fisher, bias, min_periods, center))]
    fn rolling_kurtosis(
        &self,
        window_size: usize,
        fisher: bool,
        bias: bool,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights: None,
            min_periods,
            center,
            fn_params: Some(RollingFnParams::Kurtosis { fisher, bias }),
        };

        self.inner.clone().rolling_kurtosis(options).into()
    }

    #[pyo3(signature = (lambda, window_size, weights, min_periods, center))]
    fn rolling_map(
        &self,
        lambda: PyObject,
        window_size: usize,
        weights: Option<Vec<f64>>,
        min_periods: Option<usize>,
        center: bool,
    ) -> Self {
        let min_periods = min_periods.unwrap_or(window_size);
        let options = RollingOptionsFixedWindow {
            window_size,
            weights,
            min_periods,
            center,
            ..Default::default()
        };
        let function = move |s: &Series| {
            Python::with_gil(|py| {
                let out =
                    call_lambda_with_series(py, s, None, &lambda).expect("python function failed");
                match out.getattr(py, "_s") {
                    Ok(pyseries) => {
                        let Ok(s) = pyseries
                            .to_series(py, polars(py), s.name())
                            .map_err(|e| panic!("{e:?}"));
                        s
                    },
                    Err(_) => {
                        let obj = out;
                        let is_float = obj.bind(py).is_instance_of::<PyFloat>();

                        let dtype = s.dtype();

                        use DataType::*;
                        let Ok(s) = match dtype {
                            UInt8 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(UInt8Chunked::from_slice(PlSmallStr::EMPTY, &[v as u8])
                                        .into_series())
                                } else {
                                    obj.extract::<u8>(py).map(|v| {
                                        UInt8Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            UInt16 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(UInt16Chunked::from_slice(PlSmallStr::EMPTY, &[v as u16])
                                        .into_series())
                                } else {
                                    obj.extract::<u16>(py).map(|v| {
                                        UInt16Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            UInt32 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(UInt32Chunked::from_slice(PlSmallStr::EMPTY, &[v as u32])
                                        .into_series())
                                } else {
                                    obj.extract::<u32>(py).map(|v| {
                                        UInt32Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            UInt64 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(UInt64Chunked::from_slice(PlSmallStr::EMPTY, &[v as u64])
                                        .into_series())
                                } else {
                                    obj.extract::<u64>(py).map(|v| {
                                        UInt64Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Int8 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(Int8Chunked::from_slice(PlSmallStr::EMPTY, &[v as i8])
                                        .into_series())
                                } else {
                                    obj.extract::<i8>(py).map(|v| {
                                        Int8Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Int16 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(Int16Chunked::from_slice(PlSmallStr::EMPTY, &[v as i16])
                                        .into_series())
                                } else {
                                    obj.extract::<i16>(py).map(|v| {
                                        Int16Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Int32 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(Int32Chunked::from_slice(PlSmallStr::EMPTY, &[v as i32])
                                        .into_series())
                                } else {
                                    obj.extract::<i32>(py).map(|v| {
                                        Int32Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Int64 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(Int64Chunked::from_slice(PlSmallStr::EMPTY, &[v as i64])
                                        .into_series())
                                } else {
                                    obj.extract::<i64>(py).map(|v| {
                                        Int64Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Int128 => {
                                if is_float {
                                    let v = obj.extract::<f64>(py).unwrap();
                                    Ok(Int128Chunked::from_slice(PlSmallStr::EMPTY, &[v as i128])
                                        .into_series())
                                } else {
                                    obj.extract::<i128>(py).map(|v| {
                                        Int128Chunked::from_slice(PlSmallStr::EMPTY, &[v])
                                            .into_series()
                                    })
                                }
                            },
                            Float32 => obj.extract::<f32>(py).map(|v| {
                                Float32Chunked::from_slice(PlSmallStr::EMPTY, &[v]).into_series()
                            }),
                            Float64 => obj.extract::<f64>(py).map(|v| {
                                Float64Chunked::from_slice(PlSmallStr::EMPTY, &[v]).into_series()
                            }),
                            dt => panic!("{dt:?} not implemented"),
                        }
                        .map_err(|e| panic!("{e:?}"));
                        s
                    },
                }
            })
        };
        let output_type = if options.weights.is_some() {
            GetOutput::float_type()
        } else {
            GetOutput::same_type()
        };
        self.inner
            .clone()
            .rolling_map(Arc::new(function), output_type, options)
            .into()
    }
}
