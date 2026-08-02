use crate::state::nv_state_t;
use crate::utils::nv_format_amount_cents;

use ledger_device_sdk::buttons::{ButtonEvent, ButtonsState};
use ledger_device_sdk::ui::bagls::Label;
use ledger_device_sdk::ui::bagls::{
    RectFull, LEFT_ARROW, LEFT_S_ARROW, RIGHT_ARROW, RIGHT_S_ARROW,
};
use ledger_device_sdk::ui::gadgets::{clear_screen, get_event};
use ledger_device_sdk::ui::layout::{Draw, Layout, Location, StringPlace};

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum nv_screen_t {
    NV_SCREEN_OVERVIEW = 0,
    NV_SCREEN_CAROUSEL = 1,
    NV_SCREEN_DETAIL = 2,
    NV_SCREEN_ABOUT = 3,
}

#[repr(C)]
pub struct nv_ui_t {
    pub current_screen: nv_screen_t,
    pub current_idx: usize,
}

impl nv_ui_t {
    pub const fn new() -> Self {
        Self {
            current_screen: nv_screen_t::NV_SCREEN_OVERVIEW,
            current_idx: 0,
        }
    }

    fn next(&mut self, state: &nv_state_t) {
        match self.current_screen {
            nv_screen_t::NV_SCREEN_OVERVIEW => {
                if state.entry_count > 0 {
                    self.current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
                    self.current_idx = 0;
                } else {
                    self.current_screen = nv_screen_t::NV_SCREEN_ABOUT;
                }
            }
            nv_screen_t::NV_SCREEN_CAROUSEL => {
                if self.current_idx + 1 < state.entry_count as usize {
                    self.current_idx += 1;
                } else {
                    self.current_screen = nv_screen_t::NV_SCREEN_ABOUT;
                }
            }
            nv_screen_t::NV_SCREEN_DETAIL => {
                self.current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
            }
            nv_screen_t::NV_SCREEN_ABOUT => {
                self.current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
            }
        }
    }

    fn prev(&mut self, state: &nv_state_t) {
        match self.current_screen {
            nv_screen_t::NV_SCREEN_OVERVIEW => {
                self.current_screen = nv_screen_t::NV_SCREEN_ABOUT;
            }
            nv_screen_t::NV_SCREEN_CAROUSEL => {
                if self.current_idx > 0 {
                    self.current_idx -= 1;
                } else {
                    self.current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
                }
            }
            nv_screen_t::NV_SCREEN_DETAIL => {
                self.current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
            }
            nv_screen_t::NV_SCREEN_ABOUT => {
                if state.entry_count > 0 {
                    self.current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
                    self.current_idx = state.entry_count as usize - 1;
                } else {
                    self.current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
                }
            }
        }
    }

    fn select(&mut self) {
        match self.current_screen {
            nv_screen_t::NV_SCREEN_CAROUSEL => {
                self.current_screen = nv_screen_t::NV_SCREEN_DETAIL;
            }
            nv_screen_t::NV_SCREEN_DETAIL => {
                self.current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
            }
            _ => {}
        }
    }
}

impl Default for nv_ui_t {
    fn default() -> Self {
        Self::new()
    }
}

pub fn nv_ui_run(ui: &mut nv_ui_t, state: &nv_state_t) -> ! {
    let mut buttons = ButtonsState::new();
    let mut amount_buffer = [0u8; 32];
    loop {
        match ui.current_screen {
            nv_screen_t::NV_SCREEN_OVERVIEW => {
                render_overview(state, &mut amount_buffer);
            }
            nv_screen_t::NV_SCREEN_CAROUSEL => {
                if let Some(entry) = state.entry_at(ui.current_idx) {
                    let label = entry.label();
                    let amount = format_amount(entry.amount_cents, &mut amount_buffer);
                    render_carousel(label, amount, ui.current_idx, state.entry_count as usize);
                } else {
                    ui.current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
                    continue;
                }
            }
            nv_screen_t::NV_SCREEN_DETAIL => {
                if let Some(entry) = state.entry_at(ui.current_idx) {
                    let label = entry.label();
                    let amount = format_amount(entry.amount_cents, &mut amount_buffer);
                    let category = entry.category.as_str();
                    render_detail(label, category, amount);
                } else {
                    ui.current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
                    continue;
                }
            }
            nv_screen_t::NV_SCREEN_ABOUT => render_about(),
        }

        loop {
            match get_event(&mut buttons) {
                Some(ButtonEvent::LeftButtonPress) => LEFT_S_ARROW.instant_display(),
                Some(ButtonEvent::RightButtonPress) => RIGHT_S_ARROW.instant_display(),
                Some(ButtonEvent::LeftButtonRelease) => {
                    LEFT_S_ARROW.erase();
                    ui.prev(state);
                    break;
                }
                Some(ButtonEvent::RightButtonRelease) => {
                    RIGHT_S_ARROW.erase();
                    ui.next(state);
                    break;
                }
                Some(ButtonEvent::BothButtonsPress) => {}
                Some(ButtonEvent::BothButtonsRelease) => {
                    ui.select();
                    break;
                }
                _ => {}
            }
        }
    }
}

