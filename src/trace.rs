use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::thread::JoinHandle;

use crate::bindings::{
    CloseTrace, ERROR_SUCCESS, EVENT_RECORD, EVENT_TRACE_LOGFILEW, EVENT_TRACE_LOGFILEW_0, EVENT_TRACE_LOGFILEW_1, OpenTraceW,
    PROCESS_TRACE_MODE_EVENT_RECORD, PROCESS_TRACE_MODE_RAW_TIMESTAMP, PROCESS_TRACE_MODE_REAL_TIME,
    PROCESSTRACE_HANDLE, ProcessTrace,
};
use windows_core::PWSTR;

use crate::error::{Error, Result};
use crate::event::Event;
use crate::schema::Schemas;

/// What an event's timestamp counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timestamps {
    /// 100 ns ticks since 1601, as FILETIME.
    SystemTime,
    /// The session's own clock, unconverted: QPC ticks for the sessions this
    /// crate starts.
    Raw,
}

/// A real-time session's events, handed one at a time to a callback on a
/// thread of its own until dropped.
pub struct Trace {
    handle: u64,
    clock: Clock,
    pump: Option<JoinHandle<()>>,
}

impl Trace {
    pub fn real_time<F>(session: &str, timestamps: Timestamps, on_event: F) -> Result<Self>
    where
        F: FnMut(&Event<'_>) + Send + 'static,
    {
        let mut pump = Box::new(Pump::new(on_event));
        let mut name: Vec<u16> = session.encode_utf16().chain(Some(0)).collect();
        let mut logfile = EVENT_TRACE_LOGFILEW {
            LoggerName: PWSTR(name.as_mut_ptr()),
            Context: (&mut *pump as *mut Pump<F>).cast(),
            Anonymous: EVENT_TRACE_LOGFILEW_0 {
                ProcessTraceMode: mode(timestamps) | PROCESS_TRACE_MODE_REAL_TIME as u32,
            },
            Anonymous2: EVENT_TRACE_LOGFILEW_1 {
                EventRecordCallback: Some(dispatch::<F>),
            },
            ..Default::default()
        };
        let handle = open(&mut logfile)?;
        let clock = Clock::of(&logfile);
        let pump = std::thread::Builder::new()
            .name("etw-pump".into())
            .spawn(move || {
                let _ = unsafe { ProcessTrace(&[PROCESSTRACE_HANDLE(handle.0)], None, None) };
                drop(pump);
            })
            .map_err(|error| {
                unsafe { CloseTrace(handle) };
                Error::new("CreateThread", error.raw_os_error().unwrap_or(0) as u32)
            })?;
        Ok(Self {
            handle: handle.0,
            clock,
            pump: Some(pump),
        })
    }

    /// The clock the session's raw timestamps are taken with.
    pub fn clock(&self) -> Clock {
        self.clock
    }

    /// Whether events still come: false once ETW ended the trace, as when
    /// its session was stopped.
    pub fn pumping(&self) -> bool {
        self.pump.as_ref().is_some_and(|pump| !pump.is_finished())
    }
}

/// What a session's raw timestamps count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockKind {
    Qpc,
    SystemTime,
    CpuCycles,
    Unknown(u32),
}

/// The clock a trace's raw timestamps were taken with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    pub kind: ClockKind,
    /// Ticks per second, for QPC.
    pub frequency: i64,
}

/// Hands the events a session wrote to `file` to `on_event`, in the order
/// they were written, and returns once all are read.
pub fn read_file<F>(file: &Path, timestamps: Timestamps, on_event: F) -> Result<Clock>
where
    F: FnMut(&Event<'_>),
{
    let mut pump = Pump::new(on_event);
    let mut path: Vec<u16> = file.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut logfile = EVENT_TRACE_LOGFILEW {
        LogFileName: PWSTR(path.as_mut_ptr()),
        Context: (&mut pump as *mut Pump<F>).cast(),
        Anonymous: EVENT_TRACE_LOGFILEW_0 {
            ProcessTraceMode: mode(timestamps),
        },
        Anonymous2: EVENT_TRACE_LOGFILEW_1 {
            EventRecordCallback: Some(dispatch::<F>),
        },
        ..Default::default()
    };
    let handle = open(&mut logfile)?;
    let clock = Clock::of(&logfile);
    let code = unsafe { ProcessTrace(&[handle], None, None) };
    unsafe { CloseTrace(handle) };
    if code != ERROR_SUCCESS as u32 {
        return Err(Error::new("ProcessTrace", code));
    }
    Ok(clock)
}

impl Clock {
    fn of(logfile: &EVENT_TRACE_LOGFILEW) -> Self {
        let header = &logfile.LogfileHeader;
        Self {
            kind: match header.ReservedFlags {
                1 => ClockKind::Qpc,
                2 => ClockKind::SystemTime,
                3 => ClockKind::CpuCycles,
                other => ClockKind::Unknown(other),
            },
            frequency: header.PerfFreq,
        }
    }
}

fn mode(timestamps: Timestamps) -> u32 {
    let mut mode = PROCESS_TRACE_MODE_EVENT_RECORD as u32;
    if timestamps == Timestamps::Raw {
        mode |= PROCESS_TRACE_MODE_RAW_TIMESTAMP as u32;
    }
    mode
}

fn open(logfile: &mut EVENT_TRACE_LOGFILEW) -> Result<PROCESSTRACE_HANDLE> {
    let handle = unsafe { OpenTraceW(logfile) };
    if handle.0 == u64::MAX {
        let code = std::io::Error::last_os_error().raw_os_error().unwrap_or(0) as u32;
        return Err(Error::new("OpenTraceW", code));
    }
    Ok(handle)
}

impl Drop for Trace {
    fn drop(&mut self) {
        unsafe { CloseTrace(PROCESSTRACE_HANDLE(self.handle)) };
        if let Some(pump) = self.pump.take() {
            let _ = pump.join();
        }
    }
}

/// What one trace's callbacks share: the caller's callback and the schemas
/// of the kinds of events seen so far. ProcessTrace calls back on one
/// thread, so nothing in it is locked.
struct Pump<F> {
    on_event: F,
    schemas: Schemas,
}

impl<F> Pump<F> {
    fn new(on_event: F) -> Self {
        Self {
            on_event,
            schemas: Schemas::default(),
        }
    }
}

unsafe extern "system" fn dispatch<F: FnMut(&Event<'_>)>(record: *mut EVENT_RECORD) {
    let Some(record) = (unsafe { record.as_ref() }) else {
        return;
    };
    let Some(pump) = (unsafe { record.UserContext.cast::<Pump<F>>().as_mut() }) else {
        return;
    };
    (pump.on_event)(&Event::new(record, &pump.schemas));
}
