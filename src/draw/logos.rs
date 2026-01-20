pub fn get_logo(os_name: &str) -> Vec<&'static str> {
    match os_name.to_lowercase() {
        n if n.contains("arch") => vec![
            "       /\\       ",
            "      /  \\      ",
            "     /    \\     ",
            "    /      \\    ",
            "   /   ,,   \\   ",
            "  /   |  |   \\  ",
            " /_-''    ''-_\\ ",
        ],
        n if n.contains("debian") => vec![
            "  _____  ",
            " /  __ \\ ",
            "|  /    |",
            "|  \\___/ ",
            " \\____   ",
            "      \\  ",
            "  ____/  ",
        ],
        n if n.contains("ubuntu") => vec![
            "           _  ",
            "         -   -",
            "       -   _   -",
            "      -  _| |_  -",
            "     -  |_   _|  -",
            "      -   |_|   -",
            "       -       -",
            "         - _ -",
        ],
        n if n.contains("fedora") => vec![
            "      _____",
            "     /   __)",
            "     |  /  ",
            "  ___|  |__",
            " (___    __)",
            "    |   |  ",
            "    |___|  ",
        ],
        n if n.contains("windows") => vec![
            " ,.=:!!t3Z3z.,",
            " :tt:::tt333EE3",
            " Et:::ztt33EEEL",
            " ;3=*^\"\"\"\"\"*4EE",
        ],
        _ => vec![
            "   _______   ",
            "  |       |  ",
            "  | MIZU  |  ",
            "  | FETCH |  ",
            "  |_______|  ",
        ],
    }
}
