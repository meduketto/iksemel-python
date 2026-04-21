# This file is a part of Iksemel (XML parser for Jabber/XMPP)
# Copyright (C) 2000-2026 Gurer Ozen
#
# Iksemel is free software: you can redistribute it and/or modify it
# under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of
# the License, or (at your option) any later version.

import pyiks
import pytest


def test_parse():
    xml = "<a>lala</a>"
    doc = pyiks.parse(xml)
    assert str(doc) == xml


def test_parse_error():
    with pytest.raises(pyiks.BadXmlError):
        pyiks.parse(b"<<>")


def test_build():
    doc = pyiks.Document("a")
    doc.insert_tag("b").insert_cdata("lala").parent().set_attribute("x", "123")
    assert str(doc) == '<a><b x="123">lala</b></a>'


def test_edits():
    doc = pyiks.Document("a")
    t = doc.insert_tag("b")
    after = t.append_tag("after")
    before = t.prepend_tag("before")
    after.append_cdata("123")
    before.prepend_cdata("456")
    t.insert_attribute("id", "@")
    assert str(doc) == '<a>456<before/><b id="@"/><after/>123</a>'
    with pytest.raises(pyiks.BadXmlError):
        t.insert_attribute("id", "@")
    t.set_attribute("id", None)
    assert str(doc) == "<a>456<before/><b/><after/>123</a>"
    t.set_attribute("id", "$")
    after.remove()
    assert str(doc) == '<a>456<before/><b id="$"/>123</a>'
    t.set_attribute("id", "%")
    before.previous().remove()
    assert str(doc) == '<a><before/><b id="%"/>123</a>'


def test_navigation():
    doc = pyiks.parse(
        "<a><b>123<d>456</d>789<f id='xyz'/></b>012<c>345<e>678</e>901</c></a>"
    )
    assert doc.first_child().parent().name() == "a"
    assert doc.first_child().first_child().root().name() == "a"
    assert doc.find_tag("b").find_tag("f").attribute("id") == "xyz"
    assert doc.first_child().name() == "b"
    assert doc.first_child().first_tag().next_tag().attribute("id") == "xyz"
    assert str(doc.last_child().last_child().previous_tag()) == "<e>678</e>"
    assert doc.first_child().last_child().previous().cdata() == "789"
    assert doc.last_child().first_tag().next().cdata() == "901"
    assert doc.first_child().is_tag()
    assert not doc.first_child().next().is_tag()
    assert doc.find_tag("lala").is_null()


def test_iter():
    doc = pyiks.parse("<a><b/><c/><d/></a>")
    assert [tag.name() for tag in doc] == ["b", "c", "d"]


def test_attributes():
    # Multiple attributes are returned in insertion order
    doc = pyiks.parse('<a x="1" y="2" z="3"/>')
    assert doc.attributes() == [("x", "1"), ("y", "2"), ("z", "3")]

    # A tag with no attributes returns an empty list
    doc = pyiks.parse("<a/>")
    assert doc.attributes() == []

    # A non-tag (cdata) node returns an empty list
    doc = pyiks.parse("<a>hello</a>")
    cdata = doc.first_child()
    assert not cdata.is_tag()
    assert cdata.attributes() == []
