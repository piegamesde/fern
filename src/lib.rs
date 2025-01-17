// #![deny(missing_docs)] // TODO add back in
#![doc(html_root_url = "https://docs.rs/fern/0.6.0")]
//! Efficient, configurable logging in Rust.
//!
//! # Depending on fern
//!
//! Ensure you require both fern and log in your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! log = "0.4"
//! fern = "0.5"
//! ```
//!
//! # Basic design
//!
//! [`Dispatch`] is a fancy multiplexer to build a tree of sub-loggers:
//!
//! ```text
//! Dispatch
//! ├── Dispatch
//! │   └── Logger
//! ├── Logger
//! └── Logger
//! ```
//!
//! Each dispatch may control its sub-tree of child loggers by:
//!
//! - Formatting the messages
//! - Setting per module log levels
//! - Arbitrary message filtering
//!
//! # Example setup
//!
//! With fern, all logger configuration is done via builder-like methods on
//! instances of the [`Dispatch`] structure.
//!
//! Here's an example logger which formats messages, and sends everything Debug
//! and above to both stdout and an output.log file:
//!
//! ```no_run
//! use log::{debug, error, info, trace, warn};
//!
//! fn setup_logger() -> Result<(), fern::InitError> {
//!     fern::Dispatch::new()
//!         .format(|out, message, record| {
//!             out.finish(format_args!(
//!                 "{}[{}][{}] {}",
//!                 chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
//!                 record.target(),
//!                 record.level(),
//!                 message
//!             ))
//!         })
//!         .level(log::LevelFilter::Debug)
//!         .chain(std::io::stdout())
//!         .chain(fern::log_file("output.log")?)
//!         .apply()?;
//!     Ok(())
//! }
//! # fn main() {
//! #     setup_logger().expect("failed to set up logger")
//! # }
//! ```
//!
//! Let's unwrap this:
//!
//! ---
//!
//! [`fern::Dispatch::new()`]
//!
//! Create an empty configuration.
//!
//! ---
//!
//! [`.format(|...| ...)`]
//!
//! Add a formatter to the logger, modifying all messages sent through.
//!
//! ___
//!
//! [`chrono::Local::now()`]
//!
//! Get the current time in the local timezone using the [`chrono`] library.
//! See the [time-and-date docs].
//!
//! ___
//!
//! [`.format("[%Y-%m-%d][%H:%M:%S]")`][chrono-format]
//!
//! Use chrono's lazy format specifier to turn the time into a readable string.
//!
//! ---
//!
//! [`out.finish(format_args!(...))`]
//!
//! Call the `fern::FormattingCallback` to submit the formatted message.
//!
//! This roundabout way is slightly odd, but it allows for very fast logging.
//! No string allocation required!
//!
//! [`format_args!()`] has the same format as [`println!()`] \(and every other
//! [`std::fmt`]-based macro).
//!
//! ---
//!
//! [`.level(log::LevelFilter::Debug)`]
//!
//! Set the minimum level needed to output to `Debug`.
//!
//! ---
//!
//! [`.chain(std::io::stdout())`]
//!
//! Add a child to the logger. All messages which pass the filters will be sent
//! to stdout.
//!
//! [`Dispatch::chain`] accepts [`Stdout`], [`Stderr`], [`File`] and other
//! [`Dispatch`] instances.
//!
//! ---
//!
//! [`.chain(fern::log_file(...)?)`]
//!
//! Add a second child sending messages to the file "output.log".
//!
//! See [`fern::log_file()`] for more info on file output.
//!
//! ---
//!
//! [`.apply()`][`.apply`]
//!
//! Consume the configuration and instantiate it as the current runtime global
//! logger.
//!
//! This will fail if and only if `.apply()` or equivalent form another crate
//! has already been used this runtime.
//!
//! Since the binary crate is the only one ever setting up logging, the
//! [`apply`] result can be reasonably unwrapped: it's a bug if any crate is
//! calling this method more than once.
//!
//! ---
//!
//! The final output will look like:
//!
//! ```text
//! [2017-01-20][12:55:04][crate-name][INFO] Hello, world!
//! [2017-01-20][12:56:21][crate-name][WARN] Ahhh!
//! [2017-01-20][12:58:00][crate-name][DEBUG] Something less important happened.
//! ```
//!
//! # Logging
//!
//! Once the logger has been set, it will pick up all logging calls from your
//! crate and all libraries you depend on.
//!
//! ```rust
//! # use log::{debug, error, info, trace, warn};
//!
//! # fn setup_logger() -> Result<(), fern::InitError> {
//! fern::Dispatch::new()
//!     // ...
//!     .apply()?;
//! # Ok(())
//! # }
//!
//! # fn main() {
//! # setup_logger().ok(); // we're ok with this not succeeding.
//! trace!("Trace message");
//! debug!("Debug message");
//! info!("Info message");
//! warn!("Warning message");
//! error!("Error message");
//! # }
//! ```
//!
//! # More
//!
//! The [`Dispatch` documentation] has example usages of each method, and the
//! [full example program] might be useful for using fern in a larger
//! application context.
//!
//! See the [colors] module for examples using ANSI terminal coloring.
//!
//! See the [syslog] module for examples outputting to the unix syslog, or the
//! [syslog full example program] for a more realistic sample.
//!
//! See the [meta] module for information on getting logging-within-logging
//! working correctly.
//!
//! [`fern::Dispatch::new()`]: struct.Dispatch.html#method.new
//! [`.format(|...| ...)`]: struct.Dispatch.html#method.format
//! [`chrono::Local::now()`]: https://docs.rs/chrono/0.4/chrono/offset/local/struct.Local.html#method.now
//! [chrono-format]: https://docs.rs/chrono/0.4/chrono/datetime/struct.DateTime.html#method.format
//! [`out.finish(format_args!(...))`]: struct.FormatCallback.html#method.finish
//! [`.level(log::LevelFilter::Debug)`]: struct.Dispatch.html#method.level
//! [`Dispatch::chain`]: struct.Dispatch.html#method.chain
//! [`.chain(std::io::stdout())`]: struct.Dispatch.html#method.chain
//! [`Stdout`]: https://doc.rust-lang.org/std/io/struct.Stdout.html
//! [`Stderr`]: https://doc.rust-lang.org/std/io/struct.Stderr.html
//! [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
//! [`Dispatch`]: struct.Dispatch.html
//! [`.chain(fern::log_file(...)?)`]: struct.Dispatch.html#method.chain
//! [`fern::log_file()`]: fn.log_file.html
//! [`.apply`]: struct.Dispatch.html#method.apply
//! [`format_args!()`]: https://doc.rust-lang.org/std/macro.format_args.html
//! [`println!()`]: https://doc.rust-lang.org/std/macro.println.html
//! [`std::fmt`]: https://doc.rust-lang.org/std/fmt/
//! [`chrono`]: https://github.com/chronotope/chrono
//! [time-and-date docs]: https://docs.rs/chrono/0.4/chrono/index.html#date-and-time
//! [the format specifier docs]: https://docs.rs/chrono/0.4/chrono/format/strftime/index.html#specifiers
//! [`Dispatch` documentation]: struct.Dispatch.html
//! [full example program]: https://github.com/daboross/fern/tree/master/examples/cmd-program.rs
//! [syslog full example program]: https://github.com/daboross/fern/tree/master/examples/syslog.rs
//! [`apply`]: struct.Dispatch.html#method.apply
//! [colors]: colors/index.html
//! [syslog]: syslog/index.html
//! [meta]: meta/index.html
use std::{
    convert::AsRef,
    fmt,
    io::{self, Write},
    sync::mpsc,
};

pub use crate::{
    dispatch::{Dispatch, Output},
    errors::InitError,
    dispatch::FormatCallback,
};

pub mod formatter;

mod dispatch;
mod errors;

/// Logger implementations for different output targets
// pub mod logger {
    // pub use crate::log_impl::*;
    // pub use crate::*;
// }
pub mod logger;

// For backwards compatibilty
// Also see https://github.com/rust-lang/rust/issues/30827
//#[deprecated(note = "Moved to fern::logger::Panic")]
//pub use crate::logger::Panic;

#[cfg(feature = "colored")]
pub mod colors;
#[cfg(all(not(windows), feature = "syslog-3", feature = "syslog-4"))]
pub mod syslog;

pub mod meta;

/// A type alias for a log formatter.
///
/// As of fern `0.5`, the passed `fmt::Arguments` will always be the same as
/// the given `log::Record`'s `.args()`.
pub type Formatter = dyn Fn(FormatCallback, &fmt::Arguments, &log::Record) + Sync + Send + 'static;

/// A type alias for a log filter. Returning true means the record should
/// succeed - false means it should fail.
pub type Filter = dyn Fn(&log::Metadata) -> bool + Send + Sync + 'static;

#[inline(always)]
fn fallback_on_error<F>(record: &log::Record, log_func: F)
where
    F: FnOnce(&log::Record) -> Result<(), LogError>,
{
    if let Err(error) = log_func(record) {
        backup_logging(record, &error)
    }
}

fn backup_logging(record: &log::Record, error: &LogError) {
    let second = write!(
        io::stderr(),
        "Error performing logging.\
         \n\tattempted to log: {}\
         \n\trecord: {:?}\
         \n\tlogging error: {}",
        record.args(),
        record,
        error
    );

    if let Err(second_error) = second {
        panic!(
            "Error performing stderr logging after error occurred during regular logging.\
             \n\tattempted to log: {}\
             \n\trecord: {:?}\
             \n\tfirst logging error: {}\
             \n\tstderr error: {}",
            record.args(),
            record,
            error,
            second_error,
        );
    }
}

#[derive(Debug)]
enum LogError {
    Io(io::Error),
    Send(mpsc::SendError<String>),
    #[cfg(all(not(windows), feature = "syslog-4"))]
    Syslog4(syslog4::Error),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogError::Io(ref e) => write!(f, "{}", e),
            LogError::Send(ref e) => write!(f, "{}", e),
            #[cfg(all(not(windows), feature = "syslog-4"))]
            LogError::Syslog4(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for LogError {
    fn from(error: io::Error) -> Self {
        LogError::Io(error)
    }
}

impl From<mpsc::SendError<String>> for LogError {
    fn from(error: mpsc::SendError<String>) -> Self {
        LogError::Send(error)
    }
}
