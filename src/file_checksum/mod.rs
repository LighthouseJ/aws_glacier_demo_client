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

    pub fn get_sha256_digest<R: std::io::Read>(mut reader: R) -> Result<Sha256Checksum, String> {
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

        let sha256_result = ctx.result();

        trace!("hash {} bytes", sha256_result.len());
        let exported = sha256_result.as_slice();

        let mut result_array: [u8; 32] = [0; 32];
        result_array.copy_from_slice(&exported[..32]);

        Ok(Sha256Checksum {
            checksum: result_array,
        })
    }

    impl std::fmt::Display for Sha256Checksum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", HEXUPPER.encode(&self.checksum))
        }
    }
}
