# pyiks

Copyright (c) 2025 Gurer Ozen <meduketto at gmail.com>

[pyiks][pyiks] is a Python binding for the [iksemel][iksemel]
which is an XML parser library for Jabber/XMPP and
general XML processing applications written in Rust.
Iksemel aims to be easy to use, fast, and usable in
resource-constrained environments.

# Features

pyiks only provides the DOM interface of iksemel with this
first release.

# Usage

Here is a simple example showing parsing and editing:

```python
xml_text = "<doc><a>123</a><b><a>456</a><a>789</a></b></doc>"

doc = pyiks.parse(xml_text)

doc.find_tag("b").first_tag().remove()
doc.find_tag("a").set_attribute("x", "1")

assert str(doc) == '<doc><a x="1">123</a><b><a>789</a></b></doc>'
```

# License

Iksemel is free software: you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

Iksemel is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public
License for more details.

You should have received a copy of the GNU Lesser General Public License
along with Iksemel. If not, see <https://www.gnu.org/licenses/>.


[iksemel]: https://github.com/meduketto/iksemel-rust
[pyiks]: https://github.com/meduketto/iksemel-python
