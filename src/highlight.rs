//! Minimal line-based syntax highlighter producing HTML `<span>` tokens.
//!
//! Not a full grammar — a pragmatic scanner good enough for short
//! illustrative snippets: comments, strings, numbers, keywords, types
//! (capitalized identifiers) and function calls (identifier followed by `(`).

pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

const KW_C_LIKE: &[&str] = &[
    "if",
    "else",
    "for",
    "while",
    "return",
    "break",
    "continue",
    "new",
    "in",
    "of",
    "do",
    "switch",
    "case",
    "default",
    "try",
    "catch",
    "finally",
    "throw",
    "typeof",
    "instanceof",
    "void",
    "delete",
    "yield",
    "class",
    "extends",
    "super",
    "this",
    "import",
    "export",
    "from",
    "as",
    "async",
    "await",
    "static",
    "function",
    "let",
    "const",
    "var",
    "interface",
    "implements",
    "enum",
    "type",
    "namespace",
    "declare",
    "readonly",
    "public",
    "private",
    "protected",
    "abstract",
    "null",
    "undefined",
    "true",
    "false",
];

const KW_RUST: &[&str] = &[
    "fn", "let", "mut", "pub", "use", "mod", "crate", "impl", "trait", "struct", "enum", "match",
    "if", "else", "for", "while", "loop", "return", "break", "continue", "ref", "move", "async",
    "await", "dyn", "where", "unsafe", "as", "in", "self", "Self", "super", "const", "static",
    "type", "true", "false", "Some", "None", "Ok", "Err",
];

const KW_PYTHON: &[&str] = &[
    "def", "class", "if", "elif", "else", "for", "while", "return", "break", "continue", "import",
    "from", "as", "with", "try", "except", "finally", "raise", "pass", "lambda", "global",
    "nonlocal", "yield", "async", "await", "in", "is", "not", "and", "or", "del", "assert",
    "match", "case", "True", "False", "None", "self",
];

fn keywords_for(lang: &str) -> &'static [&'static str] {
    match lang {
        "rust" | "rs" => KW_RUST,
        "python" | "py" => KW_PYTHON,
        _ => KW_C_LIKE,
    }
}

fn uses_hash_comments(lang: &str) -> bool {
    matches!(
        lang,
        "python" | "py" | "yaml" | "yml" | "toml" | "bash" | "sh" | "shell" | "ruby" | "rb"
    )
}

fn push_span(out: &mut String, class: &str, text: &str) {
    out.push_str("<span class=\"");
    out.push_str(class);
    out.push_str("\">");
    out.push_str(&escape_html(text));
    out.push_str("</span>");
}

/// Highlight a single line of code into HTML. The output is escaped and
/// safe to inject with `|safe`.
pub fn highlight_code_line(line: &str, lang: &str) -> String {
    let keywords = keywords_for(lang);
    let hash_comments = uses_hash_comments(lang);
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        let comment_start = if hash_comments {
            c == '#'
        } else {
            c == '/' && chars.get(i + 1) == Some(&'/')
        };
        if comment_start {
            let rest: String = chars[i..].iter().collect();
            push_span(&mut out, "tok-com", &rest);
            break;
        }

        if c == '"' || c == '\'' || c == '`' {
            let quote = c;
            let mut j = i + 1;
            while j < chars.len() {
                if chars[j] == '\\' {
                    j += 2;
                    continue;
                }
                if chars[j] == quote {
                    j += 1;
                    break;
                }
                j += 1;
            }
            let end = j.min(chars.len());
            let lit: String = chars[i..end].iter().collect();
            push_span(&mut out, "tok-str", &lit);
            i = end;
            continue;
        }

        if c.is_ascii_digit() {
            let mut j = i;
            while j < chars.len()
                && (chars[j].is_ascii_alphanumeric() || chars[j] == '.' || chars[j] == '_')
            {
                j += 1;
            }
            let num: String = chars[i..j].iter().collect();
            push_span(&mut out, "tok-num", &num);
            i = j;
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let mut j = i;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            let word: String = chars[i..j].iter().collect();
            let class = if keywords.contains(&word.as_str()) {
                Some("tok-kw")
            } else if chars.get(j) == Some(&'(') {
                Some("tok-fn")
            } else if c.is_ascii_uppercase() {
                Some("tok-ty")
            } else {
                None
            };
            match class {
                Some(cl) => push_span(&mut out, cl, &word),
                None => out.push_str(&escape_html(&word)),
            }
            i = j;
            continue;
        }

        out.push_str(&escape_html(&c.to_string()));
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_keywords_and_functions() {
        let html = highlight_code_line("const x = compute();", "ts");
        assert!(html.contains("<span class=\"tok-kw\">const</span>"));
        assert!(html.contains("<span class=\"tok-fn\">compute</span>"));
    }

    #[test]
    fn highlights_types_strings_numbers() {
        let html = highlight_code_line("let s: ServiceCollection = \"hi\" + 42;", "rust");
        assert!(html.contains("<span class=\"tok-kw\">let</span>"));
        assert!(html.contains("<span class=\"tok-ty\">ServiceCollection</span>"));
        assert!(html.contains("<span class=\"tok-str\">&quot;hi&quot;</span>"));
        assert!(html.contains("<span class=\"tok-num\">42</span>"));
    }

    #[test]
    fn highlights_line_comments() {
        let html = highlight_code_line("x(); // set the error handler", "ts");
        assert!(html.contains("<span class=\"tok-com\">// set the error handler</span>"));
    }

    #[test]
    fn hash_comments_for_python() {
        let html = highlight_code_line("x = 1  # counter", "python");
        assert!(html.contains("<span class=\"tok-com\"># counter</span>"));
    }

    #[test]
    fn escapes_html_in_plain_text() {
        let html = highlight_code_line("a < b && c > d", "ts");
        assert!(html.contains("&lt;"));
        assert!(html.contains("&gt;"));
        assert!(html.contains("&amp;&amp;"));
        assert!(!html.contains("<b"));
    }

    #[test]
    fn unterminated_string_consumes_rest_of_line() {
        let html = highlight_code_line("say(\"oops", "ts");
        assert!(html.contains("<span class=\"tok-str\">&quot;oops</span>"));
    }
}
