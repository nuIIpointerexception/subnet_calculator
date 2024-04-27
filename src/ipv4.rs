/// Represents an IPv4 address.
///
/// The `Ipv4Addr` struct stores an IPv4 address as an array of four octets.
/// It provides methods to create an `Ipv4Addr` from individual octets or a `u32` value,
/// and to convert an `Ipv4Addr` back to a `u32` value.
///
/// The struct derives the `Debug`, `Clone`, `Copy`, `PartialEq`, and `Eq` traits for
/// convenient debugging, cloning, copying, and equality comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Addr {
    /// The four octets of the IPv4 address.
    /// Example: [192, 168, 0, 1]
    pub octets: [u8; 4],
}

impl Ipv4Addr {
    /// Creates a new `Ipv4Addr` from four individual octets.
    ///
    /// This method is marked with `#[allow(dead_code)]` to suppress warnings
    /// about unused code, as it may not be used in all cases.
    ///
    /// # Arguments
    ///
    /// * `a` - The first octet of the IPv4 address.
    /// * `b` - The second octet of the IPv4 address.
    /// * `c` - The third octet of the IPv4 address.
    /// * `d` - The fourth octet of the IPv4 address.
    ///
    /// # Returns
    ///
    /// A new `Ipv4Addr` instance with the specified octets.
    #[allow(dead_code)]
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { octets: [a, b, c, d] }
    }

    /// Creates a new `Ipv4Addr` from a `u32` value.
    ///
    /// The `u32` value is expected to be in network byte order.
    ///
    /// # Arguments
    ///
    /// * `ip` - The `u32` value representing the IPv4 address.
    ///
    /// # Returns
    ///
    /// A new `Ipv4Addr` instance with the octets extracted from the `u32` value.
    pub fn from_u32(ip: u32) -> Self {
        let octets = [
            (ip >> 24) as u8,
            (ip >> 16) as u8,
            (ip >> 8) as u8,
            ip as u8,
        ];
        Self { octets }
    }

    /// Converts the `Ipv4Addr` to a `u32` value.
    ///
    /// The resulting `u32` value is in network byte order.
    ///
    /// # Returns
    ///
    /// The `u32` value representing the IPv4 address.
    pub fn to_u32(&self) -> u32 {
        ((self.octets[0] as u32) << 24)
            | ((self.octets[1] as u32) << 16)
            | ((self.octets[2] as u32) << 8)
            | (self.octets[3] as u32)
    }
}

/// Implements the `Display` trait for `Ipv4Addr` to provide a custom string representation.
///
/// The `Ipv4Addr` is displayed in the format "a.b.c.d", where a, b, c, and d are the
/// individual octets of the IPv4 address.
impl std::fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.octets[0], self.octets[1], self.octets[2], self.octets[3]
        )
    }
}

/// Implements the `FromStr` trait for `Ipv4Addr` to allow parsing from a string.
///
/// The `FromStr` trait allows creating an `Ipv4Addr` from a string representation
/// in the format "a.b.c.d", where a, b, c, and d are the individual parts of the
/// IPv4 address.
///
/// If the string is not a valid IPv4 address, an error is returned.
use std::str::FromStr;

impl FromStr for Ipv4Addr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let part: Vec<&str> = s.split('.').collect();
        if part.len() != 4 {
            return Err(format!("Invalid IPv4 address: {}", s));
        }

        let mut parsed = [0; 4];
        for (i, octet) in part.iter().enumerate() {
            match octet.parse::<u8>() {
                Ok(value) => parsed[i] = value,
                Err(_) => return Err(format!("Invalid IPv4 address: {}", s)),
            }
        }

        Ok(Ipv4Addr { octets: parsed })
    }
}
