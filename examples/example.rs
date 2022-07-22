use rwbuffer::*;

fn main() {
    let mut buf = RwBuffer::new();

    println!("{:?}\n  {:?}", buf, buf.as_slice());

    buf.put_u8(0x01);
    buf.put_u8(0x02);
    buf.put_u8(0x03);

    println!("{:?}\n  {:?}", buf, buf.as_slice());

    let u = buf.get_u16().unwrap();
    println!("0x{:04x}", u);

    buf.clear_read();

    
    println!("{:?}\n  {:?}", buf, buf.as_slice());
    
}
