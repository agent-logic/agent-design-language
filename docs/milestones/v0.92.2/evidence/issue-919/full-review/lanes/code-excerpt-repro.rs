type Result<T> = std::result::Result<T,String>;
macro_rules! ensure { ($cond:expr, $msg:expr $(,)?) => { if !$cond { return Err($msg.into()); } }; }
fn unsafe_content(_: &str, _: &str) -> bool { false }
fn markdown_text(value: &str) -> Result<String> {
    ensure!(
        !value.is_empty() && value.len() <= 64 * 1024,
        "markdown_text_empty_or_too_large"
    );
    ensure!(
        !unsafe_content("", value),
        "markdown_text_failed_redaction_recheck"
    );
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        ensure!(
            !character.is_control() || matches!(character, '\n' | '\r' | '\t'),
            "markdown_text_contains_control_character"
        );
        match character {
            '\n' | '\r' | '\t' => output.push(' '),
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '<' | '>' | '(' | ')' | '#' | '+'
            | '-' | '.' | '!' | '|' => {
                output.push('\\');
                output.push(character);
            }
            _ => output.push(character),
        }
    }
    Ok(output)
}

fn html_text(value: &str) -> Result<String> {
    ensure!(
        !value.is_empty() && value.len() <= 64 * 1024,
        "html_text_empty_or_too_large"
    );
    ensure!(
        !unsafe_content("", value),
        "html_text_failed_redaction_recheck"
    );
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        ensure!(
            !character.is_control() || matches!(character, '\n' | '\r' | '\t'),
            "html_text_contains_control_character"
        );
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            '\n' | '\r' | '\t' => output.push(' '),
            _ => output.push(character),
        }
    }
    Ok(output)
}

fn main() {
 let quote = "if authorized:\n\tdelete_records()\nreturn ok";
 let md=markdown_text(quote).unwrap(); let html=html_text(quote).unwrap();
 println!("original={quote:?}\nmarkdown={md:?}\nhtml={html:?}");
 assert!(!md.contains('\n') && !md.contains('\t'));
 assert!(!html.contains('\n') && !html.contains('\t'));
 println!("REPRODUCED: both exact-source-excerpt renderers erase line breaks and indentation");
}
