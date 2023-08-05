// demo contract
//pragma solidity >=0.4.16 <0.9.0;

// contract Foo {
//     function bar(bytes3[2] memory) public pure {}
//     function baz(uint32 x, bool y) public pure returns (bool r) { r = x > 32 || y; }
//     function sam(bytes memory, bool, uint[] memory) public pure {}
// }
// Although uint is same as uint256 in solidity.
// but uint and uint256 is different when in Selector.
// You need to declare clearly about uint256.

use byteorder::{BigEndian, ByteOrder};
use chrono::offset;
use ethereum_types::U256;

fn main() {
    // (Selector)选择器：
    // 4 bytes  :0xa5643bf2
    // 数据：
    // 0-32 bytes :0x0000000000000000000000000000000000000000000000000000000000000060 -> 96
    // 32-64 bytes :0x0000000000000000000000000000000000000000000000000000000000000001
    // 64-96 bytes :0x00000000000000000000000000000000000000000000000000000000000000a0 -> 160
    // 96-128 bytes :0x0000000000000000000000000000000000000000000000000000000000000004
    // 128-160 bytes :0x6461766500000000000000000000000000000000000000000000000000000000
    // 160-192 bytes :0x0000000000000000000000000000000000000000000000000000000000000003
    // 192-224 bytes :0x0000000000000000000000000000000000000000000000000000000000000001
    // 224-256 bytes :0x0000000000000000000000000000000000000000000000000000000000000002
    // 256-288 bytes :0x0000000000000000000000000000000000000000000000000000000000000003

    // assume sam function parameters are: "dave", true and [1,2,3]
    // let data = "";

    // data is a hex number
    let data = hex::decode("a5643bf20000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000464617665000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003").expect("decode error");

    let (function_selector, argument_encoding) = data.split_at(4);
    println!("function_selector: {:?}", function_selector);
    // argument_encoding
    // parameter 1 is bytes type, which is 32 bytes to show where is the start of the data,
    // which is 0x60 means we can read the truely data at argument_encoding since 96 bytes.
    // from 96 to 128 is the length of the byte array in elements

    // data[0..32] is the offset of the first parameter
    let mut offset = 0;
    let next = || -> &[u8] {
        let start = offset;
        let offset = offset + 32;
        &argument_encoding[start..offset]
    };
    let parameter_1_start_offset = U256::from_big_endian(next()).low_u32() as usize;
    println!("parameter_1_start_offset: {:?}", parameter_1_start_offset);
    let parameter_2_start_offset = U256::from_big_endian(next()).low_u32() as usize;

    let parameter_1_length = U256::from_big_endian(
        &argument_encoding[parameter_1_start_offset..parameter_1_start_offset + 32],
    )
    .low_u32() as usize;
    println!("parameter_1_length: {:?}", parameter_1_length);

    let parameter_2_length = U256::from_big_endian(
        &argument_encoding[parameter_2_start_offset..parameter_2_start_offset + 32],
    );
    println!("parameter_2_length: {:?}", parameter_2_length);
}
