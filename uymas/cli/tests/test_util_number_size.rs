use uymas_cli::util_number::size::{size_transform_1000, size_transform_1024};

#[test]
fn test_size_transform_1024() {
    let fmt_fn = |v: f64| format!("{:.3}", v);
    let (value, unit) = size_transform_1024(8749202348f64);
    assert_eq!(unit, "Gib");
    assert_eq!("8.148", fmt_fn(value));

    let (value, unit) = size_transform_1000(8749202348f64);
    assert_eq!(unit, "Gb");
    assert_eq!("8.749", fmt_fn(value));

    let (value, unit) = size_transform_1024(739944301f64);
    assert_eq!(unit, "Mib");
    assert_eq!("705.666", fmt_fn(value));

    let (value, unit) = size_transform_1024(534301f64);
    assert_eq!(unit, "Kib");
    assert_eq!("521.778", fmt_fn(value));
}
