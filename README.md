# Rustaceos

Rustaceos are a set of libraries derived from eosio libraries. Rustaceos is intended to provide rust tooling for eosio.

### Basic setup for fill-x


* Run nodeos with the [state_history_plugin](https://developers.eos.io/eosio-nodeos/docs/monitoring-with-state-history) plugin enabled:

```
nodeos -e -p eosio --plugin eosio::producer_plugin --plugin eosio::chain_api_plugin --plugin eosio::http_plugin --plugin eosio::state_history_plugin --data-dir .nodeos/data --config-dir .nodeos/config --access-control-allow-origin='*' --contracts-console --http-validate-host=false --state-history-dir ship --trace-history --chain-state-history --verbose-http-errors --filter-on='*' --disable-replay-opts --delete-all-blocks
```

* Install the [rust toolchain](https://www.rust-lang.org/tools/install)
* Start the app:
```
git submodule update --init --recursive
cd apps/fill-x
cargo run
```

### Basic setup for Contracts

```
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-gc

brew install binaryen

# optional tool to debug contracts (converts wasm to wat)
brew install wabt

cd contracts/hello
./build.sh

# feel free to play with the generate wasms:
# ./target/wasm32-unknown-unknown/release/hello.wasm >> full released wasm
# ./hello_gc.wasm >> wasm optimized by wasm-gc
# ./hello_gc_opt.wasm >> wasm-gc optimized by wasm-opt

cleos set code hello hello_gc_opt.wasm
cleos set abi hello helo.abi.json
```
