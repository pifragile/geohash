extern crate geohash;
extern crate alloc;
use fixed::types::I64F64;
use geohash::{decode, encode, neighbors};

use alloc::string::String;

#[test]
fn test_encode() {
    let lon = I64F64::from_num(112.5584);
    let lat = I64F64::from_num(37.8324f64);
    assert_eq!(encode(lat, lon, 9usize).unwrap(), String::from("ww8p1r4t8"));

    let lon = I64F64::from_num(117);
    let lat = I64F64::from_num(32);
    assert_eq!(encode(lat, lon, 3usize).unwrap(), String::from("wte"));

    let lon = I64F64::from_num(190);
    let lat = I64F64::from_num(-100);
    assert!(encode(lat, lon, 3usize).is_err());
}

fn compare_within(a: I64F64, b: I64F64, diff: I64F64) {
    assert!(
        (a - b).abs() < diff,
        format!("{:?} and {:?} should be within {:?}", a, b, diff)
    );
}

fn compare_decode(gh: String, exp_lon: I64F64, exp_lat: I64F64, exp_lon_err: I64F64, exp_lat_err: I64F64) {
    let (lon, lat, lon_err, lat_err) = decode(&gh).unwrap();
    let diff = I64F64::from_num(1e-5);
    compare_within(lon_err, exp_lon_err, diff);
    compare_within(lat_err, exp_lat_err, diff);
    compare_within(lon, exp_lon, diff);
    compare_within(lat, exp_lat, diff);
}

#[test]
fn test_decode() {
    compare_decode(String::from("ww8p1r4t8"), I64F64::from_num(112.558386), I64F64::from_num(37.832386), I64F64::from_num(0.000021457), I64F64::from_num(0.000021457));
    compare_decode(String::from("9g3q"), I64F64::from_num(-99.31640625), I64F64::from_num(19.423828125), I64F64::from_num(0.175781250), I64F64::from_num(0.087890625));

    assert!(decode(&String::from("abcd")).is_err());
}

#[test]
fn test_neighbor() {
    let ns = neighbors(&String::from("ww8p1r4t8")).unwrap();
    assert_eq!(ns.sw, "ww8p1r4mr");
    assert_eq!(ns.s, "ww8p1r4t2");
    assert_eq!(ns.se, "ww8p1r4t3");
    assert_eq!(ns.w, "ww8p1r4mx");
    assert_eq!(ns.e, "ww8p1r4t9");
    assert_eq!(ns.nw, "ww8p1r4mz");
    assert_eq!(ns.n, "ww8p1r4tb");
    assert_eq!(ns.ne, "ww8p1r4tc");
}

#[test]
fn test_neighbor_wide() {
    let ns = neighbors(&String::from("9g3m")).unwrap();
    assert_eq!(ns.sw, "9g3h");
    assert_eq!(ns.s, "9g3k");
    assert_eq!(ns.se, "9g3s");
    assert_eq!(ns.w, "9g3j");
    assert_eq!(ns.e, "9g3t");
    assert_eq!(ns.nw, "9g3n");
    assert_eq!(ns.n, "9g3q");
    assert_eq!(ns.ne, "9g3w");
}
