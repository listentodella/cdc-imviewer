use bytemuck::{Pod, Zeroable};
use clap::{Arg, Command};
use serialport;
use std::io::{self, Write};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[repr(C)]
#[derive(Copy, Debug, Clone)]
struct ImuDataType {
    magic_number: u64,
    frame_id: u64,
    timestamp: u64,
    data: [f32; 7],
}
// Manually implement the Pod and Zeroable traits
unsafe impl Pod for ImuDataType {}
unsafe impl Zeroable for ImuDataType {}
const IMU_DATA_SIZE: usize = std::mem::size_of::<ImuDataType>();
const IMU_DATA_MAGIC_NUMBER: u64 = 0x5a5a5a5a_a5a5a5a5u64;

static RECOVER: AtomicBool = AtomicBool::new(false);
/// 从缓冲区中提取一帧完整数据（52 字节）
fn extract_frame(buffer: &mut Vec<u8>) -> Option<[u8; IMU_DATA_SIZE]> {
    if buffer.len() >= IMU_DATA_SIZE {
        let mut frame = [0u8; IMU_DATA_SIZE];
        frame.copy_from_slice(&buffer[..IMU_DATA_SIZE]);
        // simple check magic number
        let magic_number = u64::from_le_bytes(frame[0..=7].try_into().unwrap());
        if magic_number != IMU_DATA_MAGIC_NUMBER {
            RECOVER.store(true, Ordering::Relaxed);
            eprintln!(
                "Invalid magic number: {:#x}, drop 1 Byte and waiting for luck...",
                magic_number
            );
            buffer.drain(..1);
            None
        } else {
            buffer.drain(..IMU_DATA_SIZE);
            match RECOVER.load(Ordering::Relaxed) {
                true => {
                    RECOVER.store(false, Ordering::Relaxed);
                    println!("Recovered from magic number error, drop first data");
                    None
                }
                false => Some(frame),
            }
        }
    } else {
        None
    }
}

fn main() {
    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
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
            port.clear(serialport::ClearBuffer::All).unwrap();
            let mut temp_buf: [u8; IMU_DATA_SIZE] = [0; IMU_DATA_SIZE];
            let mut buffer: Vec<u8> = Vec::new();
            println!("Receiving data on {} at {} baud:", port_name, baud_rate);
            let mut file = std::fs::File::create("imu_data.csv").unwrap();
            println!("frame,ts,GX,GY,GZ,AX,AY,AZ,temperature");
            writeln!(file, "frame,ts,GX,GY,GZ,AX,AY,AZ,temperature").unwrap();

            loop {
                match port.read(&mut temp_buf) {
                    Ok(t) => {
                        if t > 0 {
                            buffer.extend_from_slice(&temp_buf[..t]);
                            while let Some(frame) = extract_frame(&mut buffer) {
                                let imu_data: &ImuDataType = bytemuck::from_bytes(&frame);
                                // println!(
                                //     "frame_id: {}, timestamp: {}, data: {:?}",
                                //     imu_data.frame_id, imu_data.timestamp, imu_data.data
                                // );
                                if imu_data.frame_id == 0 {
                                    continue;
                                }

                                if imu_data.data[6] < 2.0 || imu_data.data[6] > 80.0 {
                                    eprintln!("our env is indoor, temp = {}, maybe we still in xfer error, drop this frame", imu_data.data[6]);
                                    continue;
                                }

                                if imu_data.frame_id % 1000 == 0 {
                                    println!(
                                        "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                                        imu_data.frame_id,
                                        imu_data.timestamp,
                                        imu_data.data[0],
                                        imu_data.data[1],
                                        imu_data.data[2],
                                        imu_data.data[3],
                                        imu_data.data[4],
                                        imu_data.data[5],
                                        imu_data.data[6]
                                    );
                                }

                                writeln!(
                                    file,
                                    "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                                    imu_data.frame_id,
                                    imu_data.timestamp,
                                    imu_data.data[0],
                                    imu_data.data[1],
                                    imu_data.data[2],
                                    imu_data.data[3],
                                    imu_data.data[4],
                                    imu_data.data[5],
                                    imu_data.data[6]
                                )
                                .unwrap();
                            }
                        } else {
                            // println!("Read 0 bytes.");
                            println!("xfer some error...drop...");
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("Error reading port: {:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}

fn valid_baud(val: &str) -> Result<(), String> {
    val.parse::<u32>()
        .map(|_| ())
        .map_err(|_| format!("Invalid baud rate '{}' specified", val))
}
