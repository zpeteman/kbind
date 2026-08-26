pub fn check_command_safety(cmd: &str) {
    let patterns = [
        ("rm -rf /", "rm -rf /"),
        ("rm -rf", "bare rm -rf"),
        ("dd if=", "dd if="),
        ("mkfs", "mkfs"),
        ("> /dev/sda", "> /dev/sda"),
        ("git push --force", "git push --force"),
        ("git push -f", "git push -f"),
        ("DROP TABLE", "DROP TABLE"),
        ("DROP DATABASE", "DROP DATABASE"),
        (":(){ :|:& };:", "fork bomb"),
        ("chmod -R", "chmod -R"),
        ("chown -R", "chown -R"),
    ];

    for (pattern, reason) in patterns {
        if cmd.contains(pattern) {
            eprintln!("⚠ this command is destructive, double-check before running (matched pattern: {})", reason);
            break;
        }
    }
}
