//! A protobuf encoder written by hand (wire format), independent of `prost`, for
//! example Sparkplug B messages.

pub fn varint(mut v: u64, out: &mut Vec<u8>) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            break;
        }
        out.push(b | 0x80);
    }
}

pub fn key(field: u32, wire: u8, out: &mut Vec<u8>) {
    varint(u64::from(field) << 3 | u64::from(wire), out);
}

pub fn uint(field: u32, v: u64, out: &mut Vec<u8>) {
    key(field, 0, out);
    varint(v, out);
}

pub fn bytes(field: u32, b: &[u8], out: &mut Vec<u8>) {
    key(field, 2, out);
    varint(b.len() as u64, out);
    out.extend_from_slice(b);
}

pub fn double(field: u32, v: f64, out: &mut Vec<u8>) {
    key(field, 1, out);
    out.extend_from_slice(&v.to_le_bytes());
}

pub enum V<'a> {
    Int(u32),
    Long(u64),
    Double(f64),
    Bool(bool),
    Str(&'a str),
}

pub fn metric(name: Option<&str>, alias: Option<u64>, datatype: u32, v: V) -> Vec<u8> {
    let mut m = Vec::new();
    if let Some(n) = name {
        bytes(1, n.as_bytes(), &mut m);
    }
    if let Some(a) = alias {
        uint(2, a, &mut m);
    }
    uint(3, 1_700_000_000_123, &mut m);
    uint(4, u64::from(datatype), &mut m);
    // An unknown field (properties, 9) the decoder must skip.
    bytes(9, &[0x0a, 0x01, b'u'], &mut m);
    match v {
        V::Int(x) => uint(10, u64::from(x), &mut m),
        V::Long(x) => uint(11, x, &mut m),
        V::Double(x) => double(13, x, &mut m),
        V::Bool(x) => uint(14, u64::from(x), &mut m),
        V::Str(s) => bytes(15, s.as_bytes(), &mut m),
    }
    m
}

pub fn payload(seq: u64, metrics: &[Vec<u8>]) -> Vec<u8> {
    let mut p = Vec::new();
    uint(1, 1_700_000_000_000, &mut p);
    for m in metrics {
        bytes(2, m, &mut p);
    }
    uint(3, seq, &mut p);
    p
}
