# waseda-moodle

A simple crate to get a list of the enrolled courses.

## Usage

```rust
use waseda-moodle::*;

#[tokio::main]
async fn main() -> Result<()> {
    //login
    let session = Session::login("login id", "password").await?;

    //get a list of the enrolled courses (except for hidden courses)
    let list = course::fetch_enrolled_courses(&session).await?;
    //get a list of the hidden courses
    let hidden_list = course::fetch_hidden_courses(&session).await?;

    println!("list: {:#?}", list);
    println!("hidden_list: {:#?}", hidden_list);

    Ok(())
}

```
