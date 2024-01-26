use anyhow::Result;

pub fn login(username: &str, password: &str) -> Result<()> {
    println!("username: {}, password: {}", username, password);
    Ok(())
}
