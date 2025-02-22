`rust-lld` works in most of cases but here not. I don't know why. I failed to reproduce this error without the libs in the `libs` folder. 

# Reproduce
`cargo build` creates an error `rust-lld: error: undefined symbol: checksum(void const*, int)`

Uncomment the line in `.cargo/config.toml`

`cargo build` works.

# History
I created a module for the game Garry's Mod. For this I used https://github.com/danielga/garrysmod_common with the branch `x86-64-support-sourcesdk`. This repo contains a way to compile using premake. By default it creates some static libs and using these static libs and your own C++ project, it creates a shared lib. This shared lib can be used with the game Garry's Mod. But because I prefer Rust I modified a line in the premake configs to create a static lib instead of a shared lib. I compiled using the command `make config=release_x86_64` and it created some default static libs and a static lib for my project. In this case in the folder libs `libexample.a` is my project and `libtier1.a` a default static lib.

One day I updated with rustup. This update added `rust-lld`. After I had an `undefined symbol:` error. When I added `rustflags = ["-Zlinker-features=-lld"]` in `.cargo/config.toml` it worked again. That's when I discovered that `rust-lld` is broken.