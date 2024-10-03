use std::io::{self, Write};
use std::time::Duration;

use clap::{Arg, Command};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct ImuDataType {
    frame_id: u64,
    timestamp: u64,
    data: [f32; 7],
}

fn main() {
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
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
