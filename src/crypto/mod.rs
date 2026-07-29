pub unsafe fn nv_crypto_derive_key(key_out: *mut u8) {
    let seed = b"nanovaultmasterhardwareseedkey32";
    let mut i = 0;
    while i < 32 {
        *key_out.add(i) = seed[i];
        i += 1;
    }
}

pub unsafe fn nv_crypto_encrypt(data: *mut u8, len: usize, key: *const u8) {
    let mut i = 0;
    while i < len {
        *data.add(i) ^= *key.add(i % 32);
        i += 1;
    }
}

pub unsafe fn nv_crypto_decrypt(data: *mut u8, len: usize, key: *const u8) {
    nv_crypto_encrypt(data, len, key);
}

pub unsafe fn nv_crypto_wipe(key: *mut u8) {
    let mut i = 0;
    while i < 32 {
        core::ptr::write_volatile(key.add(i), 0);
        i += 1;
    }
}
