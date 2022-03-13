pub fn match_from_end<'a>(source: &'a [u8], regex: &'a [u8]) -> Result<&'a [u8], u8> {
    if source.len() == 0 {
        println!("source len is 0.");
        return Err(2);
    }
    if regex.len() == 0 {
        println!("regex len is 0.");
        return Err(3);
    }
    if source.len() < regex.len() {
        println!("regex is larger than source");
        return Err(4);
    }

    let mut matched_len = 0;
    let mut regex_pos = 0;

    for i in (1..source.len()).rev() {
        regex_pos = regex.len() - matched_len - 1;

        if source[i] == regex[regex_pos] {
            matched_len = matched_len + 1;
        } else {
            matched_len = 0;
        }

        regex_pos = regex.len() - matched_len;
        if regex_pos <= 0 {
            return Ok(&source[i..]);
        }
    }

    Err(1)
}
pub fn u8_to_u16(src: &[u8]) -> u16 {
    (src[1] as u16) << 8 | src[0] as u16
}
pub fn u8_to_u32(src: &[u8]) -> u32 {
    ((((src[3] as u32) << 8 | src[2] as u32) << 8) | src[1] as u32) << 8 | src[0] as u32
}
