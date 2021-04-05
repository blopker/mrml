use super::MJCarouselImage;
use crate::prelude::print::{print_open, Print};
use std::fmt;

impl Print for MJCarouselImage {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(
            f,
            super::NAME,
            Some(&self.attributes),
            true,
            pretty,
            level,
            indent_size,
        );
    }
}

impl fmt::Display for MJCarouselImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_carousel_image::MJCarouselImage::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-carousel-image src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}