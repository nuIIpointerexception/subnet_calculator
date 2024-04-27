use std::str::FromStr;

use iced::{Application, color, Command, Element, Length, Settings, Theme};
use iced::widget::{Button, Column, Container, Row, Text, TextInput};
use crate::ipv4::Ipv4Addr;

use crate::subnet::{Calculator, Subnet};

mod subnet;
mod ipv4;

struct SubnetCalculator {
    network_address: String,
    num_networks: String,
    last_address: Option<Ipv4Addr>,
    result: Option<Subnet>,
    error_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    NetworkAddressChanged(String),
    NumNetworksChanged(String),
    Calculate,
    Reset,
}

impl Application for SubnetCalculator {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::Nord.to_owned()
    }

    fn new(_flags: ()) -> (SubnetCalculator, Command<Message>) {
        (
            SubnetCalculator {
                network_address: String::new(),
                num_networks: String::new(),
                last_address: None,
                result: None,
                error_message: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("IPv4 Subnet Calculator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NetworkAddressChanged(network_address) => {
                self.network_address = network_address;
                self.last_address = Ipv4Addr::from_str(&self.network_address).ok();
            }
            Message::NumNetworksChanged(num_networks) => self.num_networks = num_networks,
            Message::Calculate => {
                self.error_message.clear();
                let num_networks = self.num_networks.parse::<u32>().unwrap_or_else(|_| {
                    self.error_message = "Invalid number of networks".to_string();
                    0
                });
                let last_address = match self.last_address {
                    Some(addr) => addr,
                    None => {
                        self.error_message = "Invalid network address".to_string();
                        return Command::none();
                    }
                };
                if !self.error_message.is_empty() {
                    return Command::none();
                }
                let result = Calculator::generate_subnet(last_address, num_networks);
                self.result = Some(result.0);
                self.last_address = Some(result.1);
            }
            Message::Reset => {
                self.network_address.clear();
                self.num_networks.clear();
                self.last_address = None;
                self.result = None;
                self.error_message.clear();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = Text::new("IPv4 Subnet Calculator")
            .size(30)
            .width(Length::Fill);

        let network_address_input = TextInput::new("Network Address", &self.network_address)
            .on_input(Message::NetworkAddressChanged)
            .padding(10);

        let num_networks_input = TextInput::new("Number of Networks", &self.num_networks)
            .on_input(Message::NumNetworksChanged)
            .padding(10);

        let button_row = Row::new()
            .push(
                Button::new("Calculate")
                    .on_press(Message::Calculate)
                    .padding(10),
            )
            .push(Button::new("Reset").on_press(Message::Reset).padding(10))
            .spacing(10);

        let result_text = if let Some(result) = &self.result {
            Column::new()
                .push(Text::new("Result").size(20).width(Length::Fill))
                .push(
                    Container::new(
                        Text::new(format!(
                            "Address: {}\nMask: {}\nStart: {}\nEnd: {}\nBroadcast: {}\nHosts: {}",
                            result.address,
                            result.mask_length,
                            result.start,
                            result.end,
                            result.broadcast,
                            result.hosts
                        ))
                            .width(Length::Fill)
                            .size(16),
                    )
                        .padding(10),
                )
        } else {
            Column::new().push(
                Text::new(&self.error_message)
                    .width(Length::Fill)
                    .size(20)
                    .style(color!(0xff0000)),
            )
        };

        let content = Column::new()
            .push(title)
            .push(network_address_input)
            .push(num_networks_input)
            .push(button_row)
            .push(result_text)
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    SubnetCalculator::run(Settings::default())
}