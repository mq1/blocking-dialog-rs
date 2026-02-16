// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "cli")]
use blocking_dialog::BlockingDialogLevel;

#[cfg(feature = "cli")]
const HELP: &str = "\
blocking-dialog - Display system dialog boxes from the command line

USAGE:
    blocking-dialog [OPTIONS] <MESSAGE>

ARGS:
    <MESSAGE>    The message text to display in the dialog box

OPTIONS:
    -t, --title <TITLE>    Set the dialog title [default: BlockingDialog]
    -i, --info             Show an information dialog (default)
    -w, --warning          Show a warning dialog
    -e, --error            Show an error dialog
    -a, --alert            Show an alert dialog (default)
    -c, --confirm          Show a confirmation dialog
    -h, --help             Print this help message

EXAMPLES:
    blocking-dialog \"Hello world\"
    blocking-dialog -et \"An error\" \"Hello world\"
    blocking-dialog -ct \"A confirmation\" \"Hello world\"
";

#[cfg(feature = "cli")]
enum DialogKind {
    Alert,
    Confirm,
}

#[cfg(feature = "cli")]
struct Args {
    level: BlockingDialogLevel,
    title: String,
    message: String,
    dialog_kind: DialogKind,
}

#[cfg(feature = "cli")]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut level = BlockingDialogLevel::Info;
    let mut title = "BlockingDialog".to_string();
    let mut message = String::new();
    let mut dialog_kind = DialogKind::Alert;

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
            Short('c') | Long("confirm") => {
                dialog_kind = DialogKind::Confirm;
            }
            Short('a') | Long("alert") => {
                dialog_kind = DialogKind::Alert;
            }
            Short('h') | Long("help") => {
                println!("{HELP}");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        level,
        title,
        message,
        dialog_kind,
    })
}

#[cfg(feature = "cli")]
fn main() -> Result<(), lexopt::Error> {
    use blocking_dialog::BlockingAlertDialog;

    let args = parse_args()?;

    match args.dialog_kind {
        DialogKind::Alert => {
            let dialog = BlockingAlertDialog {
                window: None,
                title: &args.title,
                message: &args.message,
                level: args.level,
            };

            dialog.show().expect("Failed to show dialog");
        }
        DialogKind::Confirm => {
            let dialog = blocking_dialog::BlockingConfirmDialog {
                window: None,
                title: &args.title,
                message: &args.message,
                level: args.level,
            };

            let result = dialog.show().expect("Failed to show dialog");
            println!("{}", result);
        }
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("Please add the `cli` feature to enable the CLI");
}
