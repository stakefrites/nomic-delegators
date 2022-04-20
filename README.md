# Nomic delegators

## What

This script connects to the merk database that is part of the nomic stack.
We strongly suggest to do this on a backup of your database in case anything happens.

The script in it's actual form will get your validator and iterate over all the delegators at current height.

## How

1. Clone the repo

```sh

git clone https://github.com/stakefrites/nomic-delegators.git
cd nomic-delegators

```


2. Change the variables
```rs
// Change these values to reflect the reality in your main.rs
const YOUR_ADDRESS: &str = "nomicxxxxxxxxxxxxxxxxxxxxxxxxxxx"; // Your validator address
const MERK_FOLDER: &str = "/absolute/path/to/.nomic-stakenet/merk";

```

3. Compile and run

```sh
rustup default nightly
cargo run
```

### Stake safely my friends 🥩🍟

Stake Frites is a bunch of french canadians entrepreneurs from Quebec, Canada. We believe in Cosmos and are actively participating in building stuff for the ecosystem.
Consider delegating to our validators on a few different chains: Nomic, Atom, Akash, Evmos, Sifchain to only name a few.