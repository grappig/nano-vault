#![no_std]
#![no_main]

use nano_vault::state::{nv_category_t, nv_state_t};
use nano_vault::ui::{nv_ui_run, nv_ui_t};

#[used]
#[no_mangle]
pub static NV_APP_TITLE: &str = "nano vault";

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ledger_device_sdk::exiting_panic(info)
}

#[no_mangle]
pub extern "C" fn sample_main() {
    let mut vault_state = nv_state_t::new();
    vault_state.add_entry(
        1,
        nv_category_t::NV_CAT_EMERGENCY,
        b"Emergency Fund",
        50_000,
    );

    let mut ui_state = nv_ui_t::new();
    nv_ui_run(&mut ui_state, &vault_state);
}
