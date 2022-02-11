# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [[Unreleased]](https://github.com/oshaked1/elf-explorer/compare/v0.1.0...HEAD)

### Added

- The **ELF Header** section now includes all of the ELF header's fields.

### Changed

- Most fields are now added using a set of macros for common field types.

## [[0.1.0]](https://github.com/oshaked1/elf-explorer/releases/tag/v0.1.0) - 2022-02-07

Initial release

### Added
- GUI window with 2 main sections - navigation bar on the left, and information view on the right.
- Only **ELF Header** is available in the navigation menu.

- **ELF Header** information view includes only the `e_ident` field.
- Selecting the `e_ident` field show an additional information view at the bottom of the main information view. It includes the all of the fields of `e_ident`.

- ELF files can be opened by dragging them into the window, of using **File -> Open** dialog in the menu bar at the top.