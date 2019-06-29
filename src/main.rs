extern crate chrono;
extern crate date_time;
extern crate serde_derive;
use serde_json::{Result, Value};
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::ffi::OsStr;
use chrono::prelude::*;
use date_time::time_tuple::{Duration, Time, TimeTuple};
use std::mem;
use chrono::Local;
use std::num::*;

fn main() -> std::io::Result<()> { 
    let mut file = File::open("file.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // //-----------------------------------------------------------------------------// 
    let js = contents;
    let v: Value = serde_json::from_str(&js).unwrap();
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));
    let date = Local::now();
    let systemTime: String = date.format("%H:%M").to_string();  
    let timeRn = [&date.format("%H:%M").to_string()];


            Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(v["event"][0].to_string())
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });
      

    Ok(())
    
}