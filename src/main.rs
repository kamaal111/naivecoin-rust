mod models;
use models::blockchain::Blockchain;

fn main() {
    let blockchain = Blockchain::new();
    println!("{:?}", blockchain.get_latest_block().unwrap().hash);
}
