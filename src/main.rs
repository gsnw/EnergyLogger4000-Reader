use std::{fs, env};
use std::io::{self, Read};

use getopts::Options;
use chrono::{NaiveDateTime, Duration};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROGRAM: &str = env!("CARGO_PKG_NAME");

#[derive(Debug)]
#[allow(dead_code)]
struct InfoDay {
  day: usize,
  value: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Info {
  total_power_consumption: f32,
  total_recorded_time: f32,
  total_on_time: f32,
  total_kwh_today_min: Vec<InfoDay>,
  total_recorded_time_today_min: Vec<InfoDay>,
  total_on_time_today_min: Vec<InfoDay>,
  unit_id: u8,
  tariff1: f64,
  tariff2: f64,
  timestamp: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct DataSample {
  timestamp: String,
  voltage: f32,
  current: f32,
  power_factor: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Data {
  timestamp: String,
  data_samples: Vec<DataSample>,
}

fn print_usage(opts: Options) {
  let brief = format!("Usage: {} [options]", PROGRAM);
  print!("{}", opts.usage(&brief));
}

fn print_version() {
  println!("{} - v{}", PROGRAM, VERSION);
}

#[allow(dead_code)]
fn print_debug_buffer(buffer: &[u8]) {
  println!("All bytes from the file:");
  println!("Number of bytes: {}", buffer.len());
  for (index, byte) in buffer.iter().enumerate() {
    println!("Byte {}: 0x{:02X}", index, byte);
  }
}

fn print_info_file(info: &Info) {
  println!("--- INFO ---");
  println!("Unit ID: {}", info.unit_id);
  println!("Timestamp: {}", info.timestamp);
  println!("Tarif 1: {}", info.tariff1);
  println!("Tarif 2: {}", info.tariff2);
  println!("Total power comsumption: {} kWh", info.total_power_consumption);
  println!("Total recorded time {} h", info.total_recorded_time);
  println!("Total on time {} h", info.total_on_time);
  if info.total_kwh_today_min.len() == 10 && info.total_recorded_time_today_min.len() == 10 && info.total_on_time_today_min.len() == 10 {
    for i in 0..10 {
      println!("- Day: {}", i + 1);
      println!("-- Total kWh today min: {} kWh", info.total_kwh_today_min[i].value);
      println!("-- Total recorded time today min: {} h", info.total_recorded_time_today_min[i].value);
      println!("-- Total on time today min: {} h", info.total_on_time_today_min[i].value);
    }
  }
}

fn print_data_file(data: &Data) {
  for record in &data.data_samples {
    print!("[{}] ", record.timestamp);
    print!("U={}V ", record.voltage);
    print!("I={}mA ", record.current);
    // \u{03C6} print sign cosPhi
    println!("cos\u{03C6}={}%", record.power_factor);
  }
}

fn read_info_file(buffer: &[u8]) -> Result<Info, Box<dyn std::error::Error>> {

  let total_power_consumption = ((buffer[5] as u32) * 256 * 256 + (buffer[6] as u32) * 256 + (buffer[7] as u32))  as f32 / 1000.0;
  let total_recorded_time = ((buffer[8] as u32) * 256 * 256 + (buffer[9] as u32) * 256 + (buffer[10] as u32))  as f32 / 100.0;
  let total_on_time = ((buffer[11] as u32) * 256 * 256 + (buffer[12] as u32) * 256 + (buffer[13] as u32))  as f32 / 100.0;

  let mut total_kwh_today_min: Vec<InfoDay> = Vec::new();
  let total_kwh_today_min_start_index = 14;
  let total_kwh_today_min_record_size = 3; // Read next 3 Bytes
  let total_kwh_today_min_num_records = 10;
  
  for day in 0..total_kwh_today_min_num_records {
    let offset = total_kwh_today_min_start_index + day * total_kwh_today_min_record_size;
    let record_bytes = &buffer[offset..offset + total_kwh_today_min_record_size];
    let loop_total_kwh_today_min = ((record_bytes[0] as u32) * 256 * 256 + (record_bytes[1] as u32) * 256 + (record_bytes[2] as u32))  as f32 / 1000.0;
    total_kwh_today_min.push(InfoDay {
      day: day,
      value: loop_total_kwh_today_min,
    });
  }

  let mut total_recorded_time_today_min: Vec<InfoDay> = Vec::new();
  let total_recorded_time_today_min_start_index = 44;
  let total_recorded_time_today_min_record_size = 2; // Read next 2 Bytes
  let total_recorded_time_today_min_records = 10;
  
  for day in 0..total_recorded_time_today_min_records {
    let offset = total_recorded_time_today_min_start_index + day * total_recorded_time_today_min_record_size;
    let record_bytes = &buffer[offset..offset + total_recorded_time_today_min_record_size];
    let loop_total_recorded_time_today_min= ((record_bytes[0] as u16) * 256 + (record_bytes[1] as u16)) as f32 / 100.0;
    total_recorded_time_today_min.push(InfoDay {
      day: day,
      value: loop_total_recorded_time_today_min,
    });
  }

  let mut total_on_time_today_min: Vec<InfoDay> = Vec::new();
  let total_on_time_today_min_start_index = 64;
  let total_on_time_today_min_record_size = 2; // Read next 2 Bytes
  let total_on_time_today_min_records = 10;

  for day in 0..total_on_time_today_min_records {
    let offset = total_on_time_today_min_start_index + day * total_on_time_today_min_record_size;
    let record_bytes = &buffer[offset..offset + total_on_time_today_min_record_size];
    let loop_total_on_time_today_min= ((record_bytes[0] as u16) * 256 + (record_bytes[1] as u16)) as f32 / 100.0;
    total_on_time_today_min.push(InfoDay {
      day: day,
      value: loop_total_on_time_today_min,
    });
  }

  let unit_id = buffer[84];

  let tariff1 = (buffer[85] as u32) * 256 * 256 *256 + (buffer[86] as u32) * 256 * 256 + (buffer[87] as u32) * 256 + (buffer[88] as u32);
  let mut decoded_tariff1 = 0.0;
  for i in 0..4 {
    let byte = ((tariff1 >> (8 * (3 - i))) & 0xFF) as u32;
    decoded_tariff1 += (byte as f64) * 10f64.powi(-(i as i32));
  }

  let tariff2 = (buffer[89] as u32) * 256 * 256 *256 + (buffer[90] as u32) * 256 * 256 + (buffer[91] as u32) * 256 + (buffer[92] as u32);
  let mut decoded_tariff2 = 0.0;
  for i in 0..4 {
    let byte = ((tariff2 >> (8 * (3 - i))) & 0xFF) as u32;
    decoded_tariff2 += (byte as f64) * 10f64.powi(-(i as i32));
  }

  let time_hour = buffer[93];
  let time_minute = buffer[94];
  let date_month = buffer[95];
  let date_day = buffer[96];
  let date_year = buffer[97];

  let timestamp = NaiveDateTime::parse_from_str(&format!("20{:02}-{:02}-{:02} {:02}:{:02}:00", date_year, date_month, date_day, time_hour, time_minute), "%Y-%m-%d %H:%M:%S").expect("Invalid Date/Time-Format");

  if buffer[98..102] != [0xFF, 0xFF, 0xFF, 0xFF] {
    let error_message =format!("Info file has no end of file code");
    return Err(Box::<dyn std::error::Error>::from(error_message));
  }

  let result = Info {
    total_power_consumption: total_power_consumption,
    total_recorded_time: total_recorded_time,
    total_on_time: total_on_time,
    total_kwh_today_min: total_kwh_today_min,
    total_recorded_time_today_min: total_recorded_time_today_min,
    total_on_time_today_min,
    unit_id: unit_id,
    tariff1: decoded_tariff1,
    tariff2: decoded_tariff2,
    timestamp: timestamp.to_string(),
  };

  Ok(result) 
}

fn read_data_file(buffer: &[u8]) -> Result<Data, Box<dyn std::error::Error>> {
  let date_month = buffer[3];
  let date_day = buffer[4];
  let date_year = buffer[5];
  let time_hour = buffer[6];
  let time_minute = buffer[7];

  let timestamp = NaiveDateTime::parse_from_str(&format!("20{:02}-{:02}-{:02} {:02}:{:02}:00", date_year, date_month, date_day, time_hour, time_minute), "%Y-%m-%d %H:%M:%S").expect("Invalid Date/Time-Format");

  let mut data_samples: Vec<DataSample> = Vec::new();
  let mut start_timestamp = timestamp;
  let mut buffer_index = 8;
  while buffer_index + 4 < buffer.len() {
    if buffer[buffer_index..].iter().take(4).all(|&b| b == 0xFF) {
      break;
    }

    // 2 Bytes: Voltage (in tenths of volt) = byte(0) * 256 + byte(1)
    let voltage: f32 = ((buffer[buffer_index] as u16) * 256 + (buffer[buffer_index + 1] as u16)) as f32 / 10.0;
    
    // 2 Bytes: Current (in mA) = byte(2) * 256 + byte(3)
    let current: f32 = ((buffer[buffer_index + 2] as u16) * 256 + (buffer[buffer_index + 3] as u16)) as f32 / 100.0;

    // 1 Byte: PowerFactor (in procent) = byte(4)
    let power_factor = buffer[buffer_index + 4] as f32 / 100.0;
    
    data_samples.push(DataSample {
      timestamp: start_timestamp.to_string(),
      voltage: voltage,
      current: current,
      power_factor: power_factor,
    });

    start_timestamp += Duration::seconds(60);
    buffer_index += 5;
  }

  let result = Data {
    timestamp: timestamp.to_string(),
    data_samples: data_samples,
  };

  Ok(result)
}

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let mut opts = Options::new();
  opts.optopt("f", "file", "Read file", "NAME");
  opts.optopt("d", "directory", "Read files from directory", "NAME");
  opts.optflag("h", "help", "Print this help menu");
  opts.optflag("v", "version", "Output version information and exit");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!("{}", f.to_string()) }
  };

