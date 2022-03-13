/// ヘッダーを表す構造体
pub mod local_file_header {
    #[derive(Debug)]
    pub struct ZipHeader<'a> {
        local_file_header_signature: u32,
        version_need_to_extract: u16,
        general_purpose_bit_flg: u16,
        compression_method: u16,
        last_mod_file_time: u16,
        last_mod_file_date: u16,
        crc_32: u32,
        compressed_size: u32,
        uncompressed_size: u32,
        file_name_length: u16,
        extra_field_length: u16,
        file_name: &'a str,
        extra_field: &'a str,
    }

    impl<'a> ZipHeader<'a> {
        /// ヘッダーの初期化
        pub fn new() -> ZipHeader<'a> {
            ZipHeader {
                local_file_header_signature: 0x04034b50,
                version_need_to_extract: 0,
                general_purpose_bit_flg: 0,
                compression_method: 0,
                last_mod_file_time: 0,
                last_mod_file_date: 0,
                crc_32: 0,
                compressed_size: 0,
                uncompressed_size: 0,
                file_name_length: 0,
                extra_field_length: 0,
                file_name: "",
                extra_field: "",
            }
        }

        /// バージョンを設定する
        pub fn set_version(&mut self, major: u8, miner: u8) {
            self.set_major_version(major);
            self.set_miner_version(miner);
        }

        /// メジャーバージョンを設定する
        pub fn set_major_version(&mut self, major: u8) {
            if 9 < major {
                println!(
                    "major version range is out of bound. Must be 0 <= major version <= 9: {}",
                    major
                );
            } else {
                self.version_need_to_extract =
                    (major * 10 + self.get_miner_version()).try_into().unwrap();
            }
        }

        /// マイナーバージョンを設定する
        pub fn set_miner_version(&mut self, miner: u8) {
            if 9 < miner {
                println!(
                    "miner version range is out of bound. Must be 0 <= miner version <= 9: {}",
                    miner
                );
            } else {
                self.version_need_to_extract =
                    (self.get_major_version() * 10 + miner).try_into().unwrap();
            }
        }

        /// メジャーバージョンを取得する
        pub fn get_major_version(&self) -> u8 {
            (self.version_need_to_extract / 10).try_into().unwrap()
        }

        /// マイナーバージョンを取得する
        pub fn get_miner_version(&self) -> u8 {
            (self.version_need_to_extract % 10).try_into().unwrap()
        }

        /// 最終更新日時を設定する
        pub fn set_last_mod_date_time(
            &mut self,
            seconds: u32,
            min: u32,
            hour: u32,
            day: u32,
            month: u32,
            year: u32,
        ) {
            if 60 < seconds {
                println!(
                    "validate error: last mod date and time of seconds: {}",
                    seconds
                );
            }
            if 59 < min {
                println!("validate error: last mod date and time of min: {}", min);
            }
            if 24 < hour {
                println!("validate error: last mod date and time of hour: {}", hour);
            }
            if 31 < day {
                println!("validate error: last mod date and time of day: {}", day);
            }
            if 12 < month {
                println!("validate error: last mod date and time of month: {}", month);
            }
            if !(2 << 8) < year {
                println!("validate error: last mod date and time of year: {}", year);
            }

            self.last_mod_file_time = (((hour << 11) & 0b1111_1000_0000_0000)
                | ((min << 5) & 0b0000_0111_1110_0000)
                | ((seconds / 2 << 0) & 0b0000_0000_0001_1111))
                .try_into()
                .unwrap();
            self.last_mod_file_date = ((((year - 1980) << 9) & 0b1111_1110_0000_0000)
                | ((month << 5) & 0b0000_0001_1110_0000)
                | ((day << 0) & 0b0000_0000_0001_1111))
                .try_into()
                .unwrap();
        }

        /// 最終更新日時の年を取得
        pub fn get_last_mod_year(&self) -> u16 {
            ((self.last_mod_file_date & 0b1111_1110_0000_0000) >> 9) + 1980
        }

        /// 最終更新日時の月を取得
        pub fn get_last_mod_month(&self) -> u16 {
            (self.last_mod_file_date & 0b0000_0001_1110_0000) >> 5
        }

        /// 最終更新日時の日を取得
        pub fn get_last_mod_day(&self) -> u16 {
            (self.last_mod_file_date & 0b0000_0000_0001_1111) >> 0
        }

        /// 最終更新日時の時を取得
        pub fn get_last_mod_hour(&self) -> u16 {
            (self.last_mod_file_time & 0b1111_1000_0000_0000) >> 11
        }

        /// 最終更新日時の分を取得
        pub fn get_last_mod_min(&self) -> u16 {
            (self.last_mod_file_time & 0b0000_0111_1110_0000) >> 5
        }

        /// 最終更新日時の秒を取得
        pub fn get_last_mod_seconds(&self) -> u16 {
            ((self.last_mod_file_time & 0b0000_0000_0001_1111) >> 0) * 2
        }
    }
}