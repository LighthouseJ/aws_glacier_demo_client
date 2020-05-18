#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate clap;

mod file_checksum;
mod aws_client;

use log::info;
use clap::{Arg, App};
use std::fs::File;
use crate::aws_client::{AwsCredentials, UploadInfo};


fn main() {

    simple_logger::init();

    let matches = App::new("AWS Glacier uploader")
        .version("0.1.0")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true))
        .arg(Arg::with_name("aws account id")
            .short("i")
            .long("account-id")
            .takes_value(true))
        .arg(Arg::with_name("aws access key")
            .short("k")
            .long("aws-access-key")
            .takes_value(true))
        .arg(Arg::with_name("aws secret key")
            .short("s")
            .long("aws-secret-key")
            .takes_value(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    info!("File: {}", file);

    let mut input = File::open(file).unwrap();

    // let digest = lib::get_sha256_digest(input).unwrap();
    let digest = file_checksum::file_checksum::get_sha256_digest(&input).unwrap();

    info!("sha-256: {}", digest);

    // done
    let aws_info = AwsCredentials
    {
        account_id: matches.value_of("aws account id").unwrap().to_string()
    };

    let upload_info = UploadInfo{
        archive_description: "fill me in!".to_string(),
        // upload_size: 0,
        vault_name: String::from("testvault"),
    };

    let part_size: u64 = 64 * 1024 * 1024;

    aws_client::aws_client::send_file(&aws_info, &upload_info, &input, part_size);
}
