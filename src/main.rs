mod models;
use models::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    match blockchain.generate_next_block("data".to_string()) {
        Err(error) => println!("error: {error:?}"),
        Ok(_) => println!("everything ok"),
    }
}
