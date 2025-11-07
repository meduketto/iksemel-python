# This file is a part of Iksemel (XML parser for Jabber/XMPP)
# Copyright (C) 2000-2025 Gurer Ozen
#
# Iksemel is free software: you can redistribute it and/or modify it
# under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of
# the License, or (at your option) any later version.

import pytest

import iks


def test_parse():
    xml = b"<a>lala</a>"
    doc = iks.parse(xml)
    assert str(doc) == xml.decode("utf-8")


def test_parse_error():
    with pytest.raises(iks.BadXmlError):
        iks.parse(b"<<>")


def test_build():
    doc = iks.Document("a")
    doc.insert_tag("b").insert_cdata("lala").parent().set_attribute("x", "123")
    assert str(doc) == '<a><b x="123">lala</b></a>'
