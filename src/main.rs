mod block;
mod blockchain;

fn main() {
    // Create a chain
    let mut chain = blockchain::BlockChain::new();
    chain.add_block("Bob sent 4u to Alice".to_string()).unwrap();
    chain.add_block("Alice sent 50u to Bob".to_string()).unwrap();
    chain.add_block("Bob sent 46u to Jake".to_string()).unwrap();

    for block in &chain.chain {
        let json = serde_json::to_string_pretty(&block).unwrap();
        println!("{json}");
    }

    println!("valid: {}", chain.is_valid())
}
