// cmk
// use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
// use lib::shared_const::Schedule;

// cmk
// // Although defined as a `static` (a form of global variable):
// // i) scope is restricted to prevent use as a  global.  This way, `SIGNAL` must be injected like any
// //    other variable, providing loose-coupling and improved testability.
// // `SIGNAL` is synchronized (does not violate Rust's "shared XOR mutable" borrow-checker rule)
// // *but* as a global, does not support re-entrancy (e.g. re-entrant testing)
// pub static SIGNAL0: Signal<CriticalSectionRawMutex, Schedule> = Signal::new();
// pub static SIGNAL1: Signal<CriticalSectionRawMutex, Schedule> = Signal::new();
