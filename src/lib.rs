/*
** This file is a part of Iksemel (XML parser for Jabber/XMPP)
** Copyright (C) 2000-2026 Gurer Ozen
**
** Iksemel is free software: you can redistribute it and/or modify it
** under the terms of the GNU Lesser General Public License as
** published by the Free Software Foundation, either version 3 of
** the License, or (at your option) any later version.
*/

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use iks::BadJid;
use iks::Document;
use iks::DocumentParser;
use iks::Jid;
use iks::ParseError;
use iks::SyncCursor;
use iks::XmppClient;
use iks::XmppClientError;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyMemoryError;
use pyo3::prelude::*;

create_exception!(pyiks, BadXmlError, PyException);
create_exception!(pyiks, BadJidError, PyException);
create_exception!(pyiks, XmppError, PyException);

enum PyIksError {
    NoMemory,
    BadXml(&'static str),
    BadJid(&'static str),
    StreamError(&'static str),
    IOError(String),
    TlsError(String),
}

impl From<ParseError> for PyIksError {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::NoMemory => PyIksError::NoMemory,
            ParseError::BadXml(msg) => PyIksError::BadXml(msg),
        }
    }
}

impl From<BadJid> for PyIksError {
    fn from(err: BadJid) -> Self {
        match err {
            BadJid(msg) => PyIksError::BadJid(msg),
        }
    }
}

impl From<XmppClientError> for PyIksError {
    fn from(err: XmppClientError) -> Self {
        match err {
            XmppClientError::NoMemory => PyIksError::NoMemory,
            XmppClientError::BadXml(msg) => PyIksError::BadXml(msg),
            XmppClientError::BadStream(msg) => PyIksError::StreamError(msg),
            XmppClientError::IOError(err) => PyIksError::IOError(err.to_string()),
            XmppClientError::TlsError(err) => PyIksError::TlsError(err.to_string()),
        }
    }
}

impl From<PyIksError> for PyErr {
    fn from(err: PyIksError) -> Self {
        match err {
            PyIksError::NoMemory => PyMemoryError::new_err("pyiks alloc failed"),
            PyIksError::BadXml(msg) => BadXmlError::new_err(msg),
            PyIksError::BadJid(msg) => BadJidError::new_err(msg),
            PyIksError::StreamError(msg) => XmppError::new_err(msg),
            PyIksError::IOError(err) => XmppError::new_err(err),
            PyIksError::TlsError(err) => XmppError::new_err(err),
        }
    }
}

#[pyclass]
struct DocumentChildrenIterator {
    inner: SyncCursor,
}

#[pymethods]
impl DocumentChildrenIterator {
    fn __iter__(&self) -> Self {
        DocumentChildrenIterator {
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
    fn __iter__(&self) -> DocumentChildrenIterator {
        DocumentChildrenIterator {
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

    fn attributes(&self) -> Vec<(String, String)> {
        self.inner.clone().attributes().collect()
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

#[pyclass(name = "XmppClient")]
struct PyXmppClient {
    client: Arc<Mutex<XmppClient>>,
}

// Arc<Mutex<XmppClient>> guarantees PyXmppClient is Send and Sync
unsafe impl Send for PyXmppClient {}
unsafe impl Sync for PyXmppClient {}

#[pymethods]
impl PyXmppClient {
    #[new]
    #[pyo3(signature = (jid, password, debug=false, server=None))]
    fn new(
        py: Python<'_>,
        jid: String,
        password: String,
        debug: bool,
        server: Option<String>,
    ) -> Result<Self, PyIksError> {
        let jid = Jid::new(&jid)?;
        py.detach(|| {
            let client = XmppClient::build(jid, password)
                .debug(debug)
                .server(server)
                .connect()?;
            Ok(Self {
                client: Arc::new(Mutex::new(client)),
            })
        })
    }

    fn wait_for_stanza(&self, py: Python<'_>) -> PyResult<PyDocument> {
        loop {
            let result = py.detach(|| {
                let mut client = self.client.lock().unwrap();
                client.wait_for_stanza_timeout(Some(Duration::from_millis(250)))
            });
            match result {
                Err(e) => return Err(PyIksError::from(e).into()),
                Ok(Some(document)) => {
                    return Ok(PyDocument {
                        inner: SyncCursor::new(document),
                    });
                }
                Ok(None) => {
                    py.check_signals()?;
                }
            }
        }
    }

    fn send_stanza(&self, stanza: &PyDocument) -> Result<(), PyIksError> {
        let mut client = self.client.lock().unwrap();
        client.send_bytes(stanza.inner.to_string().into_bytes())?;
        Ok(())
    }

    fn request_roster(&self) -> Result<(), PyIksError> {
        let mut client = self.client.lock().unwrap();
        client.request_roster()?;
        Ok(())
    }

    fn send_message(&self, to: String, body: String) -> Result<(), PyIksError> {
        let mut client = self.client.lock().unwrap();
        let to_jid = Jid::new(&to)?;
        client.send_message(to_jid, &body)?;
        Ok(())
    }
}

#[pymodule]
fn _pyiks(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    m.add_class::<DocumentChildrenIterator>()?;
    m.add_class::<PyXmppClient>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add("BadXmlError", py.get_type::<BadXmlError>())?;
    m.add("BadJidError", py.get_type::<BadJidError>())?;
    m.add("XmppError", py.get_type::<XmppError>())?;
    Ok(())
}
