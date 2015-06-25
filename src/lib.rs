use std::str::Bytes;

fn to_op_code (byte : u8) -> OpCode {
    match byte {
        41 => OpCode::Mark,
        108 => OpCode::List,
        _ => unimplemented!()
    }
}

fn bytes_to_op_codes(bytes : Bytes) -> Vec<OpCode> {
    bytes.map( |byte| {
        to_op_code(byte)
    }).collect()
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Mark,
    List,
    Put,
    Pop,
    Integer,
    Append,
    Stop
}

#[cfg(test)]
fn b(input : &str) -> u8 {
    input.bytes().next().unwrap()
}

#[test]
fn test_bytes_to_op_codes() {
    let buffer = ")l";
    let bytes = buffer.bytes();
    let result = bytes_to_op_codes(bytes);
    let expected = vec![OpCode::Mark, OpCode::List];

    assert_eq!(result, expected);
}

#[test]
fn test_to_op_code() {
    assert_eq!(to_op_code(b(")")), OpCode::Mark);
    assert_eq!(to_op_code(b("l")), OpCode::List);
}
