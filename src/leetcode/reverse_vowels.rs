const VOWELS: [u8; 10] = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];

fn reverse_vowels(s: String) -> String {
    let mut bytes = s.into_bytes();
    let (mut left, mut right) = (0, bytes.len());
    while left < right {
        while left < right && !VOWELS.contains(&bytes[left]) {
            left += 1;
        }
        while left < right && !VOWELS.contains(&bytes[right]) {
            right -= 1;
        }

        if left < right {
            bytes.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    String::from_utf8_lossy(&bytes).into()
}
