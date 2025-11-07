# This file is a part of Iksemel (XML parser for Jabber/XMPP)
# Copyright (C) 2000-2025 Gurer Ozen
#
# Iksemel is free software: you can redistribute it and/or modify it
# under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of
# the License, or (at your option) any later version.

import pytest

import pyiks


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


def test_iter():
    doc = pyiks.parse("<a><b/><c/><d/></a>")
    assert [tag.name() for tag in doc] == ["b", "c", "d"]
