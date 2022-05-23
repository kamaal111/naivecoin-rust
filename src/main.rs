mod models;
use models::block::Block;

fn main() {
    let genesis_block = Block {
        index: 0,
        hash: String::from("cd2fb2ace926608315b2a5bd1bc2a259dce057a21ed63351adc0b1326da2a99e"),
        previous_hash: None,
        timestamp: 1652722519,
        data: String::from("The Genesis block!!!"),
    };

    println!("{:#?}", genesis_block);
}
