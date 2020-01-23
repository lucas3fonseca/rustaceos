# LikeTree

Someone creates a new like or dislike for a product, an idea, an artist,
a place, a technology, you name it. People can then like it or not adding
their opinions. The likes are recursive, because people can like or dislike
the like item, and this very same like can also be liked/disliked.

## Why?

The idea of LikeTree dapp is an example to explore how a project
would work leveraging the wasm ecosystem without abis:

```
liketree
|- core
|- contract -----> imports the core folder
|- fillx --------> imports the core to parse actions and table deltas
|- web ----------> imports the core to generate a web consumable wasm that submits actions
```

* core: main dapp lib. It has the business logic, structs, actions,
rules that can be reused by any app through WASM or the smart contract.
* contract: is the eos smart contract. It will import the core lib and
act as a thin layer to route the contract actions and use the to lib
to read data, validate rules and store the data.
* fillx: connects to EOS SHIP to capture the liketree contract actions
and deltas. It will also use the core lib importing to be able to parse
the actions.
* web: a frontend application WASM lib. It will import the core lib
and act as a thin layer exporting functions to javascript that will
enable people to read data and sign transactions.
