use pulldown_cmark::{html::push_html, Options, Parser};

pub fn render(input: &str) -> String {
    let parser = Parser::new_ext(input, Options::all());
    let mut out = String::new();

    push_html(&mut out, parser);

    out
}
