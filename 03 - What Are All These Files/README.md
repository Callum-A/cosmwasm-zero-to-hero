# Part Three - What Are All These Files

So we now have our contract open in our preferred editor. Let's talk about the file structure.

```bash
cw-starter/ # Root
├── .cargo/
│   └── config # Configuration for cargo commands such as cargo wasm, cargo schema, etc.
├── .circleci/
│   └── config.yml # Circle CI workflows and integration
├── .github/workflows/
│   └── Basic.yml # GitHub actions integration
├── examples/
│   └── schema.rs # Rust file to generate JSON schema via cargo schema. Outputs to schema/
├── schema/ # Output folder for JSON schema
├── src/ # Where our smart contract rust files are located
│   ├── contract.rs # Main contract logic, instantiate, execute, query
│   ├── error.rs # Where we define our contract errors
│   ├── lib.rs # Where we define our modules
│   ├── msg.rs # Where we define our message types
│   └── state.rs # Where we define any state variables
└── target/ # Where unoptimised WASM files are outputted
```

This covers most of the important directories that you will encounter. Others such as `Cargo.toml` are for your dependencies and others are simple `.md` files for reading.
