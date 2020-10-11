//! The custom panic hook used by the linter to issue a more descriptive explanation.

use crate::{lint_err, lint_note};
use codespan_reporting::term::termcolor::{self, ColorChoice};
use std::io::Write;
use std::panic::PanicInfo;

pub fn panic_hook(info: &PanicInfo) {
    lint_err!("The linter panicked unexpectedly. this is a bug.");

    let stderr = termcolor::StandardStream::stderr(ColorChoice::Always);

    let mut stderr_lock = stderr.lock();

    let mut write = |msg: &str| {
        writeln!(stderr_lock, "{}", msg).expect("panic_hook failed to write to stderr");
    };

    write("We would appreciate a bug report: https://github.com/RDambrosio016/RSLint/issues/new?labels=ILE%2C+bug&template=internal-linter-error.md\n");

    lint_note!("Please include the following info: ");

    let msg = info
        .payload()
        .downcast_ref::<String>()
        .map(|x| x.to_string())
        .unwrap_or_default();

    let location = info
        .location()
        .map(|l| format!("{}", l))
        .unwrap_or_default();

    write(format!("message: {}", msg).as_str());
    write(format!("location: {}", location).as_str());
    std::process::exit(-1);
}
