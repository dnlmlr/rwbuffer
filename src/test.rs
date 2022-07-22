use crate::*;

#[test]
fn test_rwbuffer() {
    let mut buf = RwBuffer::new();
    buf.put_slice(b"hello");
    buf.put_slice(b"world");

    assert_eq!(buf.as_slice(), b"helloworld");
    assert_eq!(buf.remaining(), 10);

    let mut tmp = [0; 5];

    buf.copy_to_slice(&mut tmp).unwrap();
    assert_eq!(&tmp, b"hello");
    assert_eq!(buf.as_slice(), b"world");
    assert_eq!(buf.remaining(), 5);

    buf.copy_to_slice(&mut tmp).unwrap();
    assert_eq!(&tmp, b"world");
    assert_eq!(buf.as_slice(), &[]);
    assert_eq!(buf.remaining(), 0);

    buf.reset_read_index();

    buf.copy_to_slice(&mut tmp).unwrap();
    assert_eq!(&tmp, b"hello");
    assert_eq!(buf.as_slice(), b"world");
    assert_eq!(buf.remaining(), 5);

    buf.clear_read();

    buf.copy_to_slice(&mut tmp).unwrap();
    assert_eq!(&tmp, b"world");
    assert_eq!(buf.as_slice(), &[]);
    assert_eq!(buf.remaining(), 0);

    buf.reset_read_index();

    buf.copy_to_slice(&mut tmp).unwrap();
    assert_eq!(&tmp, b"world");
    assert_eq!(buf.as_slice(), &[]);
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_rwbuffer_put_get_types() {
    let mut buf = RwBuffer::new();
    buf.put_u8(0x01);
    buf.put_u16(0x0203);
    buf.put_u32(0x04050607);
    buf.put_u64(0x08090a0b0c0d0e0f);
    buf.put_u128(0x101112131415161718191a1b1c1d1e1f);
    buf.put_i8(-0x01);
    buf.put_i16(-0x0203);
    buf.put_i32(-0x04050607);
    buf.put_i64(-0x08090a0b0c0d0e0f);
    buf.put_i128(-0x101112131415161718191a1b1c1d1e1f);
    buf.put_f32(0.1);
    buf.put_f64(0.2);

    assert_eq!(buf.remaining(), 74);

    assert_eq!(buf.get_u8().unwrap(), 0x01);
    assert_eq!(buf.get_u16().unwrap(), 0x0203);
    assert_eq!(buf.get_u32().unwrap(), 0x04050607);
    assert_eq!(buf.get_u64().unwrap(), 0x08090a0b0c0d0e0f);
    assert_eq!(buf.get_u128().unwrap(), 0x101112131415161718191a1b1c1d1e1f);
    assert_eq!(buf.get_i8().unwrap(), -0x01);
    assert_eq!(buf.get_i16().unwrap(), -0x0203);
    assert_eq!(buf.get_i32().unwrap(), -0x04050607);
    assert_eq!(buf.get_i64().unwrap(), -0x08090a0b0c0d0e0f);
    assert_eq!(buf.get_i128().unwrap(), -0x101112131415161718191a1b1c1d1e1f);
    assert_eq!(buf.get_f32().unwrap(), 0.1);
    assert_eq!(buf.get_f64().unwrap(), 0.2);

    assert_eq!(buf.remaining(), 0);
}
