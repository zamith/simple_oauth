# Simple OAuth

A port to rust of ruby's [simple oauth](git remote add origin git@github.com:zamith/simple_oauth.git
git push -u origin master).

*Note:* This is not usable at all, at the moment, very much work in progress.

## Adding to a project

1. Get [`Cargo`](https://github.com/rust-lang/cargo).

2. Add as a dependency to your `Cargo.toml`

        [dependencies.simple_oauth]
        path = "https://github.com/zamith/simple_oauth"
    
3. Include it

        extern crate simple_oauth
        use simple_oauth::Header;
    
## Contributing

To compile the library just run `cargo build`.

To run the tests you'll have to:

    cd tests
    cargo test
