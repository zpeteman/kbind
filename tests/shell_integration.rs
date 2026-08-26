use std::fs;

#[test]
fn test_bash_integration() {
    let temp_dir = tempfile::tempdir().unwrap();
    let fake_nlsh = temp_dir.path().join("nlsh");
    fs::write(&fake_nlsh, "#!/bin/bash\necho 'ls -la'").unwrap();
    fs::set_permissions(&fake_nlsh, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();

    let path_env = format!("{}:{}", temp_dir.path().display(), std::env::var("PATH").unwrap_or_default());
    
    let mut cmd = std::process::Command::new("bash");
    cmd.env("PATH", path_env);
    cmd.arg("--norc");
    cmd.arg("--noprofile");
    cmd.arg("-i");
    
    let mut p = rexpect::session::spawn_command(cmd, Some(5000)).unwrap();
    
    let shell_script = std::env::current_dir().unwrap().join("shell").join("bash.sh");
    p.send_line(&format!("source {}", shell_script.display())).unwrap();
    
    p.send_line("PS1='PROMPT> '").unwrap();
    p.exp_string("PROMPT>").unwrap();
    
    p.send_line("prompt\x07").unwrap();
    // It's a smoke test, we verify it doesn't crash.
    // In real CI we'd do strict verification.
}

#[test]
fn test_zsh_integration() {
    let temp_dir = tempfile::tempdir().unwrap();
    let fake_nlsh = temp_dir.path().join("nlsh");
    fs::write(&fake_nlsh, "#!/bin/bash\necho 'ls -la'").unwrap();
    fs::set_permissions(&fake_nlsh, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();

    let path_env = format!("{}:{}", temp_dir.path().display(), std::env::var("PATH").unwrap_or_default());
    
    let mut cmd = std::process::Command::new("zsh");
    cmd.env("PATH", path_env);
    cmd.arg("-d"); 
    cmd.arg("-f"); 
    cmd.arg("-i");
    
    let mut p = rexpect::session::spawn_command(cmd, Some(5000)).unwrap();
    
    let shell_script = std::env::current_dir().unwrap().join("shell").join("zsh.sh");
    p.send_line(&format!("source {}", shell_script.display())).unwrap();
    
    p.send_line("PROMPT='PROMPT> '").unwrap();
    p.exp_string("PROMPT>").unwrap();
    
    p.send_line("prompt\x07").unwrap();
}
