extern crate clap;

mod file_checksum;

use log::info;
use clap::{Arg, App};
use std::fs::File;

use rusoto_core::{Region, RusotoError};
use rusoto_glacier::{GlacierClient, Glacier, InitiateMultipartUploadOutput, InitiateMultipartUploadError};
use std::future::Future;

fn main() {
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
    println!("File: {}", file);

    let mut input = File::open(file).unwrap();

    // let digest = lib::get_sha256_digest(input).unwrap();
    // let digest = file_checksum::get_sha256_digest(input).unwrap();
    let digest = file_checksum::

    println!("sha-256: {}", digest);

    let region = rusoto_core::Region::UsEast1;

    let glacier_client = rusoto_glacier::GlacierClient::new(region);

    let part_size: u64 = 64 * 1024 * 1024;
    info!("Using part_size={}", part_size);
    let multipart_upload = rusoto_glacier::InitiateMultipartUploadInput
    {
        account_id: matches.value_of("aws account id").unwrap().to_string(),
        archive_description: None,
        part_size: Some(part_size.to_string()),
        vault_name: "testvault".to_string()
    };

    // figure out number of parts

    let input_file_metadata = std::fs::metadata(file).unwrap();

    let num_parts = input_file_metadata.len() / part_size;
    let num_remainder = input_file_metadata.len() % part_size;
    info!("Have {} parts, {} remainder", num_parts, num_remainer);

}
