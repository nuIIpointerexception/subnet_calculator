use std::str::FromStr;

use crate::ipv4::Ipv4Addr;
use clipboard_rs::{Clipboard, ClipboardContext};
use iced::widget::{scrollable, Button, Column, Container, Row, Text, TextInput};
use iced::{color, Application, Command, Element, Length, Settings, Theme};

use crate::subnet::{Calculator, Subnet};

mod ipv4;
mod subnet;

struct SubnetCalculator {
    network_address: String,
    num_hosts: String,
    last_address: Option<Ipv4Addr>,
    result: Option<Subnet>,
    error_message: String,
    history: Vec<Subnet>,
    clipboard: ClipboardContext,
}

#[derive(Debug, Clone)]
enum Message {
    NetworkAddressChanged(String),
    NumHostsChanged(String),
    Calculate,
    Reset,
    CopyHistory(usize),
}

impl Application for SubnetCalculator {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::Dark.to_owned()
    }

    fn new(_flags: ()) -> (SubnetCalculator, Command<Message>) {
        (
            SubnetCalculator {
                network_address: String::new(),
                num_hosts: String::new(),
                last_address: None,
                result: None,
                error_message: String::new(),
                history: Vec::new(),
                clipboard: ClipboardContext::new().unwrap(),
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
            Message::NumHostsChanged(num_hosts) => self.num_hosts = num_hosts,
            Message::Calculate => {
                self.error_message.clear();
                let num_hosts = self.num_hosts.parse().unwrap_or(0);
                let last_address = match self.last_address {
                    Some(addr) => addr,
                    None => {
                        self.error_message = "Invalid network address".to_string();
                        return Command::none();
                    }
                };
                if num_hosts == 0 {
                    self.error_message = "Invalid number of hosts".to_string();
                    return Command::none();
                }
                let result = Calculator::generate_subnet(last_address, num_hosts);
                self.result = Some(result.0);
                self.last_address = Some(result.1);
                self.history.push(result.0);
            }
            Message::Reset => {
                self.network_address.clear();
                self.num_hosts.clear();
                self.last_address = None;
                self.result = None;
                self.error_message.clear();
                self.history.clear();
            }
            Message::CopyHistory(index) => {
                if let Some(result) = self.history.get(index) {
                    let result_text = format!(
                        "Address: {}\nMask: {}\nStart: {}\nEnd: {}\nBroadcast: {}\nHosts: {}",
                        result.address,
                        result.mask_length,
                        result.start,
                        result.end,
                        result.broadcast,
                        result.hosts
                    );
                    self.clipboard.set_text(result_text).unwrap();
                }
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

        let num_hosts_input = TextInput::new("Number of Hosts", &self.num_hosts)
            .on_input(Message::NumHostsChanged)
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
            Column::new().push(
                Text::new(format!(
                    "Address: {}\nMask: {}\nStart: {}\nEnd: {}\nBroadcast: {}\nHosts: {}",
                    result.address,
                    result.mask_length,
                    result.start,
                    result.end,
                    result.broadcast,
                    result.hosts
                ))
                .size(20)
                .width(Length::Fill),
            )
        } else {
            Column::new().push(
                Text::new(&self.error_message)
                    .width(Length::Fill)
                    .size(20)
                    .style(color!(0xff0000)),
            )
        };

        let history_text = Column::new()
            .push(
                scrollable(Column::with_children(
                    self.history
                        .iter()
                        .enumerate()
                        .map(|(index, result)| {
                            Row::new()
                                .push(
                                    Text::new(format!(
                                        "Address: {}\nMask: {}\nStart: {}\nEnd: {}\nBroadcast: {}\nHosts: {}",
                                        result.address, result.mask_length, result.start, result.end,
                                        result.broadcast, result.hosts
                                    ))
                                    .width(Length::Fill)
                                    .size(16),
                                )
                                .push(
                                    Button::new("Copy")
                                        .on_press(Message::CopyHistory(index))
                                        .padding(10),
                                )
                                .spacing(10)
                                .padding(10)
                                .into()
                        })
                        .collect::<Vec<_>>(),
                ))
                .height(Length::Fill)
                .width(Length::Fill),
            )
            .padding(10);

        let content = Column::new()
            .push(title)
            .push(network_address_input)
            .push(num_hosts_input)
            .push(button_row)
            .push(result_text)
            .push(history_text)
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
