# IPv4 Subnet Calculator

The IPv4 Subnet Calculator is a Rust program that generates subnets based on a given network address and the number of required hosts.
It provides a simple and efficient way to calculate subnets for IPv4 networks.

## Features

- Calculates subnets based on the network address and the number of required hosts.
- Determines the subnet mask length, usable range, broadcast address, and the number of usable hosts for each subnet.
- Generates subnets sequentially based on the last address of the previous subnet.
- Provides a user-friendly command-line interface for inputting the network address and the number of required hosts.
- Cross-platform compatibility: runs on Windows, macOS, and Linux.

## Prerequisites

To run the IPv4 Subnet Calculator, you need to have Rust installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org](https://www.rust-lang.org)

## Usage

1. Clone the repository or download the source code files.

2. Open a terminal or command prompt and navigate to the directory containing the source code files.

3. Run the program using the following command:

```bash
cargo run --release
```

5. Follow the prompts to enter the network address and the number of required hosts for each subnet.

6. The program will generate and display the subnet information, including the subnet address, mask length, usable range, broadcast address, and the number of usable hosts.

7. You can continue generating subnets by entering the number of required hosts for the next subnet, or reset the last address and start over.

## License

This project is licensed under the [MIT License](LICENSE).