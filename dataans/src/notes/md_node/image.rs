/// Parses the image alt text to extract an information about its size.
///
/// Originally, the MD does not allow to resize images.
/// However, we use a small workaround to allow images resizing.
/// The user can specify the image size in the alt text. The image size must met
/// the predefined format:
///
/// ```not_rust
/// =WIDTHxHEIGHT
/// ```
///
/// ## Example
///
/// ```md
/// Resize the image to 200x300 pixels:
/// ![=200x300](path/to/image.png)
///
/// Resize the image to 100 pixels in width and auto height:
/// ![=100x](path/to/image.png)
///
/// Resize the image to auto width and 150 pixels in height:
/// ![=x150](path/to/image.png)
///
/// Resize the image to 20% of its original size:
/// ![=20%](path/to/image.png)
/// ```
///
/// **Attention:** you cannot specify relative image size for width and height.
/// The following format is invalid:
/// ```md
/// ![=40%x60%](path/to/image.png)
/// ```
/// Why? Because it is hard to support such a behavior in CSS. Also, it is not a common use case.
/// It's not worth the trouble.
///
/// ## Return value
///
/// Returns a CSS string that can be injected into the `style` attribute of the `<img />` element.
///
/// ## Why ALT text?
///
/// Because it is not usually used for anything else in our app. We always expect the image to exist.
/// Even if the images does not exist, the alt text will be displayed instead of the image.
pub fn parse_image_size(alt: &str) -> Option<String> {
    if !alt.starts_with('=') {
        return None;
    }

    if !alt.contains('x') {
        // Relative resizing (e.g. "=20%").

        if !alt.ends_with('%') {
            warn!(?alt, "Invalid image size format: must end with '%':");

            return None;
        }

        if !is_valid_dimension(&alt[1..alt.len() - 1]) {
            warn!(?alt, "Invalid image size format: must be a number followed by '%':");

            return None;
        }

        return Some(format!("zoom: {};", alt[1..].trim()));
    }

    let mut dimensions = alt[1..].split('x');

    let width = dimensions
        .next()
        .and_then(|w| if w.is_empty() { None } else { Some(w) });
    if let Some(width) = width.as_ref() {
        if !is_valid_dimension(width) {
            warn!(?width, "Invalid image width:");

            return None;
        }
    }

    let height = dimensions
        .next()
        .and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(height) = height.as_ref() {
        if !is_valid_dimension(height) {
            warn!(?height, "Invalid image height:");

            return None;
        }
    }

    let mut style = String::new();
    if let Some(width) = width {
        style.push_str(&format!("width: {width}px;"));
    }
    if let Some(height) = height {
        style.push_str(&format!("height: {height}px;"));
    }

    Some(style)
}

/// Validates the image dimension string.
///
/// The dimension value must be a positive number.
fn is_valid_dimension(dimension: &str) -> bool {
    dimension.parse::<u32>().is_ok()
}
