use proc_macro::TokenStream;
use std::io::Write;
use syn::{parse_macro_input, LitStr};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[proc_macro]
pub fn yellow(input: TokenStream) -> TokenStream {
    do_alert(Color::Yellow, input)
}

#[proc_macro]
pub fn red(input: TokenStream) -> TokenStream {
    do_alert(Color::Red, input)
}

fn do_alert(color: Color, input: TokenStream) -> TokenStream {
    let message = parse_macro_input!(input as LitStr).value();

    let ref mut stderr = StandardStream::stderr(ColorChoice::Auto);
    let color_spec = ColorSpec::new().set_fg(Some(color)).clone();

    for line in message.lines() {
        let _ = stderr.set_color(&color_spec);
        let _ = writeln!(stderr, "{}", line);
    }

    let _ = stderr.reset();
    let _ = writeln!(stderr);

    TokenStream::new()
}
