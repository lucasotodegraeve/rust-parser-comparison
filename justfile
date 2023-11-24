alias hf := hyperfine
alias cg := callgrind

_default:
	@just --list

watch name:
	watchexec -c clear cargo run --bin {{name}}

test:
	watchexec -c clear cargo test -- --nocapture

hyperfine:
	hyperfine --setup "cargo build --release" ./target/release/vanilla ./target/release/nom ./target/release/pest	

bench:
	cargo bench

callgrind name:
	cargo build
	valgrind --tool=callgrind ./target/debug/{{name}}
