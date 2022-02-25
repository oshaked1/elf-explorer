# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [[Unreleased]](https://github.com/oshaked1/elf-explorer/compare/v0.3.0...HEAD)

### Added

- A **Section Headers** section was added.

### Fixed

- Using arrow keys for navigation now works as intended ([#1](https://github.com/oshaked1/elf-explorer/issues/1)).

## [[0.3.0]](https://github.com/oshaked1/elf-explorer/compare/v0.2.0...v0.3.0) - 2022-02-19

### Added

- A **Program Headers** section was added.

## [[0.2.0]](https://github.com/oshaked1/elf-explorer/compare/v0.1.0...v0.2.0) - 2022-02-13

### Added

- The **ELF Header** section now includes all of the ELF header's fields.
- A text box at the bottom of the screen was added. It displays a brief description of the current selected item, as well as some startup tips.

### Changed

- Most fields are now added using a set of macros for common field types.
- Layout items which need to set the description in the text box (see the added section) are no longer implemented using the partials UI of nwg. Instead, they are placed directly into the main app struct.

## [[0.1.0]](https://github.com/oshaked1/elf-explorer/releases/tag/v0.1.0) - 2022-02-07

Initial release

### Added
- GUI window with 2 main sections - navigation bar on the left, and information view on the right.
- Only **ELF Header** is available in the navigation menu.

- **ELF Header** information view includes only the `e_ident` field.
- Selecting the `e_ident` field show an additional information view at the bottom of the main information view. It includes the all of the fields of `e_ident`.

- ELF files can be opened by dragging them into the window, of using **File -> Open** dialog in the menu bar at the top.