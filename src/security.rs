use keyring::Entry;

pub fn get_key(provider: &str) -> anyhow::Result<String> {
    let entry = Entry::new("nlsh", provider)?;
    let password = entry.get_password()?;
    Ok(password)
}

pub fn set_key(provider: &str, key: &str) -> anyhow::Result<()> {
    let entry = Entry::new("nlsh", provider)?;
    entry.set_password(key)?;
    Ok(())
}

pub fn delete_key(provider: &str) -> anyhow::Result<()> {
    let entry = Entry::new("nlsh", provider)?;
    entry.delete_password()?;
    Ok(())
}
