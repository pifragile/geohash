use crate::neighbors::Direction;
use crate::{GeohashError, Neighbors};
use fixed::types::I64F64;


#[derive(Debug)]
struct Coordinate
{
    pub lon: I64F64,
    pub lat: I64F64,
}

struct Rectangle
{
    min: Coordinate,
    max: Coordinate,
}

static BASE32_CODES: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k',
    'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Encode a coordinate to a geohash with length `len`.
///
/// ### Examples
///
/// Encoding a coordinate to a length five geohash:
///
/// ```rust
/// use fixed::types::I64F64;
/// let lon = I64F64::from_num(-120.6623);
/// let lat = I64F64::from_num(35.3003);
/// let geohash_string = geohash::encode(lon, lat, 5).expect("Invalid coordinate");
/// assert_eq!(geohash_string, "9q60y");
/// ```
///
/// Encoding a coordinate to a length ten geohash:
///
/// ```rust
/// use fixed::types::I64F64;
/// let lon = I64F64::from_num(-120.6623);
/// let lat = I64F64::from_num(35.3003);
/// let geohash_string = geohash::encode(lon, lat, 10).expect("Invalid coordinate");
///
/// assert_eq!(geohash_string, "9q60y60rhs");
/// ```
pub fn encode(lon: I64F64, lat: I64F64, len: usize) -> Result<String, GeohashError> {
    let mut out = String::with_capacity(len);

    let mut bits_total: i8 = 0;
    let mut hash_value: usize = 0;
    let mut max_lat = I64F64::from_num(90);
    let mut min_lat = I64F64::from_num(-90);
    let mut max_lon = I64F64::from_num(180);
    let mut min_lon = I64F64::from_num(-180);

    if lon < min_lon || lon > max_lon || lat < min_lat || lat > max_lat {
        return Err(GeohashError::InvalidCoordinateRange(lon, lat));
    }

    let two = I64F64::from_num(2);
    while out.len() < len {
        for _ in 0..5 {
            if bits_total % 2 == 0 {
                let mid = (max_lon + min_lon) / two;
                if lon > mid {
                    hash_value = (hash_value << 1) + 1usize;
                    min_lon = mid;
                } else {
                    hash_value <<= 1;
                    max_lon = mid;
                }
            } else {
                let mid = (max_lat + min_lat) / two;
                if lat > mid {
                    hash_value = (hash_value << 1) + 1usize;
                    min_lat = mid;
                } else {
                    hash_value <<= 1;
                    max_lat = mid;
                }
            }
            bits_total += 1;
        }

        let code: char = BASE32_CODES[hash_value];
        out.push(code);
        hash_value = 0;
    }
    Ok(out)
}

/// Decode geohash string into latitude, longitude
///
/// Parameters:
/// Geohash encoded `&str`
///
/// Returns:
/// A four-element tuple describs a bound box:
/// * min_lat
/// * max_lat
/// * min_lon
/// * max_lon
fn decode_bbox(hash_str: &str) -> Result<Rectangle, GeohashError> {
    let mut is_lon = true;
    let mut max_lat = I64F64::from_num(90);
    let mut min_lat = I64F64::from_num(-90);
    let mut max_lon = I64F64::from_num(180);
    let mut min_lon = I64F64::from_num(-180);
    let mut mid: I64F64;
    let mut hash_value: usize;

    let two = I64F64::from_num(2);

    for c in hash_str.chars() {
        hash_value = hash_value_of_char(c)?;

        for bs in 0..5 {
            let bit = (hash_value >> (4 - bs)) & 1usize;
            if is_lon {
                mid = (max_lon + min_lon) / two;

                if bit == 1 {
                    min_lon = mid;
                } else {
                    max_lon = mid;
                }
            } else {
                mid = (max_lat + min_lat) / two;

                if bit == 1 {
                    min_lat = mid;
                } else {
                    max_lat = mid;
                }
            }
            is_lon = !is_lon;
        }
    }

    Ok(Rectangle {
        min: Coordinate {
            lon: min_lon,
            lat: min_lat,
        },
        max: Coordinate {
            lon: max_lon,
            lat: max_lat,
        },
    })
}

