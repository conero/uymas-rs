use std::collections::HashMap;

/// 处理文件大小
pub trait Size {
    /// 采用理论值，1024
    fn file_size(&self) -> (Self, String)
    where
        Self: Sized;
    /// 采用理论值，1000。实际产品的
    fn file_size_1000(&self) -> (Self, String)
    where
        Self: Sized;
    /// 将尺寸渲染为人为可读的
    fn human_size(&self) -> String;

    /// 将尺寸渲染为人为可读的-1000 为转化单位
    fn human_size_1000(&self) -> String;
}

/// 单位大小-`位` Bit
pub const BIT: u64 = 1;
/// 单位大小-`字节` Byte
pub const BYTE: u64 = 8;
/// 单位大小-`千字节` Kb
pub const KB: u64 = 1000;
/// 单位大小-`兆字节` Mb
pub const MB: u64 = 1000 * KB;
/// 单位大小-`吉字节` Gb
pub const GB: u64 = 1000 * MB;
/// 单位大小-`太字节` Tb
pub const TB: u64 = 1000 * GB;
/// 单位大小-`拍字节` Pb
pub const PB: u64 = 1000 * TB;

/// 基于 1024 的单位
/// 单位大小-`千字节` Kib
pub const KIB: u64 = 1024;
/// 单位大小-`兆字节` Mib
pub const MIB: u64 = 1024 * KIB;
/// 单位大小-`吉字节` Gib
pub const GIB: u64 = 1024 * MIB;
/// 单位大小-`太字节` Tib
pub const TIB: u64 = 1024 * GIB;
/// 单位大小-`拍字节` Pib
pub const PIB: u64 = 1024 * TIB;

/// 将u64转化为单位字符串，请传入单位常量
pub fn unit_string(u: u64) -> String {
    let v_map: HashMap<_, _> = vec![
        (BIT, "Bit"),
        (BYTE, "Byte"),
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

/// 文件大小转化为1000单位的文件大小
pub fn size_transform_1000(num: f64) -> (f64, String) {
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

/// 文件大小转化为1024单位的文件大小
pub fn size_transform_1024(num: f64) -> (f64, String) {
    let mut latest = num;
    let mut unit = 1;
    for v in vec![PIB, TIB, GIB, MIB, KIB, BYTE, BIT] {
        let v_f64 = v as f64;
        if num > v_f64 {
            latest = num / v_f64;
            unit = v;
            break;
        }
    }
    return (latest, unit_string(unit));
}

impl Size for f64 {
    fn file_size(&self) -> (Self, String) {
        size_transform_1024(*self)
    }

    fn file_size_1000(&self) -> (Self, String) {
        size_transform_1000(*self)
    }

    fn human_size(&self) -> String {
        let (v, unit) = size_transform_1024(*self);
        return format!("{:.3}{}", v, unit);
    }

    fn human_size_1000(&self) -> String {
        let (v, unit) = size_transform_1024(*self);
        return format!("{:.3}{}", v, unit);
    }
}
