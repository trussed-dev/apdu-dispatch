# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0]

### Fixed

- Fixed the calculation of the maximum length of a response when deciding
  whether to send it in one or multiple APDUs. ([#10][])
- Return an error instead of panicking for invalid aids in `Select` commands ([#8][])

### Changed

- Use `rustfmt` and `clippy` ([#11][])

### Added

- Fuzzing infrastructure ([#9][])

[#8]: https://github.com/trussed-dev/apdu-dispatch/pull/8
[#9]: https://github.com/trussed-dev/apdu-dispatch/pull/9
[#10]: https://github.com/trussed-dev/apdu-dispatch/pull/10
[#11]: https://github.com/trussed-dev/apdu-dispatch/pull/11

## [0.1.1] - 2022-08-22

- respect `Le` field @sosthene-nitrokey

## [0.1.0] - 2022-03-05

- Initial release


[Unreleased]: https://github.com/trussed-dev/apdu-dispatch/compare/0.1.2...HEAD
[0.1.2]: https://github.com/trussed-dev/apdu-dispatch/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/trussed-dev/apdu-dispatch/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/trussed-dev/apdu-dispatch/releases/tag/0.1.0
