use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

const FOO: &'static str = "ABCD";

pub fn highlight(lang: &str, source: &str) -> String {
    let _ = FOO;

    let mut conf = match lang {
        "rust" => HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap(),
        "toml" => HighlightConfiguration::new(
            tree_sitter_toml::language(),
            tree_sitter_toml::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap(),

        _ => panic!("Unknown language: {}", lang),
    };

    let names = conf.query.capture_names().to_owned();
    conf.configure(&names);

    let mut highlighter = Highlighter::new();

    let highlights = highlighter
        .highlight(&conf, source.as_bytes(), None, |_| None)
        .unwrap();

    let classes: Vec<String> = names
        .iter()
        .map(|n| format!("class=\"hl-{}\"", n.replace('.', "-")))
        .collect();

    let mut html = HtmlRenderer::new();
    html.render(highlights, source.as_bytes(), &|i| classes[i.0].as_bytes())
        .unwrap();

    String::from_utf8(html.html).unwrap()
}
