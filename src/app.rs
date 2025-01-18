use color_eyre::Result;
use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset},
    DefaultTerminal, Frame,
};

pub struct App {
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
    // 决定了实际的值,在图上与轴的值的范围的缩放比例
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
        //TODO: fetch imu data here
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

impl App {
    pub fn new() -> Self {
        let mut signal1 = SinSignal::new(0.2, 5.0, 100.0);
        let mut signal2 = SinSignal::new(0.2, 10.0, 100.0);
        let mut signal3 = SinSignal::new(0.2, 20.0, 100.0);
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

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
        // 从现有的数据中,取出一定笔数
        self.data1.drain(0..5);
        // 再从数据源, 填充一定量的数据
        self.data1.extend(self.signal1.by_ref().take(5));

        self.data2.drain(0..5);
        self.data2.extend(self.signal2.by_ref().take(5));

        self.data3.drain(0..5);
        self.data3.extend(self.signal3.by_ref().take(5));

        // window 有何作用?
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }

    fn draw(&self, frame: &mut Frame) {
        let [top] = Layout::vertical([Constraint::Fill(1); 1]).areas(frame.area());
        let [animated_chart] = Layout::horizontal([Constraint::Fill(1); 1]).areas(top);

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
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::Red))
                .data(&self.data1),
            Dataset::default()
                .name("y")
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::Green))
                .data(&self.data2),
            Dataset::default()
                .name("z")
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::Blue))
                .data(&self.data3),
        ];

        let chart = Chart::new(datasets)
            .block(Block::bordered())
            // x轴的含义
            .x_axis(
                Axis::default()
                    .title("timestamp/us")
                    .style(Style::default().fg(Color::Gray))
                    .labels(x_labels)
                    .bounds(self.window),
            )
            // y轴的含义
            .y_axis(
                Axis::default()
                    .title("m/s^2")
                    .style(Style::default().fg(Color::Gray))
                    .labels(["-100".bold(), "0".into(), "100".bold()])
                    .bounds([-100.0, 100.0]),
            );

        frame.render_widget(chart, area);
    }
}
