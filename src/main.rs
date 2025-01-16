use clap::{Arg, Command};
use color_eyre::Result;

use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Chart, Dataset, GraphType, LegendPosition},
    DefaultTerminal, Frame,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct ImuDataType {
    frame_id: u64,
    timestamp: u64,
    data: [f32; 7],
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);

    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .required(true)
                .validator(valid_baud),
        )
        .get_matches();

    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            //let mut serial_buf: Vec<u8> = vec![0; 48];
            let mut serial_buf: [u8; 48] = [0; 48];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(&mut serial_buf) {
                    Ok(t) => {
                        if t != 48 {
                            println!("error len = {}", t);
                        } else {
                            let imu_data: ImuDataType = unsafe { std::mem::transmute(serial_buf) };
                            println!(
                                "frame_id: {}, timestamp: {}, data:{:?}",
                                imu_data.frame_id, imu_data.timestamp, imu_data.data
                            );
                            // println!("frame_id: {}, timestamp: {}, acc:{}, gyr:{}, temp:{}",
                            //     imu_data.frame_id, imu_data.timestamp,
                            //     imu_data.data[0..=2],
                            //     imu_data.data[3..=5],
                            //     imu_data.data[6]);
                        }
                        //io::stdout().write_fmt(format_args!("data len = {}\n", t)).unwrap();
                        //io::stdout().write_all(&serial_buf[..t]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }

    ratatui::restore();
    app_result
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}

struct App {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    signal3: SinSignal,
    data3: Vec<(f64, f64)>,
    window: [f64; 2],
}

#[derive(Clone)]
struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

impl App {
    fn new() -> Self {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.2, 3.0, 10.0);
        let mut signal3 = SinSignal::new(0.2, 3.0, 14.0);
        let data1 = signal1.by_ref().take(20).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(20).collect::<Vec<(f64, f64)>>();
        let data3 = signal3.by_ref().take(20).collect::<Vec<(f64, f64)>>();
        Self {
            signal1,
            data1,
            signal2,
            data2,
            signal3,
            data3,
            window: [0.0, 10.0],
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.data1.drain(0..5);
        self.data1.extend(self.signal1.by_ref().take(5));

        self.data2.drain(0..5);
        self.data2.extend(self.signal2.by_ref().take(5));

        self.data3.drain(0..5);
        self.data3.extend(self.signal3.by_ref().take(5));

        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }

    fn draw(&self, frame: &mut Frame) {
        let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());
        let [animated_chart, bar_chart] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(29)]).areas(top);
        let [line_chart, scatter] = Layout::horizontal([Constraint::Fill(1); 2]).areas(bottom);

        self.render_animated_chart(frame, animated_chart);
    }

    fn render_animated_chart(&self, frame: &mut Frame, area: Rect) {
        let x_labels = vec![
            Span::styled(
                format!("{}", self.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", (self.window[0] + self.window[1]) / 2.0)),
            Span::styled(
                format!("{}", self.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .name("x")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Red))
                .data(&self.data1),
            Dataset::default()
                .name("y") //.marker(symbols::Marker::Braille)
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Green))
                .data(&self.data2),
            Dataset::default()
                .name("z") //.marker(symbols::Marker::Braille)
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Blue))
                .data(&self.data3),
        ];

        let chart = Chart::new(datasets)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels(x_labels)
                    .bounds(self.window),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels(["-100".bold(), "0".into(), "100".bold()])
                    .bounds([-100.0, 100.0]),
            );

        frame.render_widget(chart, area);
    }
}
