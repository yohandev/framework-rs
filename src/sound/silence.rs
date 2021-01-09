use crate::sound::{ SampleType, Sample };

/// writes the silence value everywhere in the
/// provided `stream`
pub fn write_silence(stream: &mut cpal::Data, ty: SampleType)
{
    fn silence<T: Sample>(stream: &mut cpal::Data)
    {
        stream
            .as_slice_mut::<T>()
            .unwrap()
            .iter_mut()
            .for_each(|i| *i = T::SILENCE);
    }

    match ty
    {
        SampleType::F32 => silence::<f32>(stream),
        SampleType::I16 => silence::<i16>(stream),
        SampleType::U16 => silence::<u16>(stream),
    }
}