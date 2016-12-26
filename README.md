# smbclient-sys -- `libsmbclient` bindings

[![Crates.io](https://img.shields.io/crates/v/smbclient-sys.svg)](https://crates.io/crates/smbclient-sys)
[![Build Status](https://travis-ci.org/smbc-rs/smbclient-sys.svg?branch=master)](https://travis-ci.org/smbc-rs/smbclient-sys)
[![Crates.io](https://img.shields.io/crates/l/smbclient-sys.svg)](https://crates.io/crates/smbclient-sys)
[![Gitter](https://img.shields.io/gitter/room/smbc-rs/general.svg)](https://gitter.im/smbc-rs/general)

## About

`smbclient-sys` is bindings for `libsmbclient` from [Samba][samba] project.

Bindings are generated from `libsmbclient.h` header file which should be:

* discoverable by `pkg-config` tool;
* or by setting both `SMBCLIENT_INCLUDE_PATH` and `SMBCLIENT_LIBRARY_PATH`
  (takes precedence over `pkg-config`).


## License

Licensed under [GNU General Public License][gpl] version 3 or any later version.
It can be found at [COPYING](COPYING) or at [GNU][gpl] site.


[gpl]: https://www.gnu.org/licenses/gpl.txt
[samba]: https://www.samba.org
