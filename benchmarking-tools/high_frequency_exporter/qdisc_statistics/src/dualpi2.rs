// https://github.com/L4STeam/linux/blob/8ac01b97ffd993b1fed93a015ec30dd085857e8b/include/uapi/linux/pkt_sched.h#L1290
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DualPi2XStats {
    pub prob: u32,
    pub delay_c: u32,
    pub delay_l: u32,
    pub packets_in_c: u32,
    pub packets_in_l: u32,
    pub maxq: u32,
    pub ecn_mark: u32,
    pub credit: i32,
    pub step_marks: u32,
}

impl DualPi2XStats {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < std::mem::size_of::<Self>() {
            return None;
        }

        // SAFETY: repr(C), all fields are plain integers, alignment is u32 (4 bytes). We check the length above.
        Some(unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const Self) })
    }
}
