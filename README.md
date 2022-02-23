# Create namespaced tmp files

Sometimes you want to run multiple instances of a program and have it not
conflict with other instances of the same program. And sometimes these programs
use the `/tmp` directory and then it's all sad times all around.

This crate let's you easily namespace your tmp files, and also creates the
base dir for you!

```rust
use namespaced_tmp::blocking::in_tmp;
use std::io;

fn main() -> io::Result<()> {
    let (path, e) = in_tmp("namespace", "file");
    if let Some(e) = e {
        return Err(e);
    }
    assert!(path.parent().unwrap().is_dir());
    assert!(!path.exists());
    std::fs::remove_dir(path.parent().unwrap())?;
    Ok(())
}
```
