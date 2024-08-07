## tes3

A library for working with TES3 content.

Currently supports reading and writing of all `.esp`, `.esm`, `.nif`, `.kf` structures.

This library is still very much in-progress! At the moment it does little more than expose the core game structures for editing. Code quality or architecture may be questionable. Improvements/contributions are welcome!

The APIs are considered unstable, struct/field names may be changed if I think of something better. If you're depending on this library for your own project it may be a good idea to pin to a specific commit.

The plan is to eventually stablize a useful API and publish to `crates.io`. Not quite there yet though!

### Example

`cargo.toml`
```toml
[dependencies.tes3]
git = "https://github.com/Greatness7/tes3"
default-features = false
features = ["esp"]  # add "nif" only if you need it
```

`main.rs`
```rs
use tes3::esp::{Plugin, Npc};

fn main() -> std::io::Result<()> {
    let plugin = Plugin::from_path("./Morrowind.esm")?;

    for object in plugin.objects_of_type::<Npc>() {
        if object.id == "fargoth" {
            println!("{object:#?}");
        }
    }

    Ok(())
}
```
