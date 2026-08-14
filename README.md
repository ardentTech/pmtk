# PMTK

`#![no_std]` implementation of PMTK revision A11 definitions. This includes definitions of the commands, queries and
data types and de/serialization capabilities.

### Cargo Features

* `defmt`: include deferred formatting logging functionality
* `mt3339`: include MT3339-specific commands.

### TODO

- [ ] document
- [ ] knock out `TODO`s
- [x] trim error enum
- [x] resolve warnings from `check`
- [ ] opt for consistent use of `fn new` with private struct members?
- [ ] revisit errors and consolidate if possible
- [ ] return pub heapless:String vs [u8; N]

### Resources
* [A11 Datasheet](https://cdn-shop.adafruit.com/datasheets/PMTK_A11.pdf)

### License
* [MIT](https://github.com/ardentTech/pmtk/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/pmtk/blob/main/LICENSE-APACHE)