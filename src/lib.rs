/*
** This file is a part of Iksemel (XML parser for Jabber/XMPP)
** Copyright (C) 2000-2025 Gurer Ozen
**
** Iksemel is free software: you can redistribute it and/or modify it
** under the terms of the GNU Lesser General Public License as
** published by the Free Software Foundation, either version 3 of
** the License, or (at your option) any later version.
*/

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyMemoryError;
use pyo3::prelude::*;

use iks::DocumentParser;
use iks::ParseError;
use iks::SyncCursor;

create_exception!(pyiks, BadXmlError, PyException);

enum PyIksError {
    NoMemory,
    BadXml(&'static str),
}

impl From<ParseError> for PyIksError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::NoMemory => PyIksError::NoMemory,
            ParseError::BadXml(msg) => PyIksError::BadXml(msg),
        }
    }
}

impl From<PyIksError> for PyErr {
    fn from(err: PyIksError) -> Self {
        match err {
            PyIksError::NoMemory => PyMemoryError::new_err("iks alloc failed"),
            PyIksError::BadXml(msg) => BadXmlError::new_err(msg),
        }
    }
}

#[pyclass(name = "Document", frozen)]
struct PyDocument {
    inner: SyncCursor,
}

#[pymethods]
impl PyDocument {
    #[new]
    fn new(bytes: &[u8]) -> Result<Self, PyIksError> {
        let mut parser = DocumentParser::with_size_hint(bytes.len());
        parser.parse_bytes(bytes)?;
        let document = parser.into_document()?;
        let inner = SyncCursor::new(document);
        Ok(Self { inner })
    }

    fn find_tag(&self, tag: &str) -> Option<Self> {
        let node = self.inner.clone().find_tag(tag);
        if node.is_null() {
            Some(Self { inner: node })
        } else {
            None
        }
    }

    fn __str__(&self) -> String {
        self.inner.clone().to_string()
    }
}

#[pymodule(name = "iks")]
fn pyiks(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    Ok(())
}
