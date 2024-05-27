use wellen::{simple::Waveform, *};

pub fn read_from_bytes(bytes: Vec<u8>) -> Result<Waveform> {
    read_from_bytes_with_options(bytes, &LoadOptions::default())
}

pub fn read_from_bytes_with_options(bytes: Vec<u8>, options: &LoadOptions) -> Result<Waveform> {
    let header = viewers::read_header_from_bytes(bytes, options)?;
    let body = viewers::read_body(header.body, &header.hierarchy, None)?;
    Ok(Waveform::new(
        header.hierarchy,
        body.source,
        body.time_table,
    ))
}
