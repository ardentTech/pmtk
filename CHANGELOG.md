# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- combined `Cmd` and `Q` traits

### Removed

- `Request` and `Response` trait

## [0.3.0] - 2026-08-25

### Added

- add locus 184, 185 and 186 commands
- add locus 183, 622 queries

## [0.2.0] - 2026-08-20

### Added

- doc comments

### Changed

- return byte array instead of `heapless::String` in public API
- split `EasyEnable` cmd into discrete q and dt types

## [0.1.0] - 2026-08-14

### Added

- implementations for all PMTK CMDs, DTs and Qs