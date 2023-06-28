// extern crate byte_unit;

use byte_unit;

use log::LevelFilter;
// use log::SetLoggerError;

// use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
};
use log4rs::append::rolling_file::RollingFileAppender;

use log4rs::encode::pattern::PatternEncoder;

use log4rs::filter::threshold::ThresholdFilter;

use log4rs::config::{Appender, Root};
// use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use log4rs::Handle;

use core::fmt::Error;


use whoami;

// pub fn init()  {
    pub fn init() -> Result<Handle, Box<dyn std::error::Error>> {
    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

    let trigger_size = byte_unit::n_mb_bytes!(30) as u64;
    let trigger = Box::new(SizeTrigger::new(trigger_size));
    let roller_pattern = format!("/home/{}/.pod-kast/logs/pod-kast_{}.gz", whoami::username(), "{}");
    

    let roller_count = 5;
    let roller_base = 1;

    match FixedWindowRoller::builder().base(roller_base).build(&roller_pattern, roller_count) {
        Ok(roller) =>{
            let compound_policy = Box::new(CompoundPolicy::new(trigger, Box::new(roller)));
            match RollingFileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
                // .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
                .build(format!("/home/{}/.pod-kast/logs/pod-kast.log",whoami::username()), compound_policy) {
                    Ok(log_appender) =>{
                        match Config::builder().appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Debug))).build("logfile", Box::new(log_appender))).build(Root::builder().appender("logfile").build(LevelFilter::Debug)) {
                                Ok(config) =>{
                                    match log4rs::init_config(config) {
                                        Ok(handle) =>{
                                            // println!("{:?}", handle);
                                            return Ok(handle)
                                        },
                                        Err(e) => {
                                            println!("{}", e);
                                            return Err(Box::new(e))
                                        }
                                    }

                                },
                                Err(e) => {
                                    println!("{}", e);
                                    return Err(Box::new(e))
                                }
                            }
                    },
                    Err(e) => {
                        println!("{}", e);
                        return Err(Box::new(e))
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
                return Err(Box::new(Error))
            }
        }
    }
