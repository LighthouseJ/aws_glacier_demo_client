
// use std::fs::File;
// use log::info;

// use rusoto_core::{Region, RusotoError};
// use rusoto_glacier::{GlacierClient, Glacier, InitiateMultipartUploadOutput, InitiateMultipartUploadError};


mod aws_client
{

    use log::info;
    use rusoto_glacier::Glacier;

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
        info!("Have {} parts, {} remainder", num_parts, num_remainder);

        // glacier_client.initiate_multipart_upload(multipart_upload);

        Err("junk!")
    }
}


