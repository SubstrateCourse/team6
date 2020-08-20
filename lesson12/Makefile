restart: build purge run
rerun: purge run

expand:
	BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template > expanded-node.rs
    BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template-runtime > expanded-runtime.rs

build-run: build run

run:
	-pkill node-template
	target/debug/node-template --dev -d target/substrate --execution=Native --ws-external -lruntime=debug

alice:
	target/debug/node-template -d target/alice \
		--chain local \
		--alice \
		--port 30333 \
		--ws-port 9944 \
		--rpc-port 9933 \
		--telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
		--validator

bob:
	target/debug/node-template -d target/bob \
		--chain local \
		--bob \
		--port 30334 \
		--ws-port 9945 \
		--rpc-port 9934 \
		--telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
		--validator \
		--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNxyMzG1p7sz9fLhUHk25VBRd84L2UPe1i9j1tcjBLYPi

benchmarks:
	cd node;cargo build --release --features runtime-benchmarks
	target/release/node-template benchmark \
		--chain=dev \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=pallet-template \
		--extrinsic=do_something \
		--steps=20 \
		--repeat=51

purge:
	-target/debug/node-template purge-chain --dev -y -d target/substrate
	-target/debug/node-template purge-chain --chain local -y -d target/alice
	-target/debug/node-template purge-chain --chain local -y -d target/bob

build:
	WASM_BUILD_TYPE=release cargo build

check:
	SKIP_WASM_BUILD= cargo check
check-wasm:
	WASM_BUILD_TYPE=release cargo check

check-debug:
	RUSTFLAGS="-Z external-macro-backtrace" BUILD_DUMMY_WASM_BINARY= cargo +nightly check

check-dummy:
	BUILD_DUMMY_WASM_BINARY= cargo check

