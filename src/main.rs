use std::env;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let key = "HOME";
    match env::var(key) {
        Ok(val) => {
            let home = PathBuf::from(val);
            let taskvault_dir = home.join(".taskvault");
            println!("{}", taskvault_dir.display());
            std::fs::create_dir_all(&taskvault_dir)?;
        }
        Err(e) => println!("couldn't interpret {key}: {e}", key=key, e=e),
    }
    Ok(())
}
