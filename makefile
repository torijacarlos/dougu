
build:
	@cargo build --release
	@cp ./target/release/dougu ~/.local/bin
