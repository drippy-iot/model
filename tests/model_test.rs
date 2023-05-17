use model::{decode, encode, Header, Message, Payload};

#[test]
fn test_leak_report_1() {
    // Test the data model with a Conflict payload
    // when it is encoded into bytes.

    let message: Message = Message {
        head: Header {
            mac: [0x2a, 0x3b, 0x2c, 0x03, 0x33, 0xba],
            timestamp: 0xffffffffffffffff,
        },
        data: Payload::Conflict,
    };

    let encoded: Vec<u8> = encode(&message).unwrap();
    let actual: Vec<u8> = vec![
        0x2a, 0x3b, 0x2c, 0x03, 0x33, 0xba, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
    ];

    assert_eq!(encoded, actual);

    println!("{}", format!("{encoded:?}"));
}

#[test]
fn test_leak_report_2() {
    // Test Bytes with Conflict payload
    // when it is decoded into Message.

    let bytes: Vec<u8> = vec![
        0x45, 0x12, 0x5b, 0xab, 0x9a, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
    ];

    let test_message: Message = decode(&bytes).unwrap();

    let actual: Message = Message {
        head: Header {
            mac: [0x45, 0x12, 0x5b, 0xab, 0x9a, 0xff],
            timestamp: 0xff00ff00ff00ff00,
        },
        data: Payload::Conflict,
    };

    // We don't have an equal comparator for the Message struct
    // so this will have to do.
    let test_encode = encode(&test_message).unwrap();
    let test_actual = encode(&actual).unwrap();

    assert_eq!(test_encode, test_actual);
}

#[test]
fn test_data_report_1() {
    // Test the data model with a Flow data payload
    // when it is encoded into bytes.

    let message: Message = Message {
        head: Header {
            mac: [0x45, 0x12, 0x5b, 0xab, 0x9a, 0xff],
            timestamp: 0xff00ff00ff00ff00,
        },
        data: Payload::Flow {
            ticks: 0xffffffffffffffff,
        },
    };

    let encoded: Vec<u8> = encode(&message).unwrap();
    let actual: Vec<u8> = vec![
        0x45, 0x12, 0x5b, 0xab, 0x9a, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
    ];

    assert_eq!(encoded, actual);
    println!("{}", format!("{encoded:?}"));
}

#[test]
fn test_data_report_2() {
    // Test Bytes with Flow data payload
    // when it is decoded into Message.

    let bytes: Vec<u8> = vec![
        0x2a, 0x3b, 0x2c, 0x03, 0x33, 0xba, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        0xfe, 0x01, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 1,
    ];

    let test_message: Message = decode(&bytes).unwrap();

    let actual: Message = Message {
        head: Header {
            mac: [0x2a, 0x3b, 0x2c, 0x03, 0x33, 0xba],
            timestamp: 0xffffffffffffffff,
        },
        data: Payload::Flow {
            ticks: 0xff00ff00ff00ff00,
        },
    };

    // We don't have an equal comparator for the Message struct
    // so this will have to do.
    let test_encode = encode(&test_message).unwrap();
    let test_actual = encode(&actual).unwrap();

    assert_eq!(test_encode, test_actual);
}
