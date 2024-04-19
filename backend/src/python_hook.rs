use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub async fn run_py_hook(input: String) -> PyResult<()>
{
    println!("Running Python hook.");
    let py_psort = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/py_assets/Powersort.py"));
    let py_tsort = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/py_assets/Timsort.py"));
    let py_counters = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/py_assets/Counters.py"));

    let psort_python_app = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
       PyModule::from_code_bound(py, py_psort, "Counters", "Counters")?;

        let app: Py<PyAny> = PyModule::from_code_bound(py, py_psort, "", "")?
            .getattr("sort")?
            .into();
        app.call1(py, (input.clone(),))
    });

    let tsort_python_app = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code_bound(py, py_counters, "Counters", "Counters")?;
        let app: Py<PyAny> = PyModule::from_code_bound(py, py_tsort, "", "")?
            .getattr("name")?
            .into();
        app.call0(py)
    });

    println!("PY {}", psort_python_app?);

    Ok(())
}