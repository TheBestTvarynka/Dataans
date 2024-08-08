use std::sync::LazyLock;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static THEMES: LazyLock<ThemeSet> = LazyLock::new(|| ThemeSet::load_defaults());
static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(|| SyntaxSet::load_defaults_newlines());

#[tauri::command]
pub fn parse_code(lang: String, code: String) -> String {
    let syntax = if let Some(syntax) = SYNTAXES.find_syntax_by_name(&lang) {
        syntax
    } else if let Some(syntax) = SYNTAXES.find_syntax_by_extension(&lang) {
        syntax
    } else {
        SYNTAXES
            .find_syntax_by_extension("txt")
            .expect("The default plain text syntax should present.")
    };

    let html_rs = highlighted_html_for_string(&code, &SYNTAXES, syntax, &THEMES.themes["Solarized (dark)"])
        .expect("Code HTML generation should not fail.");

    html_rs
}
