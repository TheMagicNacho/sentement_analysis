use pyo3::prelude::*;
use pyembed::*;



pub struct Summarizer;

impl Summarizer {

    /// read_file is a public function that takes a path to a file and returns a string.
    /// 
    /// On success, it returns a string of the summary from the text provided.
    /// On failure, it returns an empty string. And prints the error to stderr.
    pub async fn read_file(path: &str) -> String {
        let from_python = Self::runner(path).await;
        return from_python; // remove before flight
    }

    /// runner is a private function that runs the Python code.
    /// It is staticly linked to the directory `python/ai.py`.
    /// Errors from python are returned here as a string and are un-handled.
    async fn runner(input: &str) -> String {
        let mut config: OxidizedPythonInterpreterConfig<'_> = OxidizedPythonInterpreterConfig::default();
        config.interpreter_config.isolated = Some(true);
        config.interpreter_config.filesystem_encoding = Some("utf-8".to_string());
        config.set_missing_path_configuration = false;
        config.interpreter_config.parse_argv = Some(false);
        config.argv = Some(vec!["python".into()]);
        config.interpreter_config.executable = Some("python".into());

        let embeded_python: MainPythonInterpreter<'_, '_> = MainPythonInterpreter::new(config).unwrap();
        let py_ai: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/ai.py"));

        let from_python = embeded_python.with_gil(|py: Python<'_>| -> PyResult<Py<PyAny>> {
            let app: Py<PyAny> = PyModule::from_code(py, py_ai, "", "")?

            .getattr("run")?.into();
            app.call1(py, (input,))
        });

        match from_python {
            Ok(result) => {
                return result.to_string();
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                return String::new();
            }
        }
    }
}
