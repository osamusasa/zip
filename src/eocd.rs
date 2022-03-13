use super::util;

#[derive(Debug)]
pub struct Eocd<'a> {
    end_of_central_directory_signature: u32,
    number_of_disk: u16,
    number_of_disk_with_start_cd: u16,
    number_of_cd_on_disk: u16,
    number_of_cd: u16,
    size: u32,
    offset: u32,
    comment_length: u16,
    comment: &'a str,
}

impl<'a> Eocd<'a> {
    pub fn create(src: &'a [u8]) -> Result<Eocd<'a>, u8> {
        if src[0] != 0x50 || src[1] != 0x4B || src[2] != 0x05 || src[3] != 0x06 {
            println!("end of central directory signature is failed");
            return Err(0);
        }

        let ndisk = util::u8_to_u16(&src[4..6]);
        let ndisk_with_cd = util::u8_to_u16(&src[6..8]);
        let ncd_on_disk = util::u8_to_u16(&src[8..10]);
        let ncd = util::u8_to_u16(&src[10..12]);
        let size = util::u8_to_u32(&src[12..16]);
        let offset = util::u8_to_u32(&src[16..20]);
        let comment_len = util::u8_to_u16(&src[20..22]);

        Ok(Eocd{
            end_of_central_directory_signature: 0x504B0506,
            number_of_disk: ndisk,
            number_of_disk_with_start_cd: ndisk_with_cd,
            number_of_cd_on_disk: ncd_on_disk,
            number_of_cd: ncd,
            size: size,
            offset: offset,
            comment_length: comment_len,
            comment: "",
        })
    }
}
