# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-

## [v0.1.0](https://github.com/trussed-dev/apdu-dispatch/releases/tag/app-0.1.0) (2024-10-18)

- Extract `app` module from `apdu-dispatch` 0.2.0 into a separate crate.
- Replace `iso7816::Command` with `iso7816::command::CommandView` in the `App` trait and remove the `C` parameter.
