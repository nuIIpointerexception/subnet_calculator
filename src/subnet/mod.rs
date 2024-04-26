use std::net::Ipv4Addr;

pub struct Calculator;

/// Represents a subnet with its address, mask length, usable range, broadcast address, and number of hosts.
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
    fn calc_length(num_hosts: u32) -> u32 {
        32 - (num_hosts as f64).log2().ceil() as u32
    }

    /// Calculates the address increment for a given subnet mask length.
    fn calc_increment(mask_len: u32) -> u32 {
        2_u32.pow(32 - mask_len)
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
    /// A `Subnet` struct containing the subnet details.
    pub fn generate_subnet(last_address: u32, num_hosts: u32) -> (Subnet, u32) {
        let network_address = Ipv4Addr::from(last_address);
        let required_subnet_mask_length = Self::calc_length(num_hosts);
        let subnet = Self::generate_subnet_from_address(network_address, required_subnet_mask_length);
        let next_address = u32::from(subnet.broadcast) + 1;
        (subnet, next_address)
    }

    fn generate_subnet_from_address(network_address: Ipv4Addr, subnet_mask_length: u32) -> Subnet {
        let address_increment = Self::calc_increment(subnet_mask_length);
        let base_address = u32::from(network_address);
        let usable_range_start = Ipv4Addr::from(base_address + 1);
        let usable_range_end = Ipv4Addr::from(base_address + address_increment - 2);
        let hosts = address_increment - 2;
        let broadcast_address = Ipv4Addr::from(base_address + address_increment - 1);
        let subnet_address = Ipv4Addr::from(base_address);


        Subnet {
            address: subnet_address,
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
    use std::net::Ipv4Addr;


    #[test]
    fn test_sequential_subnets() {
        let initial_address = Ipv4Addr::new(192, 168, 0, 0);
        let mut last_address = u32::from(initial_address);

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