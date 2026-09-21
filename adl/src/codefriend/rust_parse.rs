//! Resource admission for inert Rust syntax, shared by architecture and fitness.
//! Parsing, visiting and recursive AST destruction stay on the bounded worker stack.
use proc_macro2::{TokenStream, TokenTree};
use std::cell::Cell;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

pub(crate) const MAX_SOURCE_BYTES: usize = 400 * 1024;
const MAX_TOKENS: usize = 32 * 1024;
const MAX_DEPTH: usize = 32;
const MAX_PATH_UNITS: usize = 2048;
const STACK_BYTES: usize = 64 * 1024 * 1024;
static PARSER: Mutex<()> = Mutex::new(());
static ADMITTED: AtomicUsize = AtomicUsize::new(0);
thread_local! { static IN_WORKER: Cell<bool> = const { Cell::new(false) }; }
struct Admission;
impl Drop for Admission {
    fn drop(&mut self) {
        ADMITTED.fetch_sub(1, Ordering::Release);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Error {
    Resource,
    Syntax,
}

// Before proc_macro2 sees recursive token groups, bound lexical delimiters with
// an iterative scanner. Comments and literals are opaque, never syntax nesting.
fn lexical(source: &str) -> Result<(), Error> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(Error::Resource);
    }
    let b = source.as_bytes();
    let mut i = 0;
    let mut groups = Vec::with_capacity(MAX_DEPTH);
    while i < b.len() {
        if b[i..].starts_with(b"//") {
            i += 2;
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b[i..].starts_with(b"/*") {
            i += 2;
            let mut depth = 1;
            while i < b.len() && depth > 0 {
                if b[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                    if depth > MAX_DEPTH {
                        return Err(Error::Resource);
                    }
                } else if b[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if depth != 0 {
                return Err(Error::Syntax);
            }
            continue;
        }
        let raw = if b[i] == b'r' {
            Some(i + 1)
        } else if b[i..].starts_with(b"br") || b[i..].starts_with(b"cr") {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut quote) = raw {
            let start = quote;
            while quote < b.len() && b[quote] == b'#' {
                quote += 1;
            }
            if quote < b.len() && b[quote] == b'"' {
                let hashes = quote - start;
                if hashes > 255 {
                    return Err(Error::Resource);
                }
                i = quote + 1;
                loop {
                    if i >= b.len() {
                        return Err(Error::Syntax);
                    }
                    if b[i] == b'"'
                        && b.get(i + 1..i + 1 + hashes)
                            .is_some_and(|v| v.iter().all(|c| *c == b'#'))
                    {
                        i += 1 + hashes;
                        break;
                    }
                    i += 1;
                }
                continue;
            }
        }
        if b[i] == b'"' {
            i += 1;
            loop {
                if i >= b.len() {
                    return Err(Error::Syntax);
                }
                if b[i] == b'"' {
                    i += 1;
                    break;
                }
                if b[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            continue;
        }
        if b[i] == b'\'' {
            // A char literal has one scalar/escape followed by a quote. Otherwise
            // this is a lifetime; leave its following identifier to the scanner.
            let mut end = i + 1;
            if b.get(end) == Some(&b'\\') {
                end += 1;
                match b.get(end) {
                    Some(b'u') if b.get(end + 1) == Some(&b'{') => {
                        end += 2;
                        while end < b.len() && b[end] != b'}' {
                            end += 1;
                        }
                        end += 1;
                    }
                    Some(b'x') => end += 3,
                    Some(_) => end += 1,
                    None => return Err(Error::Syntax),
                }
            } else if end < b.len() {
                // i starts at an ASCII quote, so the next position is a UTF-8 boundary.
                end += source[end..]
                    .chars()
                    .next()
                    .ok_or(Error::Syntax)?
                    .len_utf8();
            }
            if b.get(end) == Some(&b'\'') {
                i = end + 1;
                continue;
            }
        }
        // Consume identifiers/numbers as a unit. Recognizing a raw prefix in
        // the middle of an identifier could hide real delimiters from preflight.
        if b[i].is_ascii_alphanumeric() || b[i] == b'_' || b[i] >= 128 {
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_' || b[i] >= 128) {
                i += 1;
            }
            continue;
        }
        match b[i] {
            b'(' | b'[' | b'{' => {
                if groups.len() == MAX_DEPTH {
                    return Err(Error::Resource);
                }
                groups.push(b[i]);
            }
            b')' | b']' | b'}' => {
                let expected = match b[i] {
                    b')' => b'(',
                    b']' => b'[',
                    _ => b'{',
                };
                if groups.pop() != Some(expected) {
                    return Err(Error::Syntax);
                }
            }
            _ => {}
        }
        i += 1;
    }
    if groups.is_empty() {
        Ok(())
    } else {
        Err(Error::Syntax)
    }
}

fn token_budget(
    tokens: TokenStream,
    depth: usize,
    ancestors: usize,
    total: &mut usize,
) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(Error::Resource);
    }
    let mut longest = 0;
    let mut segment = 0;
    for token in tokens.clone() {
        *total += 1;
        segment += 1;
        longest = longest.max(segment);
        if *total > MAX_TOKENS || ancestors + longest > MAX_PATH_UNITS {
            return Err(Error::Resource);
        }
        if matches!(token, TokenTree::Punct(ref p) if p.as_char() == ';') {
            segment = 0;
        }
    }
    for token in tokens {
        if let TokenTree::Group(group) = token {
            token_budget(group.stream(), depth + 1, ancestors + longest, total)?;
        }
    }
    Ok(())
}

pub(crate) fn inspect<T: Send, F: FnOnce(&syn::File) -> T + Send>(
    source: &str,
    visitor: F,
) -> Result<T, Error> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(Error::Resource);
    }
    let body = if source.starts_with("#!") && !source.starts_with("#![") {
        source.split_once('\n').map_or("", |(_, body)| body)
    } else {
        source
    };
    lexical(body)?;
    if IN_WORKER.with(Cell::get) {
        return Err(Error::Resource);
    }
    ADMITTED
        .fetch_update(Ordering::Acquire, Ordering::Relaxed, |n| {
            (n < 32).then_some(n + 1)
        })
        .map_err(|_| Error::Resource)?;
    let _admission = Admission;
    // At most one parser stack/AST per process. No source code or macros execute.
    let _guard = PARSER.lock().map_err(|_| Error::Resource)?;
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .name("codefriend-rust-syntax".into())
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, move || {
                IN_WORKER.with(|flag| flag.set(true));
                let tokens: TokenStream = body.parse().map_err(|_| Error::Syntax)?;
                token_budget(tokens, 0, 0, &mut 0)?;
                let file = syn::parse_file(source).map_err(|_| Error::Syntax)?;
                Ok(visitor(&file))
                // file and its recursive children are dropped on this same stack.
            })
            .map_err(|_| Error::Resource)?
            .join()
            .map_err(|_| Error::Resource)?
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    #[test]
    fn ordinary_files_are_not_limited_to_128_total_units() {
        let _test = TEST_LOCK.lock().unwrap();
        let source = "pub fn f() { let x = 1; let _ = x + 2; }\n".repeat(100);
        assert_eq!(inspect(&source, |f| f.items.len()), Ok(100));
        assert!(inspect(include_str!("review/lanes.rs"), |_| ()).is_ok());
        super::resource_tests::admitted_chain_and_drop();
        assert!(inspect("#!/tool unmatched [ ( /*\nfn f() {}", |_| ()).is_ok());
    }
    #[test]
    fn comments_and_literals_do_not_consume_recursion() {
        let _test = TEST_LOCK.lock().unwrap();
        for source in [
            format!("/* {} */ fn f() {{}}", "(".repeat(40000)),
            format!("fn f() {{ let _ = r###\"{}\"###; }}", "[{".repeat(10000)),
            "fn f<'a>(_: &'a str) { let _ = '\\u{7b}'; let _ = '}'; }".into(),
        ] {
            assert_eq!(inspect(&source, |_| ()), Ok(()));
        }
    }
    #[test]
    fn pathological_recursion_and_oversize_are_rejected_before_syn() {
        let _test = TEST_LOCK.lock().unwrap();
        for source in [
            format!("fn f() {{ {}true }}", "return ".repeat(4000)),
            format!("fn f() {{ let _ = {}true; }}", "!".repeat(12000)),
            format!(
                "fn f() {{ let _ = {}true{}; }}",
                "(".repeat(12000),
                ")".repeat(12000)
            ),
            format!("type T = {}bool{};", "Vec<".repeat(6000), ">".repeat(6000)),
            format!("fn f() {{ {} {{}} }}", "if true {} else ".repeat(2000)),
            format!("// {}", "a".repeat(MAX_SOURCE_BYTES)),
            format!("foor#\"\"{}\"#", "(".repeat(12000)),
            format!("{}{}", "/*".repeat(33), "*/".repeat(33)),
        ] {
            assert_eq!(inspect(&source, |_| ()), Err(Error::Resource));
        }
    }
    #[test]
    fn malformed_input_is_controlled() {
        let _test = TEST_LOCK.lock().unwrap();
        for source in [
            "fn f(]",
            "fn f() { let x = ; }",
            "/* unfinished",
            "const X: &str = r#\"unfinished",
        ] {
            assert_eq!(inspect(source, |_| ()), Err(Error::Syntax));
        }
    }
}

#[cfg(test)]
mod resource_tests {
    use super::*;
    // A separate module would race the process-wide nonblocking admission slot;
    // these checks are invoked by one test in the main serialized test module.
    pub(super) fn admitted_chain_and_drop() {
        for source in [
            format!("fn f() {{ let _ = {}true; }}", "!".repeat(900)),
            format!("type T = {}bool{};", "Vec<".repeat(200), ">".repeat(200)),
            format!("fn f() {{ {} {{}} }}", "if true {} else ".repeat(200)),
        ] {
            assert!(inspect(&source, |file| {
                struct Visitor;
                impl<'a> syn::visit::Visit<'a> for Visitor {}
                syn::visit::Visit::visit_file(&mut Visitor, file);
            })
            .is_ok());
        }
        assert_eq!(
            inspect("fn f() {}", |_| panic!("controlled visitor panic")),
            Err::<(), _>(Error::Resource)
        );
        assert_eq!(inspect("fn f() {}", |_| ()), Ok(()));
        inspect("fn f() {}", |_| {
            assert_eq!(inspect("fn nested() {}", |_| ()), Err(Error::Resource))
        })
        .unwrap();
    }
}
