use core::mem::{align_of, offset_of};
use anchor_lang::{account, declare_id, prelude::Pubkey, zero_copy, AccountDeserialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey as SdkPubkey;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub const TICK_ARRAY_SIZE_USIZE: usize = 88;

// From https://github.com/orca-so/whirlpools
#[account(zero_copy(unsafe))]
#[repr(packed)]
#[derive(Debug)]
pub struct TickArray {
    pub start_tick_index: i32,
    pub ticks: [Tick; TICK_ARRAY_SIZE_USIZE],
    pub whirlpool: Pubkey,
}

// From https://github.com/orca-so/whirlpools
#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug, PartialEq)]
pub struct Tick {
    // Total 137 bytes
    pub initialized: bool,     // 1
    pub liquidity_net: i128,   // 16
    pub liquidity_gross: u128, // 16

    // Q64.64
    pub fee_growth_outside_a: u128, // 16
    // Q64.64
    pub fee_growth_outside_b: u128, // 16

    // Array of Q64.64
    pub reward_growths_outside: [u128; 3], // 48 = 16 * 3
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_data = match std::fs::read("account.bin") {
        Ok(data) => {
            println!("Decoding account.bin...");
            data
        }
        Err(_) => {
            println!("Re-fetching account data");

            let url = std::env::var("SOLANA_RPC_URL").expect("Missing SOLANA_RPC_URL");

            let client = RpcClient::new(url);

            // Random tick array account
            // https://explorer.solana.com/address/J8f1sBQ6LpFzA98w1m1m3fJa1MqMAgKG49QU4Fi5hw9V
            let account_key = SdkPubkey::try_from("J8f1sBQ6LpFzA98w1m1m3fJa1MqMAgKG49QU4Fi5hw9V")?;

            let data = client.get_account_data(&account_key)?;

            println!("Saving to account.bin...");
            std::fs::write("account.bin", &data)?;

            data
        },
    };

    let tick_array = TickArray::try_deserialize(&mut &account_data[..])?;

    println!("{:#?}", tick_array);

    println!("Offset of start_tick_index in TickArray: {}", offset_of!(TickArray, start_tick_index));
    println!("Alignment of TickArray: {}", align_of::<TickArray>());

    println!("Alignment of Tick: {}", align_of::<Tick>());
    println!("Offset of liquidity_gross in Tick: {}", offset_of!(Tick, liquidity_gross));
    
    Ok(())
}