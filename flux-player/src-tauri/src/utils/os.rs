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
