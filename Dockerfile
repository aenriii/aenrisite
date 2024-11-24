from rust:latest

copy . .
workdir .
cmd ["cargo", "run", "--release"]
