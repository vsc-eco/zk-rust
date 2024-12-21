.PHONY: test-merkle-tree-lib build-program run-program execute-program-gen-core-proof-groth16 execute-program-gen-core-proof-plonk

test-merkle-tree-lib:
	@cd lib3 && cargo test

build-program:
	@cd program && cargo prove build

run-program:
	@cd script && cargo run --release -- --execute --data "a b c d" --index 2

# for these two cmds below, have Docker funning; also, know it might take a few minutes
#
# they could also auto-download image: "ghcr.io/succinctlabs/sp1-gnark:v3.0.0"
#
# from what I can see... if you don't have enough memory, docker run will fail after about ~10 minutes on my machine
execute-program-gen-core-proof-groth16:
	@cd script && RUST_LOG=info cargo run --release --bin evm -- --data "a b c d" --index 2 --groth16
execute-program-gen-core-proof-plonk:
	@cd script && RUST_LOG=info cargo run --release --bin evm -- --data "a b c d" --index 2 --plonk