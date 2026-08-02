use ledger_device_sdk::ecc::{bip32_derive, CurvesId};
use zeroize::Zeroize;

const HARDENED: u32 = 0x8000_0000;
const DERIVATION_PATH: [u32; 5] = [44 | HARDENED, 60 | HARDENED, HARDENED, 0, 0];

pub fn nv_crypto_derive_key(key_out: &mut [u8; 32]) -> bool {
    let mut derived = [0u8; 64];
    let success = bip32_derive(CurvesId::Secp256k1, &DERIVATION_PATH, &mut derived, None).is_ok();
    if success {
        key_out.copy_from_slice(&derived[..32]);
    }
    derived.zeroize();
    success
}

pub fn nv_crypto_encrypt(
    data: &mut [u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
    tag: &mut [u8; 16],
) -> bool {
    let mut context = new_gcm_context();
    let status = unsafe { encrypt_gcm(&mut context, data, key, nonce, tag) };
    let success = status == ledger_device_sdk::sys::CX_OK;
    wipe_context(&mut context);
    success
}

pub fn nv_crypto_decrypt(
    data: &mut [u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
    tag: &[u8; 16],
) -> bool {
    let mut context = new_gcm_context();
    let status = unsafe { decrypt_gcm(&mut context, data, key, nonce, tag) };
    let success = status == ledger_device_sdk::sys::CX_OK;
    wipe_context(&mut context);
    success
}

pub fn nv_crypto_wipe(key: &mut [u8; 32]) {
    key.zeroize();
}

fn new_gcm_context() -> ledger_device_sdk::sys::cx_aes_gcm_context_t {
    let mut context = ledger_device_sdk::sys::cx_aes_gcm_context_t::default();
    unsafe {
        ledger_device_sdk::sys::cx_aes_gcm_init(&mut context);
    }
    context
}

unsafe fn encrypt_gcm(
    context: &mut ledger_device_sdk::sys::cx_aes_gcm_context_t,
    data: &mut [u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
    tag: &mut [u8; 16],
) -> u32 {
    let status = ledger_device_sdk::sys::cx_aes_gcm_set_key(context, key.as_ptr(), key.len());
    if status != ledger_device_sdk::sys::CX_OK {
        return status;
    }
    ledger_device_sdk::sys::cx_aes_gcm_encrypt_and_tag(
        context,
        data.as_mut_ptr(),
        data.len(),
        nonce.as_ptr(),
        nonce.len(),
        core::ptr::null(),
        0,
        data.as_mut_ptr(),
        tag.as_mut_ptr(),
        tag.len(),
    )
}

unsafe fn decrypt_gcm(
    context: &mut ledger_device_sdk::sys::cx_aes_gcm_context_t,
    data: &mut [u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
    tag: &[u8; 16],
) -> u32 {
    let status = ledger_device_sdk::sys::cx_aes_gcm_set_key(context, key.as_ptr(), key.len());
    if status != ledger_device_sdk::sys::CX_OK {
        return status;
    }
    ledger_device_sdk::sys::cx_aes_gcm_decrypt_and_auth(
        context,
        data.as_mut_ptr(),
        data.len(),
        nonce.as_ptr(),
        nonce.len(),
        core::ptr::null(),
        0,
        data.as_mut_ptr(),
        tag.as_ptr(),
        tag.len(),
    )
}

fn wipe_context(context: &mut ledger_device_sdk::sys::cx_aes_gcm_context_t) {
    let bytes = unsafe {
        core::slice::from_raw_parts_mut(
            context as *mut _ as *mut u8,
            core::mem::size_of::<ledger_device_sdk::sys::cx_aes_gcm_context_t>(),
        )
    };
    bytes.zeroize();
}
