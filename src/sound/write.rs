use crate::sound::Sample;

/// 
pub trait SampleWriter
{
    type Format: Sample;
}