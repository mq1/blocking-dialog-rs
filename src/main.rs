// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "cli")]
use blocking_dialog::BlockingDialogLevel;

#[cfg(feature = "cli")]
struct Args {
    level: BlockingDialogLevel,
    title: String,
    message: String,
}

#[cfg(feature = "cli")]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut level = BlockingDialogLevel::Info;
    let mut title = "BlockingDialog".to_string();
    let mut message = String::new();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                message = val.string()?;
            }
            Short('t') | Long("title") => {
                title = parser.value()?.string()?;
            }
            Short('i') | Long("info") => {
                level = BlockingDialogLevel::Info;
            }
            Short('w') | Long("warning") => {
                level = BlockingDialogLevel::Warning;
            }
            Short('e') | Long("error") => {
                level = BlockingDialogLevel::Error;
            }
            Short('h') | Long("help") => {
                println!("Usage: which-fs PATH");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        level,
        title,
        message,
    })
}

#[cfg(feature = "cli")]
fn main() -> Result<(), lexopt::Error> {
    use blocking_dialog::BlockingAlertDialog;

    let args = parse_args()?;

    let dialog = BlockingAlertDialog {
        window: None,
        title: &args.title,
        message: &args.message,
        level: args.level,
    };

    dialog.show().expect("Failed to show dialog");

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Please add the `cli` feature to enable the CLI");
}
