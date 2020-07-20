use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref REPLACEMENTS: HashMap<&'static str, &'static str> = [
        ("reset", "\u{001B}[0m"),
        ("black", "\u{001B}[30m"),
        ("blue", "\u{001B}[34m"),
        ("cyan", "\u{001B}[36m"),
        ("green", "\u{001B}[32m"),
        ("magenta", "\u{001B}[35m"),
        ("red", "\u{001B}[31m"),
        ("white", "\u{001B}[37m"),
        ("yellow", "\u{001B}[33m"),
        ("black-bg", "\u{001B}[40m"),
        ("blue-bg", "\u{001B}[44m"),
        ("cyan-bg", "\u{001B}[46m"),
        ("green-bg", "\u{001B}[42m"),
        ("magenta-bg", "\u{001B}[45m"),
        ("red-bg", "\u{001B}[41m"),
        ("white-bg", "\u{001B}[47m"),
        ("yellow-bg", "\u{001B}[43m"),
        ("invert", "\u{001B}[7m"),
        ("bold", "\u{001B}[1m"),
        ("blink", "\u{001B}[5m"),
        ("clear", "\u{001B}[2J"),
        ("clear-line", "\u{001B}[K"),
        ("save", "\u{001B}[s"),
        ("restore", "\u{001B}[u"),
    ].iter().cloned().collect();
}
