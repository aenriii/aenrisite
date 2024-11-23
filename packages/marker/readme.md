# marker

the not-too-thin wrapper around libraries for various markup languages
which powers the `marker!` macro

## can i use this on its own?

yes! you can!

```rs
use marker::page_from_file;

fn main() {
    let foo = page_from_file("./pages/foo.dtf");
    let bar = page_from_file("./pages/bar.md");
}
```