fn hash_value_of_char(c: char) -> Result<usize, GeohashError> {
    let ord = c as usize;
    if (48..=57).contains(&ord) {
        return Ok(ord - 48);
    } else if (98..=104).contains(&ord) {
        return Ok(ord - 88);
    } else if (106..=107).contains(&ord) {
        return Ok(ord - 89);
    } else if (109..=110).contains(&ord) {
        return Ok(ord - 90);
    } else if (112..=122).contains(&ord) {
        return Ok(ord - 91);
    }
    Err(GeohashError::InvalidHashCharacter(c))
}

/// Decode a geohash into a longitude/latitude pair with some longitude/latitude error. The
/// return value is `(<longitude>, <latitude>, <longitude error>, <latitude error>)`.
///
/// ### Examples
///
/// Decoding a length five geohash:
///
/// ```rust
/// use fixed::types::I64F64;
/// let geohash_str = "9q60y";
/// let decoded = geohash::decode(geohash_str).expect("Invalid hash string");
/// assert_eq!(
///     decoded,
///     (
///         I64F64::from_num(-120.65185546875),
///         I64F64::from_num(35.31005859375),
///         I64F64::from_num(0.02197265625),
///         I64F64::from_num(0.02197265625),
///     ),
/// );
/// ```
///
/// Decoding a length ten geohash:
///
/// ```rust
/// use fixed::types::I64F64;
/// let geohash_str = "9q60y60rhs";
/// let decoded = geohash::decode(geohash_str).expect("Invalid hash string");
/// assert_eq!(
///     decoded,
///     (
///         I64F64::from_num(-120.66229999065399),
///         I64F64::from_num(35.300298035144806),
///         I64F64::from_num(0.000005364418029785156),
///         I64F64::from_num(0.000002682209014892578),
///     ),
/// );
/// ```
pub fn decode(hash_str: &str) -> Result<(I64F64, I64F64, I64F64, I64F64), GeohashError> {
    let rect = decode_bbox(hash_str)?;
    let c0 = rect.min;
    let c1 = rect.max;
    let two = I64F64::from_num(2);
    Ok((
        (c0.lon + c1.lon) / two, // longitude
        (c0.lat + c1.lat) / two, // latitude
        (c1.lon - c0.lon) / two, // longitude error
        (c1.lat - c0.lat) / two, // latitude error
    ))
}

/// Find neighboring geohashes for the given geohash and direction.
pub fn neighbor(hash_str: &str, direction: Direction) -> Result<String, GeohashError> {
    let (lon, lat, lon_err, lat_err) = decode(hash_str)?;
    let (dlat, dlng) = direction.to_tuple();
    let two = I64F64::from_num(2);
    let neighbor_lon = lon + two * lon_err.abs() * dlng;
    let neighbor_lat = lat + two * lat_err.abs() * dlat;
    encode(neighbor_lon, neighbor_lat, hash_str.len())
}

/// Find all neighboring geohashes for the given geohash.
///
/// ### Examples
///
/// ```
/// let geohash_str = "9q60y60rhs";
///
/// let neighbors = geohash::neighbors(geohash_str).expect("Invalid hash string");
///
/// assert_eq!(
///     neighbors,
///     geohash::Neighbors {
///         n: "9q60y60rht".to_owned(),
///         ne: "9q60y60rhv".to_owned(),
///         e: "9q60y60rhu".to_owned(),
///         se: "9q60y60rhg".to_owned(),
///         s: "9q60y60rhe".to_owned(),
///         sw: "9q60y60rh7".to_owned(),
///         w: "9q60y60rhk".to_owned(),
///         nw: "9q60y60rhm".to_owned(),
///     }
/// );
/// ```
pub fn neighbors(hash_str: &str) -> Result<Neighbors, GeohashError> {
    Ok(Neighbors {
        sw: neighbor(hash_str, Direction::SW)?,
        s: neighbor(hash_str, Direction::S)?,
        se: neighbor(hash_str, Direction::SE)?,
        w: neighbor(hash_str, Direction::W)?,
        e: neighbor(hash_str, Direction::E)?,
        nw: neighbor(hash_str, Direction::NW)?,
        n: neighbor(hash_str, Direction::N)?,
        ne: neighbor(hash_str, Direction::NE)?,
    })
}
