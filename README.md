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
cd fill-x
cargo run
```
