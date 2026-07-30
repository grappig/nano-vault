#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use nano_vault::crypto::{nv_crypto_derive_key, nv_crypto_wipe};
use nano_vault::state::{nv_state_t, nv_state_init, nv_state_total_net, nv_state_add_entry, nv_category_t};
use nano_vault::ui::{nv_ui_t, nv_ui_init, nv_ui_next};

struct nv_allocator_t;
unsafe impl GlobalAlloc for nv_allocator_t {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { core::ptr::null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static NV_ALLOC: nv_allocator_t = nv_allocator_t;

#[used]
#[no_mangle]
pub static NV_APP_TITLE: &'static str = "nano vault";

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ledger_device_sdk::exiting_panic(info)
}

#[no_mangle]
pub extern "C" fn sample_main() {
    unsafe {
        let mut vault_state: core::mem::MaybeUninit<nv_state_t> = core::mem::MaybeUninit::uninit();
        let p_vault = vault_state.as_mut_ptr();
        
        let mut ui_state: core::mem::MaybeUninit<nv_ui_t> = core::mem::MaybeUninit::uninit();
        let p_ui = ui_state.as_mut_ptr();

        nv_state_init(p_vault);
        nv_ui_init(p_ui);

        // Add a sample entry to the vault
        let label = b"Emergency Fund";
        nv_state_add_entry(
            p_vault,
            1,
            nv_category_t::NV_CAT_EMERGENCY,
            label.as_ptr(),
            label.len(),
            500_00,
        );

        nv_ui_next(p_ui, p_vault);
        
        let _total = nv_state_total_net(p_vault);

        let mut key = [0u8; 32];
        let p_key = key.as_mut_ptr();
        nv_crypto_derive_key(p_key);
        
        nv_crypto_wipe(p_key);
        
        loop {}
    }
}
