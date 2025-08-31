use std::sync::LazyLock;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static THEMES: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(|| {
    use syntect::dumps::from_uncompressed_data;

    from_uncompressed_data(include_bytes!("../resources/assets/default_newlines.packdump"))
        .expect("syntax loading should not fail")
});

/// Parsed the code block value and returns generated HTML for this code.
#[tauri::command]
pub fn parse_code(lang: String, code: String) -> String {
    let syntax = if let Some(syntax) = SYNTAXES.find_syntax_by_token(&lang) {
        syntax
    } else {
        SYNTAXES.find_syntax_plain_text()
    };

    highlighted_html_for_string(&code, &SYNTAXES, syntax, &THEMES.themes["Solarized (dark)"])
        .expect("Code HTML generation should not fail.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntaxes_loading() {
        SYNTAXES.syntaxes().iter().for_each(|s| {
            println!("=> {:?} {:?}", s.name, s.file_extensions);
        });
    }
}
