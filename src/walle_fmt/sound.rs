use std::fs::File;
use std::io::{Cursor, ErrorKind};
use std::io::{Error, Write};
use std::path::Path;

use binwrite::BinWrite;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{write_option, HasReferences, WALLEObjectFormatTrait};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct SoundZHeader {
    friendly_name_crc32: u32,
    sample_rate: u32,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    data_size: Option<u32>,
    #[nom(Cond = "sample_rate != 0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    #[nom(Verify(*sound_type == 1 || *sound_type == 3 || *sound_type == 5 || *sound_type == 7))]
    sound_type: Option<u16>,
    #[nom(Cond = "sample_rate != 0 && i.len() == 2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    zero: Option<u16>,
}

impl HasReferences for SoundZHeader {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub struct SoundObjectFormat;

impl SoundObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl WALLEObjectFormatTrait for SoundObjectFormat {
    fn pack(
        self: &Self,
        input_path: &Path,
        header: &mut Vec<u8>,
        body: &mut Vec<u8>,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let wav_path = input_path.join("data.wav");
        let mut reader = hound::WavReader::open(wav_path).unwrap();

        #[derive(Deserialize)]
        struct Object {
            sound_header: SoundZHeader,
        }

        let object: Object = serde_json::from_reader(json_file)?;

        object.sound_header.write(header)?;

        for sample in reader.samples::<i16>() {
            body.write_i16::<LittleEndian>(sample.unwrap())?;
        }

        Ok((
            object.sound_header.hard_links(),
            object.sound_header.soft_links(),
        ))
    }

    fn unpack(
        self: &Self,
        header: &[u8],
        body: &[u8],
        output_path: &Path,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let wav_path = output_path.join("data.wav");

        let sound_header = match SoundZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(_) => return Err(Error::from(ErrorKind::Other)),
        };

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: if sound_header.sample_rate != 0 {
                sound_header.sample_rate
            } else {
                44100
            },
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let number_of_samples = body.len() as u32 / (spec.bits_per_sample / 8) as u32;

        let mut parent_writer = hound::WavWriter::create(wav_path, spec).unwrap();
        let mut writer = parent_writer.get_i16_writer(number_of_samples);

        let mut data_cursor = Cursor::new(&body);

        for _ in 0..number_of_samples {
            writer.write_sample(data_cursor.read_i16::<LittleEndian>()?);
        }
        writer.flush().unwrap();
        parent_writer.finalize().unwrap();

        #[derive(Serialize)]
        struct Object {
            sound_header: SoundZHeader,
        }

        let object = Object { sound_header };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok((
            object.sound_header.hard_links(),
            object.sound_header.soft_links(),
        ))
    }
}
