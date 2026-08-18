# PMTK

`#![no_std]` implementation of PMTK (rev. A11) definitions for the MT3318, MT3329 and MT3339 MediaTek chips. This
includes definitions of the commands, queries and data types with de/serialization functionality but for simplicity does
not include higher-level abstractions.

### Cargo Features

* `defmt`: include deferred formatting logging functionality
* `mt3339`: include MT3339-specific definitions (see [datasheet](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)).

### Contributing

If you find a bug, or, say, a new version of the PMTK standard comes out:

* fork this repo
* make the necessary changes
* fix and/or expand the unit tests and ensure they pass with: `$ cargo test -F mt3339`
* open a PR

### TODO

- [x] doc comments
- [ ] knock out `TODO`s
- [x] trim error enum
- [x] resolve warnings from `check`
- [x] check for and refactor `unwrap`s
- [x] opt for consistent use of `fn new` with private struct members on cmd
- [ ] return pub heapless:String vs [u8; N]?
- [ ] refactor 255 literals to const

### Resources

* [A11 Datasheet](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)

### License

* [MIT](https://github.com/ardentTech/pmtk/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/pmtk/blob/main/LICENSE-APACHE)