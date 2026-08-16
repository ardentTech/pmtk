# PMTK

`#![no_std]` implementation of PMTK revision A11 definitions. This includes definitions of the commands, queries and
data types and de/serialization capabilities.

### Cargo Features

* `defmt`: include deferred formatting logging functionality
* `mt3339`: include MT3339-specific commands (see [datasheet](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)).

### Contributing

If you find a bug, or, say, a new version of the PMTK standard comes out:

* fork this repo
* make the necessary changes
* fix and/or expand the unit tests and ensure they pass with: `$ cargo test`
* open a PR

### TODO

- [x] doc comments
- [ ] knock out `TODO`s
- [x] trim error enum
- [x] resolve warnings from `check`
- [x] check for and refactor `unwrap`s
- [ ] opt for consistent use of `fn new` with private struct members?
- [ ] return pub heapless:String vs [u8; N]?

### Resources

* [A11 Datasheet](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)

### License

* [MIT](https://github.com/ardentTech/pmtk/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/pmtk/blob/main/LICENSE-APACHE)