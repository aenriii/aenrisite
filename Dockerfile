from rustlang/rust:nightly-slim

copy . app
workdir app
cmd ["cargo", "run", "--release"]
