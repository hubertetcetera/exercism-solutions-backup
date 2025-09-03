use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut chars: Vec<&str> = input.graphemes(true).collect();
    let n = chars.len();
    let mut i = 0;
    while i < n / 2 {
        chars.swap(i, n.saturating_sub(i.saturating_add(1)));
        i += 1;
    }
    chars.concat()
}
