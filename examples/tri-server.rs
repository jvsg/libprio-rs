// SPDX-License-Identifier: MPL-2.0

use prio::client::*;
use prio::finite_field::*;
use prio::server::*;

fn recombine_seed(seeds: Vec<Vec<u8>>) -> Vec<u8> {
    let mut seed= vec![0u8; 32];
    for share in seeds {
        //println!("The share of seed is: {:?}", share);
        for (j, d) in share.iter().enumerate() {
            //println!("{} {} {}", j, seed[j], *d);
            seed[j] += *d;
        }
    }

    seed
}

#[cfg(not(feature = "without-encryption"))]
fn main() {
}

#[cfg(feature = "without-encryption")]
fn main() {
    let dim = 8;
    let server_count: usize = 3;

    let mut client = Client::new(dim, server_count).unwrap();

    let data_u32 = [0, 0, 1, 0, 0, 0, 0, 0];
    println!("Client 1 Input: {:?}", data_u32);

    let data = data_u32
        .iter()
        .map(|x| Field::from(*x))
        .collect::<Vec<Field>>();

    let data = data_u32
        .iter()
        .map(|x| Field::from(*x))
        .collect::<Vec<Field>>();

    let (proof, shares) = client.encode_simple(&data).unwrap();
    let eval_at = Field::from(12313);

    let mut server1 = Server::new(dim, true, server_count);
    let mut server2 = Server::new(dim, false, server_count);
    let mut server3 = Server::new(dim, false, server_count);

    let v_1 = server1
        .generate_verification_message(eval_at, &proof)
        .unwrap();

    // Recollect seeds, and then extract share field
    // In practise, server2 and server 3 will exchange their seed shares
    // and recombine them.
    let seed = recombine_seed(shares);

    let v_2 = server2
        .generate_verification_message(eval_at, &seed)
        .unwrap();

    let v_3 = server3
        .generate_verification_message(eval_at, &seed)
        .unwrap();

    let _ = server1.aggregate(&proof, &v_1, &v_2).unwrap();
    let _ = server2.aggregate(&seed, &v_1, &v_2).unwrap();

    assert_eq!(is_valid_share(&v_1, &v_2), true);

    server1.merge_total_shares(server2.total_shares()).unwrap();
    println!("Final Publication: {:?}", server1.total_shares());
}
