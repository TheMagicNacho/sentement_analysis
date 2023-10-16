use pyo3::prelude::*;

pub struct Summarizer;

impl Summarizer {

    /// read_file is a public function that takes a path to a file and returns a string.
    /// 
    /// On success, it returns a string of the summary from the text provided.
    /// On failure, it returns an empty string. And prints the error to stderr.
    pub fn read_file(path: &str) -> String {
        let from_python: Result<Py<PyAny>, PyErr> = Self::runner(path);
        match from_python {
            Ok(result) => result.to_string(),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                String::new()
            }
        }
    }

    /// runner is a private function that runs the Python code.
    /// It is staticly linked to the directory `python/ai.py`.
    /// Errors from python are returned here as a string and are un-handled.
    /// 
    fn runner(path: &str) -> Result<Py<PyAny>, PyErr> {
        let py_ai: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/ai.py"));

        let from_python: Result<Py<PyAny>, PyErr> = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            let app: Py<PyAny> = PyModule::from_code(py, py_ai, "", "")?
            .getattr("run")?.into();
            app.call1(py, (path,))
    });
        return from_python;
    }
}
