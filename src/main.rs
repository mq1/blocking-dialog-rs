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
    -h, --help             Print this help message

EXAMPLES:
    # Show a basic info dialog
    blocking-dialog \"Hello, World!\"

    # Show an error dialog with custom title
    blocking-dialog --error --title \"Error\" \"Something went wrong!\"

    # Show a warning dialog
    blocking-dialog -w -t \"Warning\" \"Proceed with caution\"\
";

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
