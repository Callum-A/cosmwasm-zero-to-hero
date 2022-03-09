# Part Two - Environment Setup

So you think you’re ready to start! Now we need to setup our development environment. First things first, text editors. I’ll rattle a few off for those that skipped ahead.

1. IntelliJ with Rust Plugin - [Setup Guide](https://www.youtube.com/watch?v=H_-L7sjLcH8)
1. VSCode - [Setup Guide](https://www.youtube.com/watch?v=aYsUBddY7KY)

The reason I'm not covering editor setup is that this would make the guide longer than it needed to be initially. I will simply provide pointers to help.

So we have our trusty text editor, it’s time for us to generate some boilerplate. Thankfully the guys at InterWasm have set this up for us! I’m assuming you have cargo and rust installed and ready to go!

## Ensuring Rustup is Installed

Firstly we're going to need to ensure rustup is setup. To do this run the following commands in your terminal:

```bash
rustup default stable
cargo version
# If version is lower than 1.55 we need to update by running:
rustup update stable
```

Credit to the guys at Interwasm for this, this snippet is taken from [here](https://docs.cosmwasm.com/docs/1.0/getting-started/installation#installing-rust-in-linux-and-mac).

## Adding the WASM Target

We need to build our smart contracts to WASM files. So we need to add the target for our builds. We do this by running these commands in your terminal

```bash
rustup target add wasm32-unknown-unknown
```

## Installing Cargo Generate

Another requirement we need to generate the boilerplate is Cargo Generate. This can be installed via CLI using commands:

```bash
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

## Generating CW-Template

Now we are ready to use [CW-Template](https://github.com/InterWasm/cw-template). Again credit to Interwasm.
CW-Template sets up a boilerplate contract project. It has two main ways to be used:

Using the latest version:

```bash
cargo generate --git https://github.com/CosmWasm/cw-template.git --name <PROJECT_NAME>
```

Using an older or a different branch:

```bash
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch <BRANCH_NAME> --name <PROJECT_NAME>
```

For compatibility I am going to use an older version using the branch flag. This means if you are following along (you should be) we will be using the same package versions.

The version I will be using is `clean-1.0-beta`. This version has minimal boilerplate so we won't have to delete much, although it isn't the latest version. So run the command:

```bash
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch clean-1.0-beta --name cw-starter
```

Once complete the output should look something like the following:

```bash
🔧   Generating template ...
[ 1/34]   Done: .cargo/config
[ 2/34]   Done: .cargo
[ 3/34]   Skipped: .circleci/config.yml
[ 4/34]   Done: .circleci
[ 1/34]   Done: .cargo/config
[ 2/34]   Done: .cargo
[ 3/34]   Skipped: .circleci/config.yml
[ 4/34]   Done: .circleci
[ 5/34]   Done: .editorconfig
[ 6/34]   Done: .github/workflows/Basic.yml
[ 7/34]   Done: .github/workflows
[ 8/34]   Done: .github
[ 9/34]   Done: .gitignore
[10/34]   Done: .gitpod.Dockerfile
[11/34]   Done: .gitpod.yml
[ 1/34]   Done: .cargo/config
[ 2/34]   Done: .cargo
[ 3/34]   Skipped: .circleci/config.yml
[ 4/34]   Done: .circleci
[ 5/34]   Done: .editorconfig
[ 6/34]   Done: .github/workflows/Basic.yml
[ 7/34]   Done: .github/workflows
[ 8/34]   Done: .github
[ 9/34]   Done: .gitignore
[10/34]   Done: .gitpod.Dockerfile
[11/34]   Done: .gitpod.yml
[12/34]   Done: Cargo.lock
[13/34]   Done: Cargo.toml
[14/34]   Done: Developing.md
[15/34]   Done: Importing.md
[16/34]   Done: LICENSE
[17/34]   Done: NOTICE
[18/34]   Done: Publishing.md
[19/34]   Done: README.md
[20/34]   Done: examples/schema.rs
[21/34]   Done: examples
[22/34]   Done: rustfmt.toml
[23/34]   Done: schema/execute_msg.json
[24/34]   Done: schema/instantiate_msg.json
[25/34]   Done: schema/migrate_msg.json
[26/34]   Done: schema/query_msg.json
[27/34]   Done: schema/state.json
[28/34]   Done: schema
[29/34]   Done: src/contract.rs
[30/34]   Done: src/error.rs
[31/34]   Done: src/lib.rs
[32/34]   Done: src/msg.rs
[33/34]   Done: src/state.rs
[34/34]   Done: src
🔧   Moving generated files into: `/Path/To/Whereever/cosmwasm-zero-to-hero/02 - Environment Setup/code/cw-starter`...
✨   Done! New project created /Path/To/Whereever/cosmwasm-zero-to-hero/02 - Environment Setup/code/cw-starter
```

Let's verify it's all setup correctly by running some commands:

```bash
# Change directory to root directory of the contract
cd cw-starter
# Run tests, currently there are no tests so output should display running 0 tests
cargo test
# Generate JSON schema, should generate a bunch of JSON files under the schema directory
cargo schema
# Build an unoptimised WASM file, will be located under
# target/wasm32-unknown-unknown/release/cw_starter.wasm
cargo wasm
```

We are now setup, so open this folder in your preferred text editor and we'll move on to the next section!

The code for this section is available under the `code` directory. This will be standard for chapters going forward.
