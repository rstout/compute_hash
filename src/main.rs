use anchor_lang::{
    prelude::*,
    solana_program::keccak,
};

pub fn compute_output_root_message_hash(
    output_root: &[u8; 32],
    base_block_number: u64,
    total_leaf_count: u64,
) -> [u8; 32] {
    let mut message_bytes = Vec::with_capacity(32 + 8 + 8);
    message_bytes.extend_from_slice(output_root);
    message_bytes.extend_from_slice(&base_block_number.to_be_bytes());
    message_bytes.extend_from_slice(&total_leaf_count.to_be_bytes());

    let prefix: &[u8] = b"\x19Ethereum Signed Message:\n";
    let len_dec_string = message_bytes.len().to_string();

    let mut prefixed =
        Vec::with_capacity(prefix.len() + len_dec_string.len() + message_bytes.len());
    prefixed.extend_from_slice(prefix);
    prefixed.extend_from_slice(len_dec_string.as_bytes());
    prefixed.extend_from_slice(&message_bytes);

    keccak::hash(&prefixed).0
}

fn main() {
    println!("=== Compute Output Root Message Hash Demo ===\n");

    let output_root: [u8; 32] = [
        0xd2, 0xcb, 0xe8, 0xc1, 0x85, 0xb6, 0x9c, 0x03,
        0x12, 0x22, 0x9a, 0x23, 0xd8, 0xf4, 0xed, 0x6c,
        0x4c, 0x18, 0xb7, 0x78, 0xaf, 0x61, 0xce, 0x3f,
        0x75, 0x5c, 0x9c, 0x2c, 0x1e, 0x6c, 0x23, 0xee,
    ];
    let base_block_number: u64 = 30624374;
    let total_leaf_count: u64 = 2;

    println!("Input Parameters:");
    println!("  Output Root:       0x{}", hex::encode(&output_root));
    println!("  Base Block Number: {} (0x{:016x})", base_block_number, base_block_number);
    println!("  Total Leaf Count:  {} (0x{:016x})", total_leaf_count, total_leaf_count);
    println!();

    println!("Intermediate Steps:");
    println!("1. Original message bytes (48 bytes total):");
    
    let mut message_bytes = Vec::with_capacity(32 + 8 + 8);
    message_bytes.extend_from_slice(&output_root);
    message_bytes.extend_from_slice(&base_block_number.to_be_bytes());
    message_bytes.extend_from_slice(&total_leaf_count.to_be_bytes());
    
    println!("   - Output Root (32 bytes):       0x{}", hex::encode(&output_root));
    println!("   - Block Number BE (8 bytes):    0x{}", hex::encode(&base_block_number.to_be_bytes()));
    println!("   - Leaf Count BE (8 bytes):      0x{}", hex::encode(&total_leaf_count.to_be_bytes()));
    println!("   - Combined message:              0x{}", hex::encode(&message_bytes));
    println!();

    println!("2. EIP-191 Ethereum Signed Message prefix:");
    let prefix: &[u8] = b"\x19Ethereum Signed Message:\n";
    let len_dec_string = message_bytes.len().to_string();
    
    println!("   - Prefix bytes:                  {:?}", std::str::from_utf8(prefix).unwrap());
    println!("   - Prefix hex:                    0x{}", hex::encode(prefix));
    println!("   - Message length (decimal):      \"{}\"", len_dec_string);
    println!("   - Message length bytes:          0x{}", hex::encode(len_dec_string.as_bytes()));
    println!();

    println!("3. Constructing prefixed message:");
    let mut prefixed =
        Vec::with_capacity(prefix.len() + len_dec_string.len() + message_bytes.len());
    prefixed.extend_from_slice(prefix);
    prefixed.extend_from_slice(len_dec_string.as_bytes());
    prefixed.extend_from_slice(&message_bytes);
    
    println!("   - Total prefixed length:         {} bytes", prefixed.len());
    println!("   - Prefixed message (hex):");
    println!("     0x{}", hex::encode(&prefixed));
    println!();

    println!("4. Computing keccak256 hash:");
    let hash = compute_output_root_message_hash(&output_root, base_block_number, total_leaf_count);
    
    println!("   Final Hash: 0x{}", hex::encode(&hash));
    println!();

    println!("=== Testing with different inputs ===\n");
    
    let test_cases = vec![
        (
            [0u8; 32],
            0u64,
            0u64,
            "All zeros"
        ),
        (
            [0xffu8; 32],
            u64::MAX,
            u64::MAX,
            "All max values"
        ),
        (
            {
                let mut arr = [0u8; 32];
                arr[0] = 0xde;
                arr[1] = 0xad;
                arr[2] = 0xbe;
                arr[3] = 0xef;
                arr
            },
            1000000,
            50,
            "Custom values"
        ),
    ];

    for (i, (root, block, count, desc)) in test_cases.iter().enumerate() {
        println!("Test Case {}: {}", i + 1, desc);
        println!("  Output Root: 0x{}", hex::encode(root));
        println!("  Block: {}, Leaf Count: {}", block, count);
        
        let result = compute_output_root_message_hash(root, *block, *count);
        println!("  Hash: 0x{}", hex::encode(&result));
        println!();
    }
}