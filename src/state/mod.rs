#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum nv_category_t {
    NV_CAT_ASSET = 0,
    NV_CAT_DEBT = 1,
    NV_CAT_VAULT = 2,
    NV_CAT_EMERGENCY = 3,
}

pub unsafe fn nv_category_to_str(cat: nv_category_t) -> *const u8 {
    match cat {
        nv_category_t::NV_CAT_ASSET => b"asset\0".as_ptr(),
        nv_category_t::NV_CAT_DEBT => b"debt\0".as_ptr(),
        nv_category_t::NV_CAT_VAULT => b"vault stash\0".as_ptr(),
        nv_category_t::NV_CAT_EMERGENCY => b"emergency\0".as_ptr(),
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nv_entry_t {
    pub id: u8,
    pub category: nv_category_t,
    pub label: [u8; 16],
    pub label_len: usize,
    pub amount_cents: i64,
    pub is_active: bool,
}

pub unsafe fn nv_entry_init(entry: *mut nv_entry_t, id: u8, cat: nv_category_t, label_str: *const u8, label_len: usize, amount_cents: i64) {
    (*entry).id = id;
    (*entry).category = cat;
    (*entry).amount_cents = amount_cents;
    
    let len = if label_len > 16 { 16 } else { label_len };
    (*entry).label_len = len;
    
    let mut i = 0;
    while i < len {
        (*entry).label[i] = *label_str.add(i);
        i += 1;
    }
    (*entry).is_active = true;
}

pub unsafe fn nv_entry_wipe(entry: *mut nv_entry_t) {
    core::ptr::write_volatile(&mut (*entry).id, 0);
    core::ptr::write_volatile(&mut (*entry).amount_cents, 0);
    core::ptr::write_volatile(&mut (*entry).label_len, 0);
    core::ptr::write_volatile(&mut (*entry).is_active, false);
    
    let mut i = 0;
    while i < 16 {
        core::ptr::write_volatile((*entry).label.as_mut_ptr().add(i), 0);
        i += 1;
    }
}

#[repr(C)]
pub struct nv_state_t {
    pub entries: [nv_entry_t; 8],
    pub entry_count: usize,
    pub is_locked: bool,
}

pub unsafe fn nv_state_init(state: *mut nv_state_t) {
    (*state).entry_count = 0;
    (*state).is_locked = true;
    
    let mut i = 0;
    while i < 8 {
        (*state).entries[i].is_active = false;
        i += 1;
    }
}

pub unsafe fn nv_state_add_entry(state: *mut nv_state_t, id: u8, cat: nv_category_t, label_str: *const u8, label_len: usize, amount_cents: i64) -> bool {
    if (*state).entry_count >= 8 {
        return false;
    }
    let mut i = 0;
    while i < 8 {
        if !(*state).entries[i].is_active {
            nv_entry_init(&mut (*state).entries[i], id, cat, label_str, label_len, amount_cents);
            (*state).entry_count += 1;
            return true;
        }
        i += 1;
    }
    false
}

pub unsafe fn nv_state_remove_entry(state: *mut nv_state_t, id: u8) -> bool {
    let mut i = 0;
    while i < 8 {
        if (*state).entries[i].is_active && (*state).entries[i].id == id {
            nv_entry_wipe(&mut (*state).entries[i]);
            (*state).entry_count -= 1;
            return true;
        }
        i += 1;
    }
    false
}

pub unsafe fn nv_state_update_entry(state: *mut nv_state_t, id: u8, amount_cents: i64) -> bool {
    let mut i = 0;
    while i < 8 {
        if (*state).entries[i].is_active && (*state).entries[i].id == id {
            (*state).entries[i].amount_cents = amount_cents;
            return true;
        }
        i += 1;
    }
    false
}

pub unsafe fn nv_state_total_net(state: *const nv_state_t) -> i64 {
    let mut total: i64 = 0;
    let mut i = 0;
    while i < 8 {
        if (*state).entries[i].is_active {
            total += (*state).entries[i].amount_cents;
        }
        i += 1;
    }
    total
}
