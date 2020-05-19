#[derive(Debug)]
struct SimpleByteRange
{
    start: usize,
    end: usize
}

pub struct AwsCredentials
{
    pub account_id: String
}

pub struct UploadInfo
{
    pub archive_description: String,
    // upload_size: u32,
    pub vault_name: String
}

pub mod aws_client
{

    use log::info;
    use std::convert::From;
    use rusoto_glacier::{Glacier, InitiateMultipartUploadInput, UploadMultipartPartInput, InitiateMultipartUploadOutput, InitiateMultipartUploadError, AbortMultipartUploadInput};
    use crate::aws_client::{ByteRange, AwsCredentials, UploadInfo};
    // use futures::executor::block_on;
    use tokio::runtime::Runtime;
    // use crate::file_checksum::file_checksum::get_sha256_digest_partial;

    fn calculate_file_parts(file: &std::fs::File, part_size: u64) -> Vec<SimpleByteRange>
    {
        info!("Using part_size={}", part_size);

        let input_file_metadata = file.metadata().unwrap();

        // figure out number of parts
        let num_parts = input_file_metadata.len() / part_size;
        let num_remainder = input_file_metadata.len() % part_size;
        info!("File size={}, have {} parts, {} remainder", input_file_metadata.len(),
              num_parts, num_remainder);


        let mut upload_parts: Vec<SimpleByteRange> = Vec::new();

        let mut next_start_pos: usize = 0;
        for x in 0 .. num_parts {
            let this_end_pos: usize = (part_size * (x + 1)) as usize;

            let new_byte_range = SimpleByteRange{
                start: next_start_pos,
                end: this_end_pos
            };

            upload_parts.push(new_byte_range);
            next_start_pos = this_end_pos + 1;
        }

        let final_byte_range = SimpleByteRange{
            start: next_start_pos,
            end: num_remainder as usize
        };
        upload_parts.push(final_byte_range);

        info!("parts to send are: {:?}", upload_parts);

        return upload_parts
    }

    pub fn send_file(aws_info: &AwsCredentials, upload_info: &UploadInfo, file: &std::fs::File, part_size: u64) -> Result<(), &'static str>
    {
        let this_upload_parts = calculate_file_parts(file, part_size);

        let future_result = upload_parts(aws_info,  upload_info, part_size, &this_upload_parts);

        Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(future_result)

    }

    async fn upload_parts(aws_info: &AwsCredentials, upload_info: &UploadInfo, part_size: u64, parts: &Vec<SimpleByteRange>) -> Result<(), &'static str>
    {
        let region = rusoto_core::Region::UsEast1;

        let glacier_client = rusoto_glacier::GlacierClient::new(region);

        let multipart_upload = rusoto_glacier::InitiateMultipartUploadInput
        {
            account_id: aws_info.account_id.clone(),
            archive_description: Some(upload_info.archive_description.clone()),
            part_size: Some(part_size.to_string()),
            vault_name: upload_info.vault_name.clone()
        };

        info!("initiating upload");

        let initiate_res =
            glacier_client.initiate_multipart_upload(multipart_upload).await.unwrap();

        info!("upload id={:?}", initiate_res.upload_id);


        /*let complete_upload = rusoto_glacier::CompleteMultipartUploadInput
        {
            account_id: aws_info.account_id.clone(),
            archive_size: None,
            checksum: None,
            upload_id: "".to_string(),
            vault_name: "".to_string()
        }

        let finish_upload =
            glacier_client.complete_multipart_upload(complete_upload).await; */

        unimplemented!("not finished!");
    }

    // async fn go(vault_name: &str, aws_info: &AwsCredentials, upload_info: &UploadInfo) -> Result<(), String>
    // {

    //     match initiate_res {
    //         Ok(r) => {
    //             info!("Ready to start, location={:?}, upload id={:?}", r.location, r.upload_id);



    //             glacier_client.abort_multipart_upload(AbortMultipartUploadInput{
    //                 vault_name: upload_info.vault_name.clone(),
    //                 account_id: aws_info.account_id.clone(),
    //                 upload_id: r.upload_id.unwrap()
    //             });
    //             Ok(())
    //         },
    //         Err(e) => {
    //             /*glacier_client.abort_multipart_upload(AbortMultipartUploadInput{
    //                 vault_name: upload_info.vault_name.clone(),
    //                 account_id: aws_info.account_id.clone(),
    //                 upload_id: e.
    //             })*/
    //             Err(e.to_string())
    //         },
    //     }

    //     // upload now

    // }
}


