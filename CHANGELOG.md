# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fixed the calculation of the maximum length of a response when deciding
  whether to send it in one or multiple APDUs.

### Added

- Fuzzing infrastructure ([#9][])

[#9]: https://github.com/trussed-dev/apdu-dispatch/pull/9

## [0.1.1] - 2022-08-22
- respect `Le` field @sosthene-nitrokey
