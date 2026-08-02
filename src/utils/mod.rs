pub fn nv_format_amount_cents(amount_cents: i64, out: &mut [u8]) -> usize {
    let negative = amount_cents < 0;
    let magnitude = if negative {
        amount_cents.wrapping_neg() as u64
    } else {
        amount_cents as u64
    };
    let dollars = magnitude / 100;
    let cents = (magnitude % 100) as u8;

    let prefix_len = if negative { 2 } else { 1 };
    let mut digits = [0u8; 20];
    let digit_count = nv_format_unsigned(dollars, &mut digits);
    let required = prefix_len + digit_count + 3;
    if required > out.len() {
        return 0;
    }

    let mut pos = 0;
    if negative {
        out[pos] = b'-';
        pos += 1;
    }
    out[pos] = b'$';
    pos += 1;
    out[pos..pos + digit_count].copy_from_slice(&digits[..digit_count]);
    pos += digit_count;
    out[pos] = b'.';
    out[pos + 1] = b'0' + cents / 10;
    out[pos + 2] = b'0' + cents % 10;
    required
}

fn nv_format_unsigned(mut value: u64, out: &mut [u8; 20]) -> usize {
    if value == 0 {
        out[0] = b'0';
        return 1;
    }

    let mut len = 0;
    while value != 0 {
        out[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
    }

    let mut left = 0;
    let mut right = len - 1;
    while left < right {
        out.swap(left, right);
        left += 1;
        right -= 1;
    }
    len
}
