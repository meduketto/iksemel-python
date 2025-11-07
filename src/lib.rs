/*
** This file is a part of Iksemel (XML parser for Jabber/XMPP)
** Copyright (C) 2000-2025 Gurer Ozen
**
** Iksemel is free software: you can redistribute it and/or modify it
** under the terms of the GNU Lesser General Public License as
** published by the Free Software Foundation, either version 3 of
** the License, or (at your option) any later version.
*/

use iks::Document;
use iks::DocumentParser;
use iks::ParseError;
use iks::SyncCursor;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyMemoryError;
use pyo3::prelude::*;

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
            PyIksError::NoMemory => PyMemoryError::new_err("pyiks alloc failed"),
            PyIksError::BadXml(msg) => BadXmlError::new_err(msg),
        }
    }
}

#[pyclass]
struct PyDocumentChildren {
    inner: SyncCursor,
}

#[pymethods]
impl PyDocumentChildren {
    fn __iter__(&self) -> Self {
        PyDocumentChildren {
            inner: self.inner.clone(),
        }
    }

    fn __next__(&mut self) -> Option<PyDocument> {
        if self.inner.is_null() {
            return None;
        }
        let current = self.inner.clone();
        self.inner = self.inner.clone().next();
        Some(PyDocument { inner: current })
    }
}

#[pyclass(name = "Document", frozen)]
struct PyDocument {
    inner: SyncCursor,
}

#[pymethods]
impl PyDocument {
    #[new]
    fn new(name: &str) -> Result<Self, PyIksError> {
        let document = Document::new(name)?;
        let inner = SyncCursor::new(document);
        Ok(Self { inner })
    }

    //
    // Edit
    //

    fn insert_tag(&self, tag: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().insert_tag(tag) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn append_tag(&self, tag: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().append_tag(tag) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn prepend_tag(&self, tag: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().prepend_tag(tag) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn insert_cdata(&self, cdata: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().insert_cdata(cdata) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn append_cdata(&self, cdata: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().append_cdata(cdata) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn prepend_cdata(&self, cdata: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().prepend_cdata(cdata) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn insert_attribute(&self, name: &str, value: &str) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().insert_attribute(name, value) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn set_attribute(&self, name: &str, value: Option<&str>) -> Result<PyDocument, PyIksError> {
        match self.inner.clone().set_attribute(name, value) {
            Ok(sc) => Ok(Self { inner: sc }),
            Err(err) => Err(err.into()),
        }
    }

    fn remove(&self) -> () {
        self.inner.clone().remove()
    }

    //
    // Navigation
    //

    fn next(&self) -> Self {
        Self {
            inner: self.inner.clone().next(),
        }
    }

    fn next_tag(&self) -> Self {
        Self {
            inner: self.inner.clone().next_tag(),
        }
    }

    fn previous(&self) -> Self {
        Self {
            inner: self.inner.clone().previous(),
        }
    }

    fn previous_tag(&self) -> Self {
        Self {
            inner: self.inner.clone().previous_tag(),
        }
    }

    fn parent(&self) -> Self {
        Self {
            inner: self.inner.clone().parent(),
        }
    }

    fn root(&self) -> Self {
        Self {
            inner: self.inner.clone().root(),
        }
    }

    fn first_child(&self) -> Self {
        Self {
            inner: self.inner.clone().first_child(),
        }
    }

    fn last_child(&self) -> Self {
        Self {
            inner: self.inner.clone().last_child(),
        }
    }

    fn first_tag(&self) -> Self {
        Self {
            inner: self.inner.clone().first_tag(),
        }
    }

    fn find_tag(&self, tag: &str) -> Self {
        let node = self.inner.clone().find_tag(tag);
        Self { inner: node }
    }

    //
    // Iterators
    //
    fn __iter__(&self) -> PyDocumentChildren {
        PyDocumentChildren {
            inner: self.inner.clone().first_child(),
        }
    }

    //
    // Properties
    //

    fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    fn is_tag(&self) -> bool {
        self.inner.is_tag()
    }

    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn attribute(&self, name: &str) -> Option<String> {
        self.inner.attribute(name).map(|attr| attr.to_string())
    }

    fn cdata(&self) -> String {
        self.inner.cdata().to_string()
    }

    fn __str__(&self) -> String {
        self.inner.clone().to_string()
    }
}

#[derive(FromPyObject)]
pub enum XmlText {
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    #[pyo3(transparent, annotation = "bytes")]
    Bytes(Vec<u8>),
}

#[pyfunction]
fn parse(xml_text: XmlText) -> Result<PyDocument, PyIksError> {
    let bytes = match xml_text {
        XmlText::Str(ref s) => s.as_bytes(),
        XmlText::Bytes(ref b) => b.as_slice(),
    };
    let mut parser = DocumentParser::with_size_hint(bytes.len());
    parser.parse_bytes(bytes)?;
    let document = parser.into_document()?;
    let inner = SyncCursor::new(document);
    Ok(PyDocument { inner })
}

#[pymodule]
fn pyiks(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    m.add_class::<PyDocumentChildren>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add("BadXmlError", py.get_type::<BadXmlError>())?;
    Ok(())
}
