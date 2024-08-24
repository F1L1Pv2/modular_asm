pub fn little_endian_u16(input: &[u8]) -> Option<u16>{

    if input.len() < 2{
        return None;
    }

    let lower_byte = input[0];
    let higher_byte = input[1];

    return Some(((higher_byte as u16) << 8) | lower_byte as u16);
}

pub fn little_endian_u32(input: &[u8]) -> Option<u32>{

    if input.len() < 4{
        return None;
    }

    let lower_u16 = little_endian_u16(&input[0..=1]).unwrap();
    let higher_u16 = little_endian_u16(&input[2..=3]).unwrap();

    return Some(((higher_u16 as u32) << 16) | lower_u16 as u32);
}

pub fn big_endian_u16(input: &[u8]) -> Option<u16>{

    if input.len() < 2{
        return None;
    }

    let lower_byte = input[1];
    let higher_byte = input[0];

    return Some(((higher_byte as u16) << 8) | lower_byte as u16);
}

pub fn big_endian_u32(input: &[u8]) -> Option<u32>{

    if input.len() < 4{
        return None;
    }

    let lower_u16 = little_endian_u16(&input[2..=3]).unwrap();
    let higher_u16 = little_endian_u16(&input[0..=1]).unwrap();

    return Some(((higher_u16 as u32) << 16) | lower_u16 as u32);
}