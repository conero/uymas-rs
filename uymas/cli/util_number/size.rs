use std::collections::HashMap;

/// 处理文件大小
pub trait Size {
    fn file_size(&self) -> (Self, String)
    where
        Self: Sized;
    fn human_size(&self) -> String;
}

/// 单位大小
pub const BIT: u64 = 1;
pub const BYTE: u64 = 8 * BIT;
pub const KB: u64 = 1000 * BYTE;
pub const MB: u64 = 1000 * KB;
pub const GB: u64 = 1000 * MB;
pub const TB: u64 = 1000 * GB;
pub const PB: u64 = 1000 * TB;

/// 基于 1024 的单位
pub const KIB: u64 = 1024 * BYTE;
pub const MIB: u64 = 1024 * KIB;
pub const GIB: u64 = 1024 * MIB;
pub const TIB: u64 = 1024 * GIB;
pub const PIB: u64 = 1024 * PIB;

fn unit_string(u: u64) -> String {
    let v_map: HashMap<_, _> = vec![
        (BIT, "bit"),
        (BYTE, "byte"),
        (KB, "Kb"),
        (KIB, "Kib"),
        (MB, "Mb"),
        (MIB, "Mib"),
        (GB, "Gb"),
        (GIB, "Gib"),
        (TB, "Tb"),
        (TIB, "Tib"),
        (PB, "Pb"),
        (PIB, "Pib"),
    ]
    .into_iter()
    .collect();
    if let Some(v) = v_map.get(&u) {
        String::from(*v)
    } else {
        String::new()
    }
}

fn size_transform_1000(num: f64) -> (f64, String) {
    let mut latest = num;
    let mut unit = 1;
    for v in vec![PB, TB, GB, MB, KB, BYTE, BIT] {
        let v_f64 = v as f64;
        if num > v_f64 {
            latest = num / v_f64;
            unit = v;
            break;
        }
    }

    return (latest, unit_string(unit));
}

fn size_transform_1024(num: f64) -> (f64, String) {
    let mut latest = num;
    let mut unit = 1;
    for v in vec![PIB, TIB, GIB, MIB, KIB, BYTE, BIT] {
        let v_f64 = v as f64;
        if num > v_f64 {
            latest = num / v_f64;
            break;
        }
    }
    return (latest, unit_string(unit));
}

impl Size for f64 {
    fn file_size(&self) -> (Self, String) {
        size_transform_1024(*self)
    }

    fn human_size(&self) -> String {
        let (v, unit) = size_transform_1024(*self);
        return format!("{}{}", v, unit);
    }
}
