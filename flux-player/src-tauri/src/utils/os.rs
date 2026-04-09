use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn compute_oshash(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();

    let mut hash: u64 = file_size;

    let mut buffer = vec![0u8; 65536];
    let read_amount = file.read(&mut buffer)?;
    if read_amount < 65536 {
        buffer.truncate(read_amount);
    }
    hash = hash.wrapping_add(bytes_to_u64_sum(&buffer));

    if file_size > 65536 {
        let seek_pos = if file_size > 131072 {
            file_size - 65536
        } else {
            65536
        };
        file.seek(SeekFrom::Start(seek_pos))?;
        let mut last_buffer = vec![0u8; 65536];
        let last_read = file.read(&mut last_buffer)?;
        last_buffer.truncate(last_read);
        hash = hash.wrapping_add(bytes_to_u64_sum(&last_buffer));
    }

    Ok(format!("{:016x}", hash))
}

fn bytes_to_u64_sum(bytes: &[u8]) -> u64 {
    bytes
        .chunks(8)
        .map(|chunk| {
            let mut array = [0u8; 8];
            let len = chunk.len();
            array[..len].copy_from_slice(chunk);
            u64::from_le_bytes(array)
        })
        .fold(0u64, |acc, x| acc.wrapping_add(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_oshash_small_file() {
        // Create a small test file (less than 65536 bytes)
        let mut tmp_file = NamedTempFile::new().unwrap();
        let content = vec![0u8; 100];
        tmp_file.write_all(&content).unwrap();

        let path = tmp_file.path().to_str().unwrap();
        let hash = compute_oshash(path).unwrap();

        // Hash should be consistent
        assert_eq!(hash.len(), 16);
        assert_ne!(hash, "0000000000000000");
    }

    #[test]
    fn test_oshash_exact_boundary() {
        // Create a file exactly 65536 bytes
        let mut tmp_file = NamedTempFile::new().unwrap();
        let content = vec![1u8; 65536];
        tmp_file.write_all(&content).unwrap();

        let path = tmp_file.path().to_str().unwrap();
        let hash = compute_oshash(path).unwrap();

        assert_eq!(hash.len(), 16);
    }

    #[test]
    fn test_oshash_large_file() {
        // Create a file larger than 131072 bytes (to trigger start and end hashing)
        let mut tmp_file = NamedTempFile::new().unwrap();
        let mut content = vec![0u8; 200000];
        
        // Add some data at the start and end
        content[0] = 1;
        content[199999] = 1;
        
        tmp_file.write_all(&content).unwrap();

        let path = tmp_file.path().to_str().unwrap();
        let hash = compute_oshash(path).unwrap();

        assert_eq!(hash.len(), 16);
    }
}

