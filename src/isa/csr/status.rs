use std::ops::Range;

pub const STATUS_UIE: Range<usize> = 0..0;
pub const STATUS_SIE: Range<usize> = 1..1;
pub const STATUS_MIE: Range<usize> = 3..3;
pub const STATUS_UPIE: Range<usize> = 4..4;
pub const STATUS_SPIE: Range<usize> = 5..5;
pub const STATUS_MPIE: Range<usize> = 7..7;
pub const STATUS_SPP: Range<usize> = 8..8;
pub const STATUS_MPP: Range<usize> = 11..12;
#[allow(dead_code)]
pub const STATUS_FS: Range<usize> = 13..14;
#[allow(dead_code)]
pub const STATUS_XS: Range<usize> = 15..16;
#[allow(dead_code)]
pub const STATUS_MPRV: Range<usize> = 17..17;
#[allow(dead_code)]
pub const STATUS_SUM: Range<usize> = 18..18;
#[allow(dead_code)]
pub const STATUS_MXR: Range<usize> = 19..19;
#[allow(dead_code)]
pub const STATUS_TVM: Range<usize> = 20..20;
#[allow(dead_code)]
pub const STATUS_TW: Range<usize> = 21..21;
#[allow(dead_code)]
pub const STATUS_TSR: Range<usize> = 22..22;
#[allow(dead_code)]
pub const STATUS_UXL: Range<usize> = 32..33;
#[allow(dead_code)]
pub const STATUS_SXL: Range<usize> = 34..35;
#[allow(dead_code)]
pub const STATUS_SD: Range<usize> = 63..63;
