pub mod file_checksum {

    use log::error;
    use log::trace;

    use sha2::{Digest, Sha256};
    use std::io::Read;
    // use std::fmt::Formatter;
    use data_encoding::HEXUPPER;

    pub struct Sha256Checksum {
        checksum: [u8; 32],
    }

    /*fn get_sha256checksum_from_sha256<S: digest::FixedOutput + core::array::FixedSizeArray<u8>>(checksum: &S) -> Sha256Checksum
    {
        trace!("hash {} bytes", checksum.len());
        let exported: &[u8]= checksum.as_slice();

        let mut result_array: [u8; 32] = [0; 32];
        result_array.copy_from_slice(&exported[..32]);

        Sha256Checksum{
            checksum: result_array
        }
    }*/

    fn get_sha256checksum_from_sha256<A: digest::generic_array::ArrayLength<u8>>(
        checksum: &digest::generic_array::GenericArray<u8, A>,
    ) -> Sha256Checksum {
        trace!("hash {} bytes", checksum.len());
        let exported: &[u8] = checksum.as_slice();

        let mut result_array: [u8; 32] = [0; 32];
        result_array.copy_from_slice(&exported[..32]);

        Sha256Checksum {
            checksum: result_array,
        }
    }

    pub fn get_sha256_digest_partial<R: std::io::Read>(
        mut reader: R,
        num_bytes: u64,
    ) -> Result<Sha256Checksum, String> {
        let mut ctx = Sha256::new();
        let mut buffer = Vec::with_capacity(num_bytes as usize);

        match reader.read(&mut buffer) {
            Ok(_) => Ok(get_sha256checksum_from_sha256(&ctx.result())),
            Err(_) => Err(String::from("failed to get calculate partial")),
        }
    }

    pub fn get_sha256_digest_full<R: std::io::Read>(
        mut reader: R,
    ) -> Result<Sha256Checksum, String> {
        let mut ctx = Sha256::new();
        let mut buffer = [0; 1024];

        info!("Reading file...");

        loop {
            match reader.read(&mut buffer) {
                Ok(d) => {
                    if d == 0 {
                        trace!("Finished reading file");
                        break;
                    }
                    ctx.input(&buffer[..d]);
                }
                Err(e) => {
                    error!("Failed: {}", e);
                    return Err(e.to_string());
                }
            }
        }

        let checksum = ctx.result();

        return Ok(get_sha256checksum_from_sha256(&checksum));
    }

    impl std::fmt::Display for Sha256Checksum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", HEXUPPER.encode(&self.checksum))
        }
    }
}
