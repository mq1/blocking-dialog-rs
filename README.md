# blocking-dialog-rs

[![Latest version](https://img.shields.io/crates/v/blocking-dialog.svg)](https://crates.io/crates/blocking-dialog)
[![Documentation](https://docs.rs/blocking-dialog/badge.svg)](https://docs.rs/blocking-dialog)
![License](https://img.shields.io/crates/l/blocking-dialog.svg)

Minimal, simple, opinionated desktop dialog implementation (wip)

Examples: <https://github.com/mq1/TinyWiiBackupManager/blob/main/src/ui/dialogs.rs>

### Goals and non-goals

- Windows XP+ support (using the windows crate)
- macOS support (using the objc2 crate)
- Linux support (using the zenity-rs crate)
- No async
- Few dependencies
- Simple api
