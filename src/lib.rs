//! Display a message in the Cargo build output during compilation.
//!
//! # Example
//!
//! ```
//! #[cfg(debug_assertions)]
//! build_alert::yellow! {"
//! NOTE:  use --release
//!   Syn's test suite has some tests that run on every source file
//!   and test case in the rust-lang/rust repo, which can be pretty
//!   slow in debug mode. Consider running cargo test with `--release`
//!   to speed things up.
//! "}
//!
//! #[cfg(not(feature = "all-features"))]
//! build_alert::red! {"
//! ERROR:  use --all-features
//!   Syn's test suite normally only works with all-features enabled.
//!   Run again with `--all-features`, or run with `--features test`
//!   to bypass this check.
//! "}
//! ```
//!
//! ![screenshot](https://user-images.githubusercontent.com/1940490/227811885-3eca7b65-0425-4be5-aa1a-cf52d8014817.png)

use proc_macro::TokenStream;
use std::io::Write;
use std::process;
use syn::{parse_macro_input, LitStr};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// For <kbd>NOTE:</kbd> or <kbd>WARNING:</kbd> alerts.
#[proc_macro]
pub fn yellow(input: TokenStream) -> TokenStream {
    do_alert(Color::Yellow, input)
}

/// For <kbd>ERROR:</kbd> alerts.
#[proc_macro]
pub fn red(input: TokenStream) -> TokenStream {
    do_alert(Color::Red, input)
}

fn do_alert(color: Color, input: TokenStream) -> TokenStream {
    let message = parse_macro_input!(input as LitStr).value();

    let ref mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let color_spec = ColorSpec::new().set_fg(Some(color)).clone();
    let mut has_nonspace = false;
    let mut says_error = false;

    for mut line in message.lines() {
        if !has_nonspace {
            let (indent, heading, rest) = split_heading(line);
            if let Some(indent) = indent {
                let _ = write!(stderr, "{}", indent);
            }
            if let Some(heading) = heading {
                let _ = stderr.set_color(color_spec.clone().set_bold(true));
                let _ = write!(stderr, "{}", heading);
                has_nonspace = true;
                says_error = heading == "ERROR";
            }
            line = rest;
        }
        if !line.is_empty() {
            let _ = stderr.set_color(&color_spec);
            let _ = writeln!(stderr, "{}", line);
            has_nonspace = has_nonspace || line.contains(|ch: char| ch != ' ');
        } else {
            let _ = writeln!(stderr);
        }
    }

    let _ = stderr.reset();
    let _ = writeln!(stderr);

    if color == Color::Red && says_error {
        process::exit(1);
    } else {
        TokenStream::new()
    }
}

fn split_heading(s: &str) -> (Option<&str>, Option<&str>, &str) {
    let mut start = 0;
    while start < s.len() && s[start..].starts_with(' ') {
        start += 1;
    }

    let mut end = start;
    while end < s.len() && s[end..].starts_with(|ch: char| ch.is_ascii_uppercase()) {
        end += 1;
    }

    if end - start >= 3 && (end == s.len() || s[end..].starts_with(':')) {
        let indent = (start > 0).then_some(&s[..start]);
        let heading = &s[start..end];
        let rest = &s[end..];
        (indent, Some(heading), rest)
    } else {
        (None, None, s)
    }
}
