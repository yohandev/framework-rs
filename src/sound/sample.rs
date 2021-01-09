/// trait for types that contain PCM data
pub trait Sample: Sized
{
    /// the silence value, as the name implies
    const SILENCE: Self;
    /// the type of this sample
    const TYPE: SampleType;
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

impl Sample for i16
{
    const SILENCE: Self = 0;
    const TYPE: SampleType = SampleType::I16;
}

impl Sample for u16
{
    const SILENCE: Self = 0x8000;
    const TYPE: SampleType = SampleType::U16;
}

impl Sample for f32
{
    const SILENCE: Self = 0.0;
    const TYPE: SampleType = SampleType::F32;
}