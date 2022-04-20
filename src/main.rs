use nomic::app::InnerApp;
use orga::encoding::{Decode, Encode};
use orga::merk::MerkStore;
use orga::state::State;
use orga::store::{Read, Shared, Store};
use orga::Result;

const YOUR_ADDRESS: &str = "nomicxxxxxxxxxxxxxxxxxxxxxxxxxxx";
const MERK_FOLDER: &str = "/absolute/path/to/.nomic-stakenet/merk";

fn main() -> Result<()> {
    let path = std::path::PathBuf::from(MERK_FOLDER); // path to a copy of your `merk` directory from your mainnet validator. highly suggest making a copy
    let store = Store::new(Shared::new(MerkStore::new(path)).into());
    let data_bytes = store.get(&[])?.unwrap();
    let data = <InnerApp as State>::Encoding::decode(data_bytes.as_slice())?;
    let store = store.sub(&[0, 1, 0]);
    let app: InnerApp = State::create(store, data)?;

    let addr = YOUR_ADDRESS.parse().unwrap();

    // Reference to your validator
    let my_validator = app.staking.get(addr)?;
    // Loop over your delegators
    my_validator.delegators.iter().unwrap().for_each(|entry| {
        // Do whatever you want with them
        let (del_addr, delegator) = entry.unwrap();
        if del_addr.bytes() != addr.bytes() {
            let staked_amt: u64 = delegator.staked.amount().unwrap().into();
            println!("Delegator: {0} has {1} NOM Staked.", del_addr, staked_amt);
        }
    });
    Ok(())
}
