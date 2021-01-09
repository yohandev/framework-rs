/// trait for types that contain PCM data
pub trait Sample: Sized + cpal::Sample + audrey::read::Sample
{
    /// the silence value, as the name implies
    const SILENCE: Self;
    /// the type of this sample
    const TYPE: SampleType;

    /// convert this sample type to another
    fn to_f32(self) -> f32;
    /// convert this sample type to another
    fn to_i16(self) -> i16;
    /// convert this sample type to another
    fn to_u16(self) -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleType
{
    /// f32, 32-bit signed float in the [-1.0, 1.0] range
    F32,
    /// i16, 16-bit signed integer, where 0 is silence
    I16,
    /// u16, 16-bit unsigned integer, where 0x8000 is silence
    U16
}

impl Sample for f32
{
    const SILENCE: Self = 0.0;
    const TYPE: SampleType = SampleType::F32;

    #[inline]
    fn to_f32(self) -> f32 { self }
    #[inline]
    fn to_i16(self) -> i16
    {
        match self >= 0.0
        {
            true => (self * i16::MAX as f32) as i16,
            false => (-self * i16::MIN as f32) as i16,
        }
    }
    #[inline]
    fn to_u16(self) -> u16
    {
        (((self + 1.0) * 0.5) * u16::MAX as f32).round() as u16
    }
}

impl Sample for i16
{
    const SILENCE: Self = 0;
    const TYPE: SampleType = SampleType::I16;

    #[inline]
    fn to_f32(self) -> f32
    {
        match self < 0
        {
            true => self as f32 / -(i16::MIN as f32),
            false => self as f32 / i16::MAX as f32,
        }
    }
    #[inline]
    fn to_i16(self) -> i16 { self }
    #[inline]
    fn to_u16(self) -> u16 { self.wrapping_add(-0x8000) as u16 }
}

impl Sample for u16
{
    const SILENCE: Self = 0x8000;
    const TYPE: SampleType = SampleType::U16;

    #[inline]
    fn to_f32(self) -> f32 { self.to_i16().to_f32() }
    #[inline]
    fn to_i16(self) -> i16 { self.wrapping_add(0x8000) as i16 }

    #[inline]
    fn to_u16(self) -> u16 { self }
}

impl From<cpal::SampleFormat> for SampleType
{
    fn from(format: cpal::SampleFormat) -> Self
    {
        use cpal::SampleFormat::*;

        match format
        {
            I16 => Self::I16,
            U16 => Self::U16,
            F32 => Self::F32
        }
    }
}