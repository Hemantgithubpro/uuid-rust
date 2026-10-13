use uuid::Uuid;

fn main() {
    // 1. Generate a brand new UUIDv7
    let my_uuid = Uuid::now_v7();
    println!("Generated UUIDv7: {}", my_uuid);
    // Output: 01944883-fa40-7e56-9b57-a9a3b2b73db7

    // 2. Extract the embedded timestamp data
    if let Some(timestamp) = my_uuid.get_timestamp() {
        // Retrieve Unix time parameters
        let (seconds, fractional_nanoseconds) = timestamp.to_unix();
        
        // Convert to absolute milliseconds
        let total_milliseconds = (seconds * 1000) + (fractional_nanoseconds / 1_000_000) as u64;

        println!("Extracted Unix Seconds: {}", seconds);
        println!("Extracted Milliseconds: {}", total_milliseconds);
    } else {
        println!("This was not a time-based UUID!");
    }
}
