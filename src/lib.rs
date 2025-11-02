use pyo3::prelude::*;

use iks::DocumentParser;
use iks::SyncCursor;

#[pyclass(name = "Document", frozen)]
struct PyDocument {
    inner: SyncCursor,
}

#[pymethods]
impl PyDocument {
    #[new]
    fn new(bytes: &[u8]) -> Self {
        let mut parser = DocumentParser::with_size_hint(bytes.len());
        parser.parse_bytes(bytes).unwrap();
        let document = parser.into_document().unwrap();
        let inner = SyncCursor::new(document);
        Self { inner }
    }
}

#[pymodule(name = "iks")]
fn pyiks(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    Ok(())
}