fn format_amount(amount_cents: i64, buffer: &mut [u8; 32]) -> &str {
    let length = nv_format_amount_cents(amount_cents, buffer);
    core::str::from_utf8(&buffer[..length]).unwrap_or("?")
}

fn render_overview(state: &nv_state_t, amount_buffer: &mut [u8; 32]) {
    let amount = format_amount(state.total_net(), amount_buffer);
    let mut count_buffer = [0u8; 16];
    let count = format_entry_count(state.entry_count as usize, &mut count_buffer);
    render_labels([
        Label::from("NANO VAULT")
            .location(Location::Custom(3))
            .bold(),
        Label::from("TOTAL BALANCE").location(Location::Custom(20)),
        Label::from(amount).location(Location::Custom(32)).bold(),
        Label::from(count).location(Location::Custom(51)),
    ]);
    render_rules();
    RIGHT_ARROW.display();
    LEFT_ARROW.display();
    ledger_device_sdk::ui::screen_util::screen_update();
}

fn render_carousel(label: &str, amount: &str, index: usize, count: usize) {
    let mut position = [0u8; 8];
    let position_text = format_position(index + 1, count, &mut position);
    render_labels([
        Label::from("ENTRY").location(Location::Custom(3)).bold(),
        Label::from(label).location(Location::Custom(19)).bold(),
        Label::from(amount).location(Location::Custom(32)).bold(),
        Label::from(position_text).location(Location::Custom(51)),
    ]);
    render_rules();
    LEFT_ARROW.display();
    RIGHT_ARROW.display();
    ledger_device_sdk::ui::screen_util::screen_update();
}

fn render_detail(label: &str, category: &str, amount: &str) {
    render_labels([
        Label::from("ENTRY DETAILS")
            .location(Location::Custom(3))
            .bold(),
        Label::from(label).location(Location::Custom(19)).bold(),
        Label::from(category).location(Location::Custom(31)),
        Label::from(amount).location(Location::Custom(41)).bold(),
        Label::from("BOTH: BACK").location(Location::Custom(53)),
    ]);
    render_rules();
    LEFT_ARROW.display();
    RIGHT_ARROW.display();
    ledger_device_sdk::ui::screen_util::screen_update();
}

fn render_about() {
    render_labels([
        Label::from("NANO VAULT")
            .location(Location::Custom(3))
            .bold(),
        Label::from("PRIVATE LEDGER").location(Location::Custom(21)),
        Label::from("OFFLINE MODE").location(Location::Custom(33)),
        Label::from("v0.1.0").location(Location::Custom(51)),
    ]);
    render_rules();
    LEFT_ARROW.display();
    RIGHT_ARROW.display();
    ledger_device_sdk::ui::screen_util::screen_update();
}

fn render_labels<const N: usize>(labels: [Label<'_>; N]) {
    clear_screen();
    labels.place(Location::Middle, Layout::Centered, false);
}

fn render_rules() {
    RectFull::new().pos(0, 14).width(128).height(1).display();
    RectFull::new().pos(0, 49).width(128).height(1).display();
}

fn format_position(index: usize, count: usize, buffer: &mut [u8; 8]) -> &str {
    let mut pos = 0;
    pos += format_unsigned(index, &mut buffer[pos..]);
    buffer[pos] = b'/';
    pos += 1;
    pos += format_unsigned(count, &mut buffer[pos..]);
    core::str::from_utf8(&buffer[..pos]).unwrap_or("?")
}

fn format_entry_count(count: usize, buffer: &mut [u8; 16]) -> &str {
    let length = format_unsigned(count, buffer);
    let suffix: &[u8] = if count == 1 { b" ENTRY" } else { b" ENTRIES" };
    buffer[length..length + suffix.len()].copy_from_slice(suffix);
    core::str::from_utf8(&buffer[..length + suffix.len()]).unwrap_or("?")
}

fn format_unsigned(mut value: usize, buffer: &mut [u8]) -> usize {
    let mut digits = [0u8; 20];
    let mut length = 0;
    if value == 0 {
        buffer[0] = b'0';
        return 1;
    }
    while value != 0 {
        digits[length] = b'0' + (value % 10) as u8;
        length += 1;
        value /= 10;
    }
    for index in 0..length {
        buffer[index] = digits[length - index - 1];
    }
    length
}
