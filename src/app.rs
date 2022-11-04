use std::thread;
use crate::{ChartsData, InfoTraffic, parse_packets_loop, Sniffer, Status, TrafficChart};
use iced::{executor, Application, Column, Command, Container, Element, Length, Subscription};
use std::time::Duration;
use pcap::Device;
use crate::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::gui_initial_page::initial_page;
use crate::gui_run_page::run_page;
use crate::style::{Mode};


pub const PERIOD_RUNNING: u64 = 1000; //milliseconds
pub const PERIOD_INIT: u64 = 5000; //milliseconds


#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    AdapterSelection(String),
    IpVersionSelection(String),
    TransportProtocolSelection(TransProtocol),
    AppProtocolSelection(AppProtocol),
    ChartSelection(String),
    OpenReport,
    OpenGithub,
    Start,
    Reset,
    Style,
}


impl Application for Sniffer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Sniffer;

    fn new(flags: Sniffer) -> (Sniffer, Command<Message>) {
        (
            flags,
            Command::none(),
        )
    }


    fn title(&self) -> String {
        String::from("Sniffnet")
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {}
            Message::AdapterSelection(name) => {
                for dev in Device::list().expect("Error retrieving device list\r\n") {
                    if dev.name.eq(&name) {
                        *self.device.lock().unwrap() = dev;
                        //println!("{}",dev.addresses.len());
                        break;
                    }
                }
            }
            Message::IpVersionSelection(version) => {
                self.filters.lock().unwrap().ip = version;
            }
            Message::TransportProtocolSelection(protocol) => {
                self.filters.lock().unwrap().transport = protocol;
            }
            Message::AppProtocolSelection(protocol) => {
                self.filters.lock().unwrap().application = protocol;
            }
            Message::ChartSelection(what_to_display) => {
                if what_to_display.eq("packets") {
                    self.chart_packets = true;
                }
                else {
                    self.chart_packets = false;
                }
            }
            Message::OpenReport => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg("-t")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "linux")]
                std::process::Command::new("explorer")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
            }
            Message::OpenGithub => {
                #[cfg(target_os = "windows")]
                std::process::Command::new("explorer")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "macos")]
                std::process::Command::new("open")
                    .arg("https://github.com/GyulyVGC/sniffnet")
                    .spawn()
                    .unwrap();
                #[cfg(target_os = "linux")]
                std::process::Command::new("explorer")
                    .arg("./sniffnet_report/report.txt")
                    .spawn()
                    .unwrap();
            }
            Message::Start => {
                let current_capture_id = self.current_capture_id.clone();
                let device = self.device.clone();
                let filters = self.filters.clone();
                let info_traffic_mutex = self.info_traffic.clone();
                *info_traffic_mutex.lock().unwrap() = InfoTraffic::new();
                let charts_data_mutex = self.charts_data.clone();
                *charts_data_mutex.lock().unwrap() = ChartsData::new();
                *self.status_pair.0.lock().unwrap() = Status::Running;
                self.traffic_chart = TrafficChart::new(info_traffic_mutex.clone(), charts_data_mutex.clone());
                self.status_pair.1.notify_all();
                thread::spawn(move || {
                    parse_packets_loop(current_capture_id, device,
                                       0, 65535, filters,
                                       info_traffic_mutex);
                });
            }
            Message::Reset => {
                *self.current_capture_id.lock().unwrap() += 1; //change capture id to kill previous capture and to rewrite output file
                *self.status_pair.0.lock().unwrap() = Status::Init;
            }
            Message::Style => {
                self.style = if self.style == Mode::Day {
                    Mode::Night
                } else {
                    Mode::Day
                };
            }
        }
        Command::none()
    }


    fn subscription(&self) -> Subscription<Message> {
        match *self.status_pair.0.lock().unwrap() {
            Status::Running => {
                iced::time::every(Duration::from_millis(PERIOD_RUNNING)).map(|_| Message::Tick)
            }
            _ => {
                iced::time::every(Duration::from_millis(PERIOD_INIT)).map(|_| Message::Tick)
            }
        }
    }


    fn view(&mut self) -> Element<Message> {
        let status = *self.status_pair.0.lock().unwrap();
        let mode = self.style;

        let body = match status {
            Status::Init => {
                initial_page(self)
            }
            Status::Running => {
                run_page(self)
            }
            Status::Stop => { Column::new() }
        };

        Container::new(
            body
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(mode)
            .into()
    }
}