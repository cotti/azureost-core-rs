extern crate sqpack_blue;
extern crate threadpool;
extern crate fallible_iterator;
extern crate sha1;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::path::PathBuf;
use std::fs::{OpenOptions, File};

mod process_all;
mod general_processor;
mod async_data_processor;

pub mod errors;
pub mod manifest;

pub use process_all::process_all;

use errors::AzureError;

pub enum ExportMode {
    #[cfg(feature="lamemp3")]
    MP3(PathBuf),
    OGG(PathBuf),
}

pub struct BGMOptions {
    save_file: Option<File>,
    compare_file: Option<manifest::ManifestFile>,
    export_mode: Option<ExportMode>,
}

pub struct AzureOptions {
    sqpack: String,
    thread_count: usize
}

impl BGMOptions {
    pub fn new(save_file: Option<PathBuf>,
               compare_file: Option<PathBuf>,
               export_mode: Option<ExportMode>) -> Result<BGMOptions, AzureError> {
        save_file.map_or(Ok(None), |f_str| {
            OpenOptions::new().write(true).create_new(true).open(f_str).map_err(|err| {
                AzureError::UnableToCreateSaveFile
            }).map(|f| Some(f))
        }).and_then(|save_file| {
            compare_file.map_or(Ok(None), |f_str| {
                OpenOptions::new().read(true).open(f_str).map_err(|err| {
                    AzureError::UnableToReadCompareFile
                }).and_then(|mut compare_file| {
                    ::serde_json::from_reader::<File, manifest::ManifestFile>(compare_file).map_err(|err| {
                        AzureError::UnableToReadCompareFile
                    })
                }).map(|mf| Some(mf))
            }).map(|compare_file| (save_file, compare_file))
        }).and_then(|(save_file, compare_file)| {
            Ok(BGMOptions {
                save_file,
                compare_file,
                export_mode
            })
        })
    }
}

pub fn export_one() {

}

pub fn bgm_csv() {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
