# lib1337patch-rs
Rust library dedicated to parsing 1337 patch files.

## Usage
```rust
use lib1337patch::F1337Patch;

fn main() {
    let patchfile = File::open("test.1337.txt").unwrap();
    let f1337 = F1337Patch::new(&mut patchfile).unwrap();

    println!(f1337.target_filename)
    for patch in f1337.patches {
        println!("{}: {} -> {}", patch.target_address, patch.old, patch.new);
    }

    // Or use your favorite bin edition library.
}
```

## Contributing
You are free to contribute to this project.

[Fork](https://github.com/GGLinnk/lib1337patch-rs/fork), edit, commit and [pull request](https://github.com/GGLinnk/lib1337patch-rs/compare).<br />
Easy as that.

If you encounter any issue or have any suggetion don't hesitate to post an [issue](https://github.com/GGLinnk/lib1337patch-rs/issues).