use std::io::Cursor;

use rwbuffer::*;

struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let pos = Pos {
        x: 100.0,
        y: 123.5,
        z: -12.0,
    };

    // Build a packet for a simple made up protocol.
    // Position packet:
    // - packet length: u16
    // - packet type: u16 (1 = position packet)
    // - x pos: f64
    // - y pos: f64
    // - z pos: f64
    // - simple xor checksum: u8
    let mut buf_out = RwBuffer::new();

    // Packet length (2 bytes)
    buf_out.put_u16(27);
    // Packet type (2 bytes)
    buf_out.put_u16(1);
    // X Position (8 bytes)
    buf_out.put_f64(pos.x);
    // Y Position (8 bytes)
    buf_out.put_f64(pos.y);
    // Z Position (8 bytes)
    buf_out.put_f64(pos.z);

    // Checksum (1 byte)
    let chk = buf_out.as_slice().iter().fold(0, |chk, b| chk ^ *b);
    buf_out.put_u8(chk);

    // Simulated send
    let data = buf_out.as_slice();
    println!("Simulate sending packet over network: {:?}", data);
    let mut reader = Cursor::new(data);

    // Exact packet size is unknown, so reserve enough capacity to hold the larges packet
    // Expect 2 bytes for the length
    let mut buf_in = RwBuffer::with_capacity_and_expected(64, 2);

    // Read the 2 byte length
    buf_in.read_exact_from(&mut reader).unwrap();
    // Update the checksum before the `get`, since `get` will remove the bytes from the buffer
    let mut chk_verify = buf_in.as_slice().iter().fold(0, |chk, b| chk ^ *b);
    // Extract the 2 byte length as an u16
    let pack_len = buf_in.get_u16().unwrap();

    println!("Read packet-len: {pack_len}");

    // Update the expected length to the new packet length (2 bytes for len + the packet length)
    buf_in.set_expected(2 + pack_len as usize);
    // Read the missing bytes from the reader
    buf_in.read_exact_from(&mut reader).unwrap();

    // Update the checksum (excluding the last byte, which is the sent checksum itself)
    chk_verify ^= buf_in
        .as_slice()
        .iter()
        .take(pack_len as usize - 1)
        .fold(0, |chk, b| chk ^ *b);

    // Extract the packet contents from the buffer
    let pack_type = buf_in.get_u16().unwrap();

    // In the real world this would be behind something like a match on the type
    let x = buf_in.get_f64().unwrap();
    let y = buf_in.get_f64().unwrap();
    let z = buf_in.get_f64().unwrap();

    let chk = buf_in.get_u8().unwrap();

    // Clear the read portion of the buffer, this resets the cursor and will reuse the buffer. If 
    // no data is left over (unread), this is a cheap zero copy operation.
    // This is important if the buffer is reused, since without it the buffer will grow 
    // indefinitely and not get reused
    buf_in.clear_read();

    // Print the decoded contents
    println!("Read packet-type: {pack_type}");
    println!("Read x: {x}");
    println!("Read y: {y}");
    println!("Read z: {z}");
    println!("Read checksum: {chk} (calculated: {chk_verify})");
}
