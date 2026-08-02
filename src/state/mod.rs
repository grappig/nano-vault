use zeroize::Zeroize;

pub const MAX_ENTRIES: usize = 8;
pub const MAX_LABEL_LEN: usize = 16;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum nv_category_t {
    NV_CAT_ASSET = 0,
    NV_CAT_DEBT = 1,
    NV_CAT_VAULT = 2,
    NV_CAT_EMERGENCY = 3,
}

impl nv_category_t {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NV_CAT_ASSET => "asset",
            Self::NV_CAT_DEBT => "debt",
            Self::NV_CAT_VAULT => "vault stash",
            Self::NV_CAT_EMERGENCY => "emergency",
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nv_entry_t {
    pub id: u8,
    pub category: nv_category_t,
    pub label: [u8; MAX_LABEL_LEN],
    pub label_len: u8,
    pub amount_cents: i64,
    pub is_active: bool,
}

impl nv_entry_t {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            category: nv_category_t::NV_CAT_ASSET,
            label: [0; MAX_LABEL_LEN],
            label_len: 0,
            amount_cents: 0,
            is_active: false,
        }
    }

    pub fn new(id: u8, category: nv_category_t, label: &[u8], amount_cents: i64) -> Self {
        let mut entry = Self::empty();
        entry.id = id;
        entry.category = category;
        entry.amount_cents = amount_cents;
        entry.label_len = label.len().min(MAX_LABEL_LEN) as u8;
        let length = entry.label_len as usize;
        entry.label[..length].copy_from_slice(&label[..length]);
        entry.is_active = true;
        entry
    }

    pub fn label(&self) -> &str {
        core::str::from_utf8(&self.label[..self.label_len as usize]).unwrap_or("Invalid label")
    }

    pub fn wipe(&mut self) {
        self.label.zeroize();
        self.id = 0;
        self.label_len = 0;
        self.amount_cents = 0;
        self.is_active = false;
    }
}

#[repr(C)]
pub struct nv_state_t {
    pub entries: [nv_entry_t; MAX_ENTRIES],
    pub entry_count: u8,
    pub is_locked: bool,
}

impl nv_state_t {
    pub const fn new() -> Self {
        Self {
            entries: [nv_entry_t::empty(); MAX_ENTRIES],
            entry_count: 0,
            is_locked: true,
        }
    }

    pub fn add_entry(
        &mut self,
        id: u8,
        category: nv_category_t,
        label: &[u8],
        amount_cents: i64,
    ) -> bool {
        if self.entry_count as usize >= MAX_ENTRIES {
            return false;
        }

        for entry in &mut self.entries {
            if !entry.is_active {
                *entry = nv_entry_t::new(id, category, label, amount_cents);
                self.entry_count += 1;
                return true;
            }
        }
        false
    }

    pub fn remove_entry(&mut self, id: u8) -> bool {
        for entry in &mut self.entries {
            if entry.is_active && entry.id == id {
                entry.wipe();
                self.entry_count = self.entry_count.saturating_sub(1);
                return true;
            }
        }
        false
    }

    pub fn update_entry(&mut self, id: u8, amount_cents: i64) -> bool {
        for entry in &mut self.entries {
            if entry.is_active && entry.id == id {
                entry.amount_cents = amount_cents;
                return true;
            }
        }
        false
    }

    pub fn entry_at(&self, index: usize) -> Option<&nv_entry_t> {
        let mut active_index = 0;
        for entry in &self.entries {
            if entry.is_active {
                if active_index == index {
                    return Some(entry);
                }
                active_index += 1;
            }
        }
        None
    }

    pub fn total_net(&self) -> i64 {
        self.entries
            .iter()
            .filter(|entry| entry.is_active)
            .fold(0, |total, entry| total.saturating_add(entry.amount_cents))
    }
}

impl Default for nv_state_t {
    fn default() -> Self {
        Self::new()
    }
}
