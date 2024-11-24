from rust:latest

copy . app
workdir app
cmd ["cargo", "run", "--release"]
