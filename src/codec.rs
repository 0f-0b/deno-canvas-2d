use deno_core::op2;

use super::CanvasColorSpace;

#[op2]
#[buffer]
pub fn op_canvas_2d_encode_png(
    #[buffer] data: &[u8],
    #[number] width: u64,
    #[number] height: u64,
    color_space: i32,
) -> Result<Vec<u8>, png::EncodingError> {
    use png::EncodingError::LimitsExceeded;

    mod chunk {
        #![allow(non_upper_case_globals)]
        use png::chunk::*;

        pub const cICP: ChunkType = ChunkType(*b"cICP");
    }

    let color_space = CanvasColorSpace::from_repr(color_space).unwrap();
    let width = width.try_into().map_err(|_| LimitsExceeded)?;
    let height = height.try_into().map_err(|_| LimitsExceeded)?;
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    match color_space {
        CanvasColorSpace::Srgb => writer.write_chunk(chunk::cICP, &[1, 13, 0, 1])?,
        CanvasColorSpace::DisplayP3 => writer.write_chunk(chunk::cICP, &[12, 13, 0, 1])?,
    }
    writer.write_image_data(data)?;
    writer.finish()?;
    Ok(buf)
}