  if matches.opt_present("h") {
    print_usage(opts);
    return Ok(());
  }

  if matches.opt_present("v") {
    print_version();
    return Ok(());
  }

  if let Some(load_file) = matches.opt_str("f") {
    if load_file.is_empty() {
      eprintln!("[Error] Option -f or --file has no parameter");
      return Ok(());
    }

    let mut file = fs::File::open(&load_file).unwrap();
    let metadata = file.metadata().unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    if metadata.len() == 102 {
      if buffer.len() >= 5 {
        if &buffer[0..5] == b"INFO:" {
          match read_info_file(&buffer) {
            Ok(data) => {
              print_info_file(&data);
            }
            Err(err) => {
              eprintln!("[Error] {}", err);
            }
          }
        }
      }
    } else if metadata.len() != 102 {
      if buffer.len() >= 3 {
        if &buffer[0..3] == [0xE0, 0xC5, 0xEA] {
          match read_data_file(&buffer) {
            Ok(data) => {
              print_data_file(&data);
            }
            Err(err) => {
              eprintln!("[Error] {}", err);
            }
          }
        }
      }
    }
  }

  if let Some(load_directory) = matches.opt_str("d") {
    if load_directory.is_empty() {
      eprintln!("[Error] Option -d or --directory has no parameter");
      return Ok(());
    }

    let entries = fs::read_dir(load_directory).unwrap();

    for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_file() {
        if let Some(filename) = path.file_name() {
          if let Some(filename_str) = filename.to_str() {
            if filename_str.len() == 12 && filename_str.ends_with(".BIN") {
              let _filename_prefix = &filename_str[..8];
              let mut file = fs::File::open(&path).unwrap();
              let metadata = file.metadata().unwrap();
        
              let mut buffer = Vec::new();
              file.read_to_end(&mut buffer).unwrap();

              if metadata.len() == 102 {
                if buffer.len() >= 5 {
                  if &buffer[0..5] == b"INFO:" {
                    match read_info_file(&buffer) {
                      Ok(data) => {
                        print_info_file(&data);
                      }
                      Err(err) => {
                        eprintln!("[Error] {}", err);
                      }
                    }
                  }
                }
              } else if metadata.len() != 102 {
                if buffer.len() >= 3 {
                  if &buffer[0..3] == [0xE0, 0xC5, 0xEA] {
                    match read_data_file(&buffer) {
                      Ok(data) => {
                        print_data_file(&data);
                      }
                      Err(err) => {
                        eprintln!("[Error] {}", err);
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  Ok(())
}