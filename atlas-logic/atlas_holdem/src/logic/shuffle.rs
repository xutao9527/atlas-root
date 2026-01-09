use rand::prelude::*;


// fn sha256_hex(input: &str) -> String {
//     let hash = Sha256::digest(input.as_bytes());
//     hex::encode(hash)
// }

// fn generate_seed(server_seed: &str, client_seed: &str, nonce: u32) -> u64 {
//     let input = format!("{}{}{}", server_seed, client_seed, nonce);
//     let hash = Sha256::digest(input.as_bytes());
//     // 取前8个字节作为 u64
//     let mut bytes = [0u8; 8];
//     bytes.copy_from_slice(&hash[0..8]);
//     u64::from_be_bytes(bytes)
// }
//
// fn shuffling_with_seed(mut numbers: Vec<u32>, mut seed: u64) -> Vec<u32> {
//     for i in (1..numbers.len()).rev() {
//         // 简单 LCG 生成伪随机
//         seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
//         let j = (seed % ((i + 1) as u64)) as usize;
//         numbers.swap(i, j);
//     }
//     numbers
// }

fn shuffling()-> Vec<u8>{
    let mut deck: Vec<u8> = (0..52).collect();
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);
    deck
}


#[cfg(test)]
mod tests {
    // use crate::logic::shuffle::{generate_seed, sha256_hex, shuffling, shuffling_with_seed};
    //
    // #[test]
    // pub fn test_shuffling(){
    //     println!("{:?}", shuffling());
    //
    // }
    //
    // #[test]
    // pub fn test_shuffling_with_seed(){
    //     // 服务器私有
    //     let server_seed = "server_secret";
    //     // 玩家已知
    //     let client_seed = "player_seed";
    //     let numbers: Vec<u32> = (1..=4).collect();
    //
    //     // 开始前公布 server_seed 的哈希
    //     let server_seed_hash = sha256_hex(server_seed);
    //     println!("server_seed sha256 hash (public before game): {}", server_seed_hash);
    //
    //     let nonce = 1;
    //
    //     let seed = generate_seed(server_seed, client_seed, nonce);
    //     let result = shuffling_with_seed(numbers.clone(), seed);
    //
    //     println!("Nonce {}: result: {:?}", nonce, result);
    //     println!("Nonce {}: ClientSeed (player knows): {}", nonce, client_seed);
    //     println!("Nonce {}: ServerSeed (reveal after game): {}", nonce, server_seed);
    //
    // }

}

