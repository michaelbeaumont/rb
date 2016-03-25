extern crate rb;

use rb::{RB, SpscRb, RbInspector, RbProducer, RbConsumer};

#[test]
fn test_write() {
    const SIZE: usize = 128;
    let rb = SpscRb::new(SIZE);
    let producer = rb.producer();
    assert!(rb.is_empty());
    assert_eq!(rb.slots_free(), SIZE);
    assert_eq!(rb.count(), 0);
    let data = (0..SIZE).collect::<Vec<_>>();
    for i in 0..8 {
        let slice = &data[i * 16..(i + 1) * 16];
        producer.write(slice).unwrap();
        assert_eq!(rb.count(), (i + 1) * 16);
        assert_eq!(rb.slots_free(), SIZE - (i + 1) * 16);
    }
    assert!(rb.is_full());
}

#[test]
fn test_read() {
    const SIZE: usize = 128;
    let rb = SpscRb::new(SIZE);
    let (consumer, producer) = (rb.consumer(), rb.producer());
    assert!(rb.is_empty());
    let in_data = (0..SIZE).map(|i| i * 2).collect::<Vec<_>>();
    producer.write(&in_data).unwrap();
    assert!(rb.is_full());
    let mut out_data = vec![0; SIZE];
    consumer.read(&mut out_data).unwrap();
    assert_eq!(out_data, in_data);
    assert!(rb.is_empty());
}

#[test]
fn test_clear() {
    const SIZE: usize = 128;
    let rb = SpscRb::new(SIZE);
    let (consumer, producer) = (rb.consumer(), rb.producer());
    assert!(rb.is_empty());
    let in_data = (0..SIZE).map(|i| i * 2).collect::<Vec<_>>();
    producer.write(&in_data).unwrap();
    assert!(rb.is_full());
    rb.clear();
    assert!(rb.is_empty());
    producer.write(&in_data).unwrap();
    assert!(rb.is_full());
    let mut out_data = vec![0; SIZE];
    consumer.read(&mut out_data).unwrap();
    assert_eq!(out_data, in_data);
    assert!(rb.is_empty());
}

#[test]
fn test_wrap_around() {
    const SIZE: usize = 128;
    let rb = SpscRb::new(SIZE);
    let (consumer, producer) = (rb.consumer(), rb.producer());
    let in_data = (0..SIZE * 2).map(|i| i * 2).collect::<Vec<_>>();
    producer.write(&in_data[0..64]).unwrap();
    assert_eq!(rb.count(), 64);
    let mut out_data = vec![0; SIZE*2];
    // TODO: try to read more
    consumer.read(&mut out_data[0..64]).unwrap();
    assert!(rb.is_empty());
    producer.write(&in_data[64..64 + SIZE]).unwrap();
    assert_eq!(rb.count(), 128);
    assert!(rb.is_full());
    consumer.read(&mut out_data[64..64 + SIZE]).unwrap();
    assert!(rb.is_empty());
    producer.write(&in_data[64 + SIZE..]).unwrap();
    assert_eq!(rb.count(), 64);
    consumer.read(&mut out_data[64 + SIZE..]).unwrap();
    assert_eq!(in_data, out_data);
}
