use num_bigint::BigUint;

fn main() {
    let bytes: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 207, 176, 154, 78,
        192, 56, 101, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        41, 238, 205, 3, 150, 85, 33, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 199, 70, 117, 180, 150, 239, 231, 95, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 148, 125, 76, 19, 249, 169, 41,
    ];
    // [BigInt([14965631224775206224, 0, 0, 0]), BigInt([3021577815302938909, 0, 0, 0]), BigInt([14359293880404272991, 0, 0, 0]), BigInt([1555005537055779113, 0, 0, 0])]
    let biguint = num_bigint::BigUint::from_bytes_be(&bytes);
    println!("biguint: {:?}", biguint);
    let bytes_back = biguint.to_bytes_be();
    println!("bytes_back: {:?}", bytes_back);

    // new
    println!("new");
    let mut bigints = vec![
        num_bigint::BigUint::from(14965631224775206224u64),
        num_bigint::BigUint::from(3021577815302938909u64),
        num_bigint::BigUint::from(14359293880404272991u64),
        num_bigint::BigUint::from(1555005537055779113u64),
    ];
    bigints.resize(4, num_bigint::BigUint::from(0u64));

    let bytes: Vec<u8> = bigints
        .iter()
        .flat_map(|b| {
            let mut bytes = vec![0u8; 8];
            let b_bytes = b.to_bytes_be();
            bytes[8 - b_bytes.len()..].copy_from_slice(&b_bytes);
            bytes
        })
        .collect();

    // bytes 现在是一个包含所有 BigUint 的字节序列
    println!("bytes: {:?}", bytes);

    let new_bigints: Vec<BigUint> = bytes
        .chunks(8)
        .map(|chunk| {
            println!("{:?}", BigUint::from_bytes_be(chunk));
            BigUint::from_bytes_be(chunk)
        })
        .collect();

    println!("new_bigints{:?}", new_bigints);
    //
    let tmp: BigUint =
        "9760927955188728595712886185812193482151078052825114179109641380640968959312"
            .parse()
            .unwrap();
    let bytes_back = tmp.to_bytes_be();
    println!("tmpbytes_back: {:?}", bytes_back);
    // tmp bytes_back: [21, 148, 125, 76, 19, 249, 169, 41, 199, 70, 117, 180, 150, 239, 231, 95, 41, 238, 205, 3, 150, 85, 33, 29, 207, 176, 154, 78, 192, 56, 101, 80]
}
