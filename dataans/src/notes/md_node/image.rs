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
/// Resize the image to 20% of its original size (by width):
/// ![=20%x](path/to/image.png)
///
/// Resize the image to 50% of its original size (by height):
/// ![=x50%](path/to/image.png)
/// ```
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

    let mut dimensions = alt[1..].split('x');

    let width = dimensions
        .next()
        .and_then(|w| if w.is_empty() { None } else { Some(w) });
    if let Some(width) = width.as_ref() {
        if !validate_dimension(width) {
            warn!(?width, "Invalid image width:");

            return None;
        }
    }

    let height = dimensions
        .next()
        .and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(height) = height.as_ref() {
        if !validate_dimension(height) {
            warn!(?height, "Invalid image height:");

            return None;
        }
    }

    let mut style = String::new();
    if let Some(width) = width {
        if width.ends_with('%') {
            style.push_str(&format!("width: {width};"));
        } else {
            style.push_str(&format!("width: {width}px;"));
        }
    }
    if let Some(height) = height {
        if height.ends_with('%') {
            style.push_str(&format!("height: {height};"));
        } else {
            style.push_str(&format!("height: {height}px;"));
        }
    }

    Some(style)
}

/// Validates the image dimension string.
///
/// The dimension string can be either a number (e.g. "100") or a percentage (e.g. "50%").
/// All other formats are considered invalid.
fn validate_dimension(dimension: &str) -> bool {
    if dimension.ends_with('%') {
        return dimension[..dimension.len() - 1].parse::<u32>().is_ok();
    }

    dimension.parse::<u32>().is_ok()
}
