# gstreamer-rs [![crates.io](https://img.shields.io/crates/v/gstreamer.svg)](https://crates.io/crates/gstreamer) [![Build Status](https://travis-ci.org/sdroege/gstreamer-rs.svg?branch=master)](https://travis-ci.org/sdroege/gstreamer-rs)

[GStreamer](https://gstreamer.freedesktop.org/) bindings for Rust.
Documentation can be found [here](https://sdroege.github.io/rustdoc/gstreamer/gstreamer/).

These bindings are providing a safe API that can be used to interface with
GStreamer, e.g. for writing GStreamer-based applications.

For background and motivation, see the [announcement blogpost](https://coaxion.net/blog/2017/07/writing-gstreamer-applications-in-rust/).

The bindings (since 0.8.0) are autogenerated with [gir](https://github.com/gtk-rs/gir/)
based on the [GObject-Introspection](https://wiki.gnome.org/Projects/GObjectIntrospection/)
API metadata provided by the GStreamer project. Older versions before 0.8.0 were manually
written and the repository can be found [here](https://github.com/arturoc/gstreamer1.0-rs).
The API of the two is incompatible.

A crate for writing GStreamer plugins in Rust can be found here: https://github.com/sdroege/gst-plugin-rs

## Table of Contents
1. [Installation](#installation)
   1. [Linux/BSDs](#installation-linux)
   1. [macOS](#installation-macos)
   1. [Windows](#installation-windows)
1. [Getting Started](#getting-started)
1. [License](#license)
1. [Contribution](#contribution)

<a name="installation"/>

## Installation

To build the GStreamer bindings or anything depending on them, you need to
have at least GStreamer 1.8 and gst-plugins-base 1.8 installed. In addition,
some of the examples/tutorials require various GStreamer plugins to be
available, which can be found in gst-plugins-base, gst-plugins-good,
gst-plugins-bad, gst-plugins-ugly and/or gst-libav.

<a name="installation-linux"/>

### Linux/BSDs

You need to install the above mentioned packages with your distributions
package manager, or build them from source.

On Debian/Ubuntu they can be installed with

```
$ apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev
```

Package names on other distributions should be similar.
Please submit a pull request with instructions for yours.

<a name="installation-macos"/>

### macOS

You can install GStreamer and the plugins via [Homebrew](https://brew.sh/) or
by installing the [binaries](https://gstreamer.freedesktop.org/data/pkg/osx/)
provided by the GStreamer project.

#### Homebrew

```
$ brew install gstreamer gst-plugins-base gst-plugins-good \
      gst-plugins-bad gst-plugins-ugly gst-libav gst-rtsp-server
```

#### GStreamer Binaries

You need to download the *two* `.pkg` files from the GStreamer website and
install them, e.g. `gstreamer-1.0-1.12.3-x86_64.pkg` and
`gstreamer-1.0-devel-1.12.3-x86_64.pkg`.

After installation, you also need to install `pkg-config` (e.g. via Homebrew)
and set the `PKG_CONFIG_PATH` environment variable

```
$ export PKG_CONFIG_PATH="/Library/Frameworks/GStreamer.framework/Versions/Current/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

<a name="installation-windows"/>

### Windows

You can install GStreamer and the plugins via [MSYS2](http://www.msys2.org/)
with `pacman` or by installing the
[binaries](https://gstreamer.freedesktop.org/data/pkg/windows/) provided by
the GStreamer project.

#### MSYS2 / pacman

```
$ pacman -S pkg-config mingw-w64-x86_64-gstreamer mingw-w64-x86_64-gst-plugins-base \
      mingw-w64-x86_64-gst-plugins-good mingw-w64-x86_64-gst-plugins-bad \
      mingw-w64-x86_64-gst-plugins-ugly mingw-w64-x86_64-gst-libav \
      mingw-w64-x86_64-gst-rtsp-server
```

#### GStreamer Binaries

You need to download the *two* `.msi` files for your platform from the
GStreamer website and install them, e.g. `gstreamer-1.0-x86_64-1.12.3.msi` and
`gstreamer-1.0-devel-x86_64-1.12.3.msi`.

After installation, you also need to install `pkg-config` (e.g. via MSYS2 or
from [here](https://sourceforge.net/projects/pkgconfiglite/))
and set the `PKG_CONFIG_PATH` environment variable

```
$ export PKG_CONFIG_PATH="c:\\gstreamer\\1.0\\x86_64\\lib\\pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
```

<a name="getting-started"/>

## Getting Started

The API reference can be found
[here](https://sdroege.github.io/rustdoc/gstreamer/gstreamer/), however it is
only the Rust API reference and does not explain any of the concepts.

For getting started with GStreamer development, the best would be to follow
the [documentation](https://gstreamer.freedesktop.org/documentation/) on the
GStreamer website, especially the [Application Development
Manual](https://gstreamer.freedesktop.org/documentation/application-development/).
While being C-centric, it explains all the fundamental concepts of GStreamer
and the code examples should be relatively easily translatable to Rust. The
API is basically the same, function/struct names are the same and everything
is only more convenient (hopefully) and safer.

In addition there are
[tutorials](https://gstreamer.freedesktop.org/documentation/tutorials/) on the
GStreamer website. Many of them were ported to Rust already and the code can
be found in the
[tutorials](https://github.com/sdroege/gstreamer-rs/tree/master/tutorials)
directory.

Some further examples for various aspects of GStreamer and how to use it from
Rust can be found in the
[examples](https://github.com/sdroege/gstreamer-rs/tree/master/examples)
directory.

<a name="license"/>

## LICENSE

gstreamer-rs and all crates contained in here are licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

GStreamer itself is licensed under the Lesser General Public License version
2.1 or (at your option) any later version:
https://www.gnu.org/licenses/lgpl-2.1.html

<a name="contribution"/>

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gstreamer-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
