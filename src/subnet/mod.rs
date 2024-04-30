use crate::ipv4::Ipv4Addr;

/// IPv4 Subnet Calculator
///
/// This calculator generates subnets based on a given network address and the number of required hosts.
///
/// The subnet calculation process involves the following steps:
///
/// 1. Determine the number of bits needed for the subnet mask:
///    - Find the next power of two greater than or equal to the number of required hosts.
///    - Subtract the position of the most significant bit of that power of two from 32.
///    - The result is the number of bits needed for the subnet mask.
///
/// 2. Calculate the address increment:
///    - Left-shift 1 by (32 - subnet mask length) bits.
///    - This gives the increment between consecutive subnets.
///
/// 3. Generate the subnet:
///    - The network address is the last address of the previous subnet.
///    - The usable range starts at (network address + 1).
///    - The broadcast address is (network address + address increment - 1).
///    - The usable range ends at (broadcast address - 1).
///    - The number of usable hosts is (address increment - 2).
///
/// Example:
///
/// For a network address of 192.168.0.0 and 50 required hosts:
/// - The next power of two greater than or equal to 50 is 64.
/// - The most significant bit of 64 is at position 6 (starting from 0).
/// - The subnet mask length is 32 - 6 = 26 bits.
/// - The address increment is 1 << (32 - 26) = 64.
/// - The usable range starts at 192.168.0.1 and ends at 192.168.0.62.
/// - The broadcast address is 192.168.0.63.
/// - The number of usable hosts is 64 - 2 = 62.
///
/// The calculator generates subnets sequentially based on the last address of the previous subnet
/// and the number of required hosts for each subnet.
pub struct Calculator;

/// Represents a subnet with its address, mask length, usable range, broadcast address, and number of hosts.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Subnet {
    pub address: Ipv4Addr,
    pub mask_length: u32,
    pub start: Ipv4Addr,
    pub end: Ipv4Addr,
    pub broadcast: Ipv4Addr,
    pub hosts: u32,
}

impl Calculator {
    /// Calculates the number of bits needed to represent the given number of hosts.
    ///
    /// # Arguments
    ///
    /// * `num_hosts` - The number of hosts required in the subnet.
    ///
    /// # Returns
    ///
    /// The number of bits needed to represent the hosts.
    fn calc_length(num_hosts: u32) -> u32 {
        32 - num_hosts.next_power_of_two().trailing_zeros()
    }

    /// Calculates the address increment for a given subnet mask length.
    ///
    /// # Arguments
    ///
    /// * `mask_len` - The subnet mask length.
    ///
    /// # Returns
    ///
    /// The address increment.
    fn calc_increment(mask_len: u32) -> u32 {
        1 << (32 - mask_len)
    }

    /// Generates a single subnet for the given network address and number of hosts.
    ///
    /// # Arguments
    ///
    /// * `last_address` - The last address in the previous subnet.
    /// * `num_hosts` - The number of hosts required in the subnet.
    ///
    /// # Returns
    ///
    /// A tuple containing the generated `Subnet` and the next address.
    pub fn generate_subnet(last_address: Ipv4Addr, num_hosts: u32) -> (Subnet, Ipv4Addr) {
        let network_address = last_address;
        let required_subnet_mask_length = Self::calc_length(num_hosts);
        let subnet =
            Self::generate_subnet_from_address(network_address, required_subnet_mask_length);
        let next_address = Ipv4Addr::from_u32(subnet.broadcast.to_u32() + 1);
        (subnet, next_address)
    }

    /// Generates a subnet from the given network address and subnet mask length.
    ///
    /// # Arguments
    ///
    /// * `network_address` - The network address.
    /// * `subnet_mask_length` - The subnet mask length.
    ///
    /// # Returns
    ///
    /// The generated `Subnet`.
    fn generate_subnet_from_address(network_address: Ipv4Addr, subnet_mask_length: u32) -> Subnet {
        let address_increment = Self::calc_increment(subnet_mask_length);
        let base_address = network_address.to_u32();
        let usable_range_start = Ipv4Addr::from_u32(base_address + 1);
        let broadcast_address = Ipv4Addr::from_u32(base_address + address_increment - 1);
        let usable_range_end = Ipv4Addr::from_u32(broadcast_address.to_u32() - 1);
        let hosts = address_increment - 2;

        Subnet {
            address: network_address,
            mask_length: subnet_mask_length,
            start: usable_range_start,
            end: usable_range_end,
            broadcast: broadcast_address,
            hosts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_subnets() {
        let initial_address = Ipv4Addr::new(192, 168, 0, 0);
        let mut last_address = initial_address;

        // Test for 50 hosts
        let (result1, next_address1) = Calculator::generate_subnet(last_address, 50);
        assert_eq!(result1.address, Ipv4Addr::new(192, 168, 0, 0));
        assert_eq!(result1.mask_length, 26);
        assert_eq!(result1.start, Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(result1.end, Ipv4Addr::new(192, 168, 0, 62));
        assert_eq!(result1.broadcast, Ipv4Addr::new(192, 168, 0, 63));
        assert_eq!(result1.hosts, 62);
        last_address = next_address1;

        // Test for 20 hosts
        let (result2, next_address2) = Calculator::generate_subnet(last_address, 20);
        assert_eq!(result2.address, Ipv4Addr::new(192, 168, 0, 64));
        assert_eq!(result2.mask_length, 27);
        assert_eq!(result2.start, Ipv4Addr::new(192, 168, 0, 65));
        assert_eq!(result2.end, Ipv4Addr::new(192, 168, 0, 94));
        assert_eq!(result2.broadcast, Ipv4Addr::new(192, 168, 0, 95));
        assert_eq!(result2.hosts, 30);
        last_address = next_address2;

        // Test for 5 hosts
        let (result3, _) = Calculator::generate_subnet(last_address, 5);
        assert_eq!(result3.address, Ipv4Addr::new(192, 168, 0, 96));
        assert_eq!(result3.mask_length, 29);
        assert_eq!(result3.start, Ipv4Addr::new(192, 168, 0, 97));
        assert_eq!(result3.end, Ipv4Addr::new(192, 168, 0, 102));
        assert_eq!(result3.broadcast, Ipv4Addr::new(192, 168, 0, 103));
        assert_eq!(result3.hosts, 6);
    }
}
