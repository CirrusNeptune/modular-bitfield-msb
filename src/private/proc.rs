use crate::{
    private::{
        PopBits,
        PopBuffer,
        PushBits,
        PushBuffer,
    },
    Specifier,
};

/// Creates a new push buffer with all bits initialized to 0.
#[inline]
fn push_buffer<T>() -> PushBuffer<<T as Specifier>::Bytes>
where
    T: Specifier,
    PushBuffer<T::Bytes>: Default,
{
    <PushBuffer<<T as Specifier>::Bytes> as Default>::default()
}

#[doc(hidden)]
#[inline]
pub fn read_specifier<T>(bytes: &[u8], offset: usize) -> <T as Specifier>::Bytes
where
    T: Specifier,
    PushBuffer<T::Bytes>: Default + PushBits,
{
    let end = offset + <T as Specifier>::BITS;
    let ms_byte = offset / 8; // compile-time
    let ls_byte = (end - 1) / 8; // compile-time
    let msb_offset = offset % 8; // compile-time
    let lsb_offset = end % 8; // compile-time
    let lsb_offset = if lsb_offset == 0 { 8 } else { lsb_offset };

    let mut buffer = push_buffer::<T>();

    if msb_offset == 0 && lsb_offset == 8 {
        // Edge-case for whole bytes manipulation.
        for byte in bytes[ms_byte..(ls_byte + 1)].iter() {
            buffer.push_bits(8, *byte)
        }
    } else {
        if ms_byte == ls_byte {
            buffer.push_bits(<T as Specifier>::BITS as u32, (bytes[ms_byte] >> (8 - lsb_offset)) & ((1 << <T as Specifier>::BITS) - 1) as u8);
        } else {
            buffer.push_bits(8 - msb_offset as u32, bytes[ms_byte] & ((1 << (8 - msb_offset)) - 1) as u8);
        }
        if ls_byte - ms_byte >= 2 {
            // Middle bytes
            for byte in bytes[(ms_byte + 1)..ls_byte].iter() {
                buffer.push_bits(8, *byte);
            }
        }
        if ms_byte != ls_byte {
            // Least-significant byte
            buffer.push_bits(lsb_offset as u32, bytes[ls_byte] >> (8 - lsb_offset));
        }
    }
    buffer.into_bytes()
}

#[doc(hidden)]
#[inline]
pub fn write_specifier<T>(
    bytes: &mut [u8],
    offset: usize,
    new_val: <T as Specifier>::Bytes,
) where
    T: Specifier,
    PopBuffer<T::Bytes>: PopBits,
{
    let end = offset + <T as Specifier>::BITS;
    let ms_byte = offset / 8; // compile-time
    let ls_byte = (end - 1) / 8; // compile-time
    let msb_offset = offset % 8; // compile-time
    let lsb_offset = end % 8; // compile-time
    let lsb_offset = if lsb_offset == 0 { 8 } else { lsb_offset };

    let mut buffer = <PopBuffer<T::Bytes>>::from_bytes(new_val);

    if msb_offset == 0 && lsb_offset == 8 {
        // Edge-case for whole bytes manipulation.
        for byte in bytes[ms_byte..(ls_byte + 1)].iter_mut().rev() {
            *byte = buffer.pop_bits(8);
        }
    } else {
        if ms_byte != ls_byte {
            // Least-significant byte
            if lsb_offset == 8 {
                // We don't need to respect what was formerly stored in the byte.
                bytes[ls_byte] = buffer.pop_bits(lsb_offset as u32);
            } else {
                // All bits that do not belong to this field should be preserved.
                let stays_same = bytes[ls_byte] & ((1 << (8 - lsb_offset)) - 1) as u8;
                let overwrite = buffer.pop_bits(lsb_offset as u32);
                bytes[ls_byte] = stays_same | (overwrite << (8 - lsb_offset));
            }
        }
        if ls_byte - ms_byte >= 2 {
            // Middle bytes
            for byte in bytes[(ms_byte + 1)..ls_byte].iter_mut().rev() {
                *byte = buffer.pop_bits(8);
            }
        }
        // Most-significant byte
        if ms_byte == ls_byte {
            let upper_mask = !((1 << (8 - msb_offset)) - 1) as u8;
            let lower_mask = ((1 << (8 - lsb_offset)) - 1) as u8;
            let stays_same = bytes[ms_byte] & (upper_mask | lower_mask);
            let overwrite = buffer.pop_bits(<T as Specifier>::BITS as u32) << (8 - lsb_offset);
            bytes[ms_byte] = stays_same | overwrite;
        } else {
            let stays_same = bytes[ms_byte] & !((1 << (8 - msb_offset)) - 1) as u8;
            let overwrite = buffer.pop_bits(8 - msb_offset as u32);
            bytes[ms_byte] = stays_same | overwrite;
        }
    }
}
