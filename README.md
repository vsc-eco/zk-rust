# SP1 Project Template

This is a template for creating an end-to-end [SP1](https://github.com/succinctlabs/sp1) project
that can generate a proof of any RISC-V program.

# Shorthands

Use the `Makefile` to run the following commands from the root directory of the project:

```sh
# runs tests on the merkle tree lib
make test-merkle-tree-lib

# builds the main program (creates and sets `SP1_ELF_merkle-tree-program`)
make build-program

# runs the main program with dummy data
make run-program

### these output fixtures for evm-compatible proofs into
### the ~/contracts/fixtures directory upon succesfully completing
#
# generates a core proof (more time-consuming) for groth16 + you need Docker running
make execute-program-gen-core-proof-groth16
# generates a core proof (more time-consuming) for plonk + you need Docker running
make execute-program-gen-core-proof-plonk
```

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)
- Docker (for core proof generation)

## Running the Project

There are four main ways to run this project: build a program, execute a program, generate a core proof, and
generate an EVM-compatible proof.

### Build the Program

To build the program, run the following command:

```sh
cd program
cargo prove build
```

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate a Core Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 128GB RAM to generate a Groth16 or PLONK proof.

To generate a proof that is small enough to be verified on-chain and verifiable by the EVM:

```sh
cd script
cargo run --release --bin evm -- --system groth16
```

this will generate a Groth16 proof. If you want to generate a PLONK proof, run the following command:

```sh
cargo run --release --bin evm -- --system plonk
```

These commands will also generate fixtures that can be used to test the verification of SP1 zkVM proofs
inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command:

```sh
cargo prove vkey --program fibonacci-program
```

## Using the Prover Network

We highly recommend using the Succinct prover network for any non-trivial programs or benchmarking purposes. For more information, see ~~the [setup guide](https://docs.succinct.xyz/generating-proofs/prover-network.html).~~ [this setup guide](https://docs.succinct.xyz/docs/generating-proofs/prover-network/key-setup).

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `SP1_PRIVATE_KEY`
environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following
command:

```sh
SP1_PROVER=network SP1_PRIVATE_KEY=... cargo run --release --bin evm
```

## Notes

- If you are running this on MacOS M1 aarch64, don't use `zsh` as your shell. Use `bash` instead; `zsh` throws unintelligible errors.

## Next Steps

- Right now `~/lib` contains the old fibonacci program. `/~lib3` is the current refactoring effort to make this work for Merkle Trees.
- There are some `.expect()` and `.unwrap()` calls in the code that should be handled more gracefully (marked with `// todo ...`s).
- The `evm.rs` file should be generating fixtures for the EVM-compatible proof (output in `~/contracts/fixtures`), but I think my computer is running out of memory to do so; everything code-wise *seems* correct.
- To deploy on their [Prover Network](https://docs.succinct.xyz/docs/generating-proofs/prover-network) you need to sign up for beta access! So, it could be worth [signing up for that](https://docs.google.com/forms/d/e/1FAIpQLSd-X9uH7G0bvXH_kjptnQtNil8L4dumrVPpFE4t8Ci1XT1GaQ/viewform) in case it takes a while. They claim this network is the best way to generate proofs for large, complex programs.

