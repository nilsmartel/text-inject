#![feature(slice_patterns)]

fn main() {
    let input = match std::env::args().nth(1) {
        Some(s) => s,
        _ => print_help(),
    };

    let file = std::fs::read_to_string(input).expect("Failed to read file");

    let result = parse(file.as_bytes());

    print!("{}", result);
}

fn print_help() -> ! {
    eprintln!(
        "text-inject
        Usage:  text-inject <input-file>"
    );

    std::process::exit(1)
}

fn parse(s: &[u8]) -> String {
    match s {
        [b'\\', b'\\', rest @ ..] => format!("\\{}", parse(rest)),
        [b'\\', b'(', rest @ ..] => parse_env(String::new(), rest),
        [a, rest @ ..] => format!("{}{}", *a as char, parse(rest)),
        [] => String::new(),
    }
}

/// injecting for pattern ENVVAR_NAME)<rest>..
fn parse_env(n: String, s: &[u8]) -> String {
    match s {
        [b')', rest @ ..] => format!("{}{}", resolve_env(n), parse(rest)),
        [n1, rest @ ..] => parse_env(format!("{}{}", n, (*n1 as char)), rest),
        [] => String::new(),
    }
}

fn resolve_env(n: String) -> String {
    std::env::var(&n).expect(&format!("Can't find Environment Variable `{}`", &n))
}
