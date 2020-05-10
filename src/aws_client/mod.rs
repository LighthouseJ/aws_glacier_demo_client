
// use std::fs::File;
// use log::info;

// use rusoto_core::{Region, RusotoError};
// use rusoto_glacier::{GlacierClient, Glacier, InitiateMultipartUploadOutput, InitiateMultipartUploadError};

#[derive(Debug)]
struct ByteRange
{
    start: usize,
    end: usize
}


pub mod aws_client
{

    use log::info;
    use rusoto_glacier::{Glacier, InitiateMultipartUploadInput, UploadMultipartPartInput};
    use crate::aws_client::ByteRange;

    pub fn send_file(account_id: &str, file: &std::fs::File) -> Result<(), &'static str>
    {

        let region = rusoto_core::Region::UsEast1;

        let glacier_client = rusoto_glacier::GlacierClient::new(region);

        let part_size: u64 = 64 * 1024 * 1024;
        info!("Using part_size={}", part_size);
        let multipart_upload = rusoto_glacier::InitiateMultipartUploadInput
        {
            account_id: account_id.to_string(),
            archive_description: None,
            part_size: Some(part_size.to_string()),
            vault_name: "testvault".to_string()
        };

        // figure out number of parts

        // let input_file_metadata = std::fs::metadata(file).unwrap();
        let input_file_metadata = file.metadata().unwrap();

        let num_parts = input_file_metadata.len() / part_size;
        let num_remainder = input_file_metadata.len() % part_size;
        info!("File size={}, have {} parts, {} remainder", input_file_metadata.len(),
              num_parts, num_remainder);

        // glacier_client.initiate_multipart_upload(multipart_upload);

        // glacier_client.initiate_multipart_upload()
        let mut upload_parts: Vec<ByteRange> = Vec::new();
        // upload_parts.push(ByteRange{start: 0, end: num_remainder as usize });
        let mut next_start_pos: usize = 0;
        for x in 0 .. num_parts {
            let this_end_pos: usize = (part_size * (x + 1)) as usize;
            upload_parts.push(ByteRange{start: next_start_pos, end: this_end_pos});
            next_start_pos = this_end_pos + 1;
        }
        upload_parts.push(ByteRange{ start: next_start_pos, end: num_remainder as usize});

        info!("parts to send are: {:?}", upload_parts);

        unimplemented!("not finished!");
    }
}


