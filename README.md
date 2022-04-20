# Nomic delegators

## What

This script connects to the merk database that is part of the nomic stack.
We strongly suggest to do this on a backup of your database in case anything happens.

The script in it's actual form will get your validator and iterate over all the delegators at current height.

## How

1. 

```sh

git clone https://github.com/stakefrites/nomic-delegators.git
cd nomic-delegators

```


2.
```rs
// Change these values to reflect the reality
const YOUR_ADDRESS: &str = "nomicxxxxxxxxxxxxxxxxxxxxxxxxxxx"; // Your validator address
const MERK_FOLDER: &str = "/absolute/path/to/.nomic-stakenet/merk";

```

3.

```sh
rustup default nightly
cargo run
```