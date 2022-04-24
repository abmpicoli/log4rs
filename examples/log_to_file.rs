use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,

};
use std::fs;
use std::path::Path;

fn main() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = "/tmp/foo.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");

    // abmpicoli: this is a check for an antipattern: initialization overrides can cause unpredictable
    // log shifting, and logs all over unpredictable places. So this can never happen.

    // the this_config_shouldnt_be_read.yml creates a log file at /tmp/log4rs_EXAMPLE_LOG_TO_FILE_OH_OH.log.
    // if this initialization is fine, the file should not exist.

    // making sure the file really doesn't exists before initialization.
    let file_created_inside_the_config_that_shoudlnt_be_read = Path::new("/tmp/log4rs_EXAMPLE_LOG_TO_FILE_OH_OH.log");

    let _remove_file_outcome = fs::remove_file(file_created_inside_the_config_that_shoudlnt_be_read);
    if file_created_inside_the_config_that_shoudlnt_be_read.exists() {
        error!("FAILURE!!! THE FILE REMOVAL DIDNT WORK!");
    }

    let init_file_result = log4rs::init_file("examples/this_config_shouldnt_be_read.yml", Default::default());
    if init_file_result.is_err() {
        info!("SUCCESS!! It should be really an error to reconfigure logs at runtime");
    };
    if file_created_inside_the_config_that_shoudlnt_be_read.exists() {
        error!("FAILURE!! Although init file DIDN'T reconfigure logs, it has tried to do so anyway, creating extraneous log files");
    }


    Ok(())
}
