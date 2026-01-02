pub fn write_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}

pub fn write_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend(&v.to_le_bytes());
}

pub fn write_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend(&v.to_le_bytes());
}

pub fn write_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend(&v.to_le_bytes());
}

pub fn write_f32(buf: &mut Vec<u8>, v: f32) {
    buf.extend(&v.to_le_bytes());
}

// -------- read --------

pub fn read_u8(data: &[u8], c: &mut usize) -> Option<u8> {
    let v = *data.get(*c)?;
    *c += 1;
    Some(v)
}

pub fn read_u16(data: &[u8], c: &mut usize) -> Option<u16> {
    let s = data.get(*c..*c + 2)?;
    *c += 2;
    Some(u16::from_le_bytes(s.try_into().ok()?))
}

pub fn read_u32(data: &[u8], c: &mut usize) -> Option<u32> {
    let s = data.get(*c..*c + 4)?;
    *c += 4;
    Some(u32::from_le_bytes(s.try_into().ok()?))
}

pub fn read_u64(data: &[u8], c: &mut usize) -> Option<u64> {
    let s = data.get(*c..*c + 8)?;
    *c += 8;
    Some(u64::from_le_bytes(s.try_into().ok()?))
}

pub fn read_f32(data: &[u8], c: &mut usize) -> Option<f32> {
    let s = data.get(*c..*c + 4)?;
    *c += 4;
    Some(f32::from_le_bytes(s.try_into().ok()?))
}
