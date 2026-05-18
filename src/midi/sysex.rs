pub enum ParsedMessage {
    SysExKatana { target_msb: u8, address: [u8; 4], data: Vec<u8> },
    SysExIdentityReply { manufacturer: u8, family: [u8; 2], member: [u8; 2] },
    SysExOther,
    NoteOn        { channel: u8, note: u8, velocity: u8 },
    NoteOff       { channel: u8, note: u8, velocity: u8 },
    ControlChange { channel: u8, controller: u8, value: u8 },
    ProgramChange { channel: u8, program: u8 },
    PitchBend     { channel: u8, value: i16 },
    Unknown,
}

pub fn roland_checksum(payload: &[u8]) -> u8 {
    let sum: u32 = payload.iter().map(|&x| x as u32).sum();
    let rem = (sum % 128) as u8;
    if rem == 0 { 0 } else { 128 - rem }
}

pub fn build_sysex(address: [u8; 4], payload_data: &[u8]) -> Vec<u8> {
    let mut msg = vec![0xF0, 0x41, 0x00, 0x00, 0x00, 0x00, 0x33, 0x12];
    let mut checksum_payload = address.to_vec();
    checksum_payload.extend_from_slice(payload_data);
    msg.extend_from_slice(&checksum_payload);
    msg.push(roland_checksum(&checksum_payload));
    msg.push(0xF7);
    msg
}

pub fn parse_midi(bytes: &[u8]) -> ParsedMessage {
    if bytes.is_empty() { return ParsedMessage::Unknown; }
    let first = bytes[0];

    if first == 0xF0 && bytes.len() >= 15
        && bytes[1] == 0x7E && bytes[3] == 0x06 && bytes[4] == 0x02
    {
        return ParsedMessage::SysExIdentityReply {
            manufacturer: bytes[5],
            family: [bytes[6], bytes[7]],
            member: [bytes[8], bytes[9]],
        };
    }

    if first == 0xF0 && bytes.len() >= 14 {
        if bytes.len() >= 13 && bytes[1] == 0x41 && bytes[6] == 0x33 
            && (bytes[7] == 0x12 || bytes[7] == 0x11) 
        {
            let address = [bytes[8], bytes[9], bytes[10], bytes[11]];
            let end = bytes.len().saturating_sub(2);
            if end > 12 {
                let data = bytes[12..end].to_vec();
                return ParsedMessage::SysExKatana { target_msb: bytes[8], address, data };
            }
        }
        return ParsedMessage::SysExOther;
    }

    let status = first & 0xF0;
    let channel = first & 0x0F;
    let d1 = *bytes.get(1).unwrap_or(&0);
    let d2 = *bytes.get(2).unwrap_or(&0);
    match status {
        0x80 => ParsedMessage::NoteOff { channel, note: d1, velocity: d2 },
        0x90 => ParsedMessage::NoteOn { channel, note: d1, velocity: d2 },
        0xB0 => ParsedMessage::ControlChange { channel, controller: d1, value: d2 },
        0xC0 => ParsedMessage::ProgramChange { channel, program: d1 },
        0xE0 => ParsedMessage::PitchBend { channel, value: ((d2 as i16) << 7 | (d1 as i16)) - 8192 },
        _ => ParsedMessage::Unknown,
    }
}