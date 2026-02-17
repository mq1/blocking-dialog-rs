// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod alert;
pub mod confirm;
pub mod pick_directory;
pub mod pick_files;
pub mod save_file;

pub fn widen<S: AsRef<str>>(s: S) -> Vec<u16> {
    s.as_ref()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

pub fn unwiden<I: IntoIterator<Item = u16>>(s: I) -> String {
    let vec = s.into_iter().collect::<Vec<u16>>();
    let s = String::from_utf16_lossy(&vec);
    s.trim_matches(char::from(0)).to_string()
}
