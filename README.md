# waseda-moodle

A simple crate to get a list of the enrolled courses.

## Usage

```rust
use waseda-moodle::*;

#[tokio::main]
async fn main() -> Result<()> {
    //login
    let session = Session::login("login id", "password").await?;

    //get a list of the enrolled courses
    let list = course::fetch_enrolled_courses(&session).await?;

    println!("{:#?}", list);

    Ok(())
}

```
