# summer-math
Train some elementary math during the summer

Just a small program that can be used into a cron to generate some elementary math operations so
that kids can train without forgetting about multiplications/divisions

## Requirements
You need to have rust installed

## ENV vars
copy the `.envrc.example` to `.envrc` and change the values according to your needs, `direnv allow` will be required after that

## Run with cargo
simply run `cargo run`

## Compile and release
run `cargo build --release` and use the executable in `./target/release`
