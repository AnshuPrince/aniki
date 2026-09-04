/// Linear resample to 16 kHz mono PCM (Speechmatics input rate).
pub fn resample_to_16k(samples: &[i16], from_rate: u32) -> Vec<i16> {
    if from_rate == 16_000 || samples.is_empty() {
        return samples.to_vec();
    }
    let ratio = 16_000.0 / from_rate as f64;
    let out_len = ((samples.len() as f64) * ratio) as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let src_idx = (i as f64 / ratio) as usize;
        out.push(samples.get(src_idx).copied().unwrap_or(0));
    }
    out
}

pub fn rms_i16(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let energy: f64 = samples
        .iter()
        .map(|&s| {
            let n = s as f64 / i16::MAX as f64;
            n * n
        })
        .sum::<f64>()
        / samples.len() as f64;
    (energy.sqrt() as f32).min(1.0)
}

pub fn f32_bits(value: f32) -> u32 {
    value.to_bits()
}

pub fn f32_from_bits(bits: u32) -> f32 {
    f32::from_bits(bits)
}

pub fn cap_buffer(buf: &mut Vec<i16>, max_samples: usize) {
    if buf.len() > max_samples {
        let overflow = buf.len() - max_samples;
        buf.drain(0..overflow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_at_16k() {
        let samples = vec![1, 2, 3];
        assert_eq!(resample_to_16k(&samples, 16_000), samples);
    }
}
