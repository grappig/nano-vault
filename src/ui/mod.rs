use crate::state::nv_state_t;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
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

pub unsafe fn nv_ui_init(ui: *mut nv_ui_t) {
    (*ui).current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
    (*ui).current_idx = 0;
}

pub unsafe fn nv_ui_next(ui: *mut nv_ui_t, state: *const nv_state_t) {
    match (*ui).current_screen {
        nv_screen_t::NV_SCREEN_OVERVIEW => {
            if (*state).entry_count > 0 {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
                (*ui).current_idx = 0;
            } else {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_ABOUT;
            }
        }
        nv_screen_t::NV_SCREEN_CAROUSEL => {
            if (*ui).current_idx + 1 < (*state).entry_count {
                (*ui).current_idx += 1;
            } else {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_ABOUT;
            }
        }
        nv_screen_t::NV_SCREEN_DETAIL => {
            (*ui).current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
        }
        nv_screen_t::NV_SCREEN_ABOUT => {
            (*ui).current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
        }
    }
}

pub unsafe fn nv_ui_prev(ui: *mut nv_ui_t, state: *const nv_state_t) {
    match (*ui).current_screen {
        nv_screen_t::NV_SCREEN_OVERVIEW => {
            (*ui).current_screen = nv_screen_t::NV_SCREEN_ABOUT;
        }
        nv_screen_t::NV_SCREEN_CAROUSEL => {
            if (*ui).current_idx > 0 {
                (*ui).current_idx -= 1;
            } else {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
            }
        }
        nv_screen_t::NV_SCREEN_DETAIL => {
            (*ui).current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
        }
        nv_screen_t::NV_SCREEN_ABOUT => {
            if (*state).entry_count > 0 {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_CAROUSEL;
                (*ui).current_idx = (*state).entry_count - 1;
            } else {
                (*ui).current_screen = nv_screen_t::NV_SCREEN_OVERVIEW;
            }
        }
    }
}

pub unsafe fn nv_ui_select(ui: *mut nv_ui_t) {
    if (*ui).current_screen == nv_screen_t::NV_SCREEN_CAROUSEL {
        (*ui).current_screen = nv_screen_t::NV_SCREEN_DETAIL;
    }
}
