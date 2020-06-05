#[derive(Debug)]
struct SimpleByteRange {
    start: usize,
    end: usize,
}

pub struct AwsCredentials {
    pub account_id: String,
}

pub struct UploadInfo {
    pub archive_description: String,
    pub vault_name: String,
}

pub mod aws_client {

    use std::cmp;
    use crate::aws_client::{AwsCredentials, UploadInfo, SimpleByteRange};
    use log::info;
    use rusoto_glacier::{AbortMultipartUploadInput, Glacier, UploadMultipartPartInput};
    use tokio::runtime::Runtime;
    use std::time::Duration;
    use futures::io::Error;
    use futures::stream::{self, StreamExt};
    use std::fs::OpenOptions;

    fn calculate_file_parts(file: &std::fs::File, part_size: u64) -> Vec<SimpleByteRange> {
        info!("Using part_size={}", part_size);

        let input_file_metadata = file.metadata().unwrap();

        // figure out number of parts
        let num_parts = input_file_metadata.len() / part_size;
        let num_remainder = input_file_metadata.len() % part_size;
        info!(
            "File size={}, have {} parts, {} remainder",
            input_file_metadata.len(),
            num_parts,
            num_remainder
        );

        let mut upload_parts: Vec<SimpleByteRange> = Vec::new();

        let mut next_start_pos: usize = 0;
        for x in 0..num_parts {
            let this_end_pos: usize = (part_size * (x + 1)) as usize;

            let new_byte_range = SimpleByteRange {
                start: next_start_pos,
                end: this_end_pos,
            };

            upload_parts.push(new_byte_range);
            next_start_pos = this_end_pos + 1;
        }

        let final_byte_range = SimpleByteRange {
            start: next_start_pos,
            end: num_remainder as usize,
        };
        upload_parts.push(final_byte_range);

        info!("parts to send are: {:?}", upload_parts);

        return upload_parts;
    }

    pub fn send_file(
        aws_info: &AwsCredentials,
        upload_info: &UploadInfo,
        file: &std::fs::File,
        part_size: u64,
    ) -> Result<(), &'static str> {
        let this_upload_ranges = calculate_file_parts(file, part_size);
        let thread_count = cmp::max(4, this_upload_ranges.len());
        info!("thread count={}", thread_count);

        let future_result = upload_parts(aws_info, upload_info, part_size, &this_upload_ranges);

        /*let threaded_rt = tokio::runtime::Builder::new()
            // .threaded_scheduler()
            .core_threads(thread_count)
            .on_thread_start(|| {
                println!("thread starting");
            })
            .on_thread_stop(|| {
                println!("thread stopping");
            })
            .build();*/
        let threaded_rt = Runtime::new();

        match threaded_rt
        {
            Ok(mut runtime) => runtime.block_on(future_result),

            Err(_) => Err("failed to allocate runtime")
        }

    }

    async fn make_upload_segment(file: &std::fs::File, r: &SimpleByteRange) -> Result<UploadMultipartPartInput, Error>
    {
        let format_string = String::new();

        Ok(UploadMultipartPartInput {
            // format:   bytes 0-4194303/*
            account_id: "".to_string(),
            body: None,
            checksum: None,
            range: Some(format_string),
            upload_id: "".to_string(),
            vault_name: "".to_string()
        })
    }

    async fn send_segment(range: &SimpleByteRange) -> Result<(), Error>
    {
        info!("sending segment: {:?}", range);

        Ok(())

    }

    async fn upload_parts(
        aws_info: &AwsCredentials,
        upload_info: &UploadInfo,
        part_size: u64,
        parts: &Vec<SimpleByteRange>,
    ) -> Result<(), &'static str> {
        let region = rusoto_core::Region::UsEast1;

        let glacier_client = rusoto_glacier::GlacierClient::new(region);

        let multipart_upload = rusoto_glacier::InitiateMultipartUploadInput {
            account_id: aws_info.account_id.clone(),
            archive_description: Some(upload_info.archive_description.clone()),
            part_size: Some(part_size.to_string()),
            vault_name: upload_info.vault_name.clone(),
        };

        info!("initiating upload");

        let initiate_res = glacier_client
            .initiate_multipart_upload(multipart_upload)
            .await
            .unwrap();

        info!("upload id={:?}", initiate_res.upload_id);

        // parts.iter().for_each(|r| tokio::spawn( async move { send_segment(r)}));
        let fut = stream::iter(for x in parts {
            calculate_file_parts(fi)
        })
            .for_each_concurrent(
                2,
                |rx| async move {
                    // println!("process={:?}", rx);
                    send_segment(rx).await.unwrap();
                }
            );

        // abort or complete now
        match glacier_client.abort_multipart_upload(AbortMultipartUploadInput{
            account_id: aws_info.account_id.clone(),
            upload_id: initiate_res.upload_id.unwrap(),
            vault_name: upload_info.vault_name.clone()
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to abort!"),
        }
    }


}
