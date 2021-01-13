use image::{ GenericImageView, ImageError };

use std::path::Path;

use crate::math::Extent2;
use crate::draw::Bitmap;

/// represents an image, which does everything a [Bitmap]
/// can
///
/// [Bitmap]: crate::Bitmap
pub type Image = Bitmap<Vec<u8>>;

impl Image
{
    /// open the image located at the path specified. the image is copied
    /// if the format isn't Rgba<u8>
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ImageError>
    {
        image::open(path).map(|img|
        {
            let size = Extent2::new(img.width(), img.height()).as_();

            Bitmap::new(img.into_rgba8().into_raw(), size)
        })
    }
}