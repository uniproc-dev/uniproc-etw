use std::mem::size_of;
use std::path::PathBuf;

use crate::bindings::{
    CONTROLTRACE_ID, ControlTraceW, ENABLE_TRACE_PARAMETERS, ENABLE_TRACE_PARAMETERS_VERSION_2,
    ERROR_ALREADY_EXISTS, ERROR_FILENAME_EXCED_RANGE, ERROR_MORE_DATA, ERROR_SUCCESS, EVENT_CONTROL_CODE_ENABLE_PROVIDER,
    EVENT_TRACE_CONTROL_QUERY, EVENT_TRACE_CONTROL_STOP, EVENT_TRACE_CONTROL_UPDATE, EVENT_TRACE_FILE_MODE_CIRCULAR,
    EVENT_TRACE_FILE_MODE_SEQUENTIAL,
    EVENT_TRACE_PRIVATE_IN_PROC, EVENT_TRACE_PRIVATE_LOGGER_MODE, EVENT_TRACE_PROPERTIES,
    EVENT_TRACE_REAL_TIME_MODE, EVENT_TRACE_SYSTEM_LOGGER_MODE, EnableTraceEx2, QueryAllTracesW,
    StartTraceW, WNODE_FLAG_TRACED_GUID,
};
use windows_core::{GUID, PCWSTR};

use crate::aligned::AlignedBuf;
use crate::error::{Error, Result};
use crate::privilege;

const EVENT_TRACE_USE_MS_FLUSH_TIMER: u32 = 0x10;
const QPC: u32 = 1;
const NAME_BYTES: usize = 1024;
const FILE_BYTES: usize = 2048;

/// What a provider is asked to write: its events up to `level` that carry
/// any of `keywords`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enable {
    pub keywords: u64,
    pub level: u8,
}

impl Enable {
    pub const ALL: Enable = Enable {
        keywords: u64::MAX,
        level: 5,
    };
}

/// How a session is started.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// The kernel's `EVENT_TRACE_FLAG_*` it collects; a system logger only.
    pub kernel_flags: u32,
    pub system_logger: bool,
    /// Seen by this process only, and started without administrator rights.
    pub private_in_proc: bool,
    pub buffer_kb: u32,
    pub minimum_buffers: u32,
    pub maximum_buffers: u32,
    /// How long ETW holds a buffer that is not full yet.
    pub flush_timer_ms: u32,
    /// Where the events go instead of to real-time consumers.
    pub file: Option<LogFile>,
}

/// A file a session writes its events to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogFile {
    pub path: PathBuf,
    /// Overwritten from its start once it holds this many megabytes; grown
    /// without end when None.
    pub ring_mb: Option<u32>,
}

impl Options {
    pub fn real_time() -> Self {
        Self {
            kernel_flags: 0,
            system_logger: false,
            private_in_proc: false,
            buffer_kb: 16,
            minimum_buffers: 1,
            maximum_buffers: 8,
            flush_timer_ms: 50,
            file: None,
        }
    }

    pub fn system_logger(kernel_flags: u32) -> Self {
        Self {
            kernel_flags,
            system_logger: true,
            ..Self::real_time()
        }
    }

    /// A session of this process's own providers, written to a ring of
    /// `ring_mb` in `file`: ETW refuses a private session in real time.
    pub fn private_in_proc(file: impl Into<PathBuf>, ring_mb: u32) -> Self {
        Self {
            private_in_proc: true,
            file: Some(LogFile {
                path: file.into(),
                ring_mb: Some(ring_mb),
            }),
            ..Self::real_time()
        }
    }

    fn log_file_mode(&self) -> u32 {
        let mut mode = EVENT_TRACE_USE_MS_FLUSH_TIMER;
        mode |= match &self.file {
            None => EVENT_TRACE_REAL_TIME_MODE as u32,
            Some(LogFile { ring_mb: Some(_), .. }) => EVENT_TRACE_FILE_MODE_CIRCULAR as u32,
            Some(LogFile { ring_mb: None, .. }) => EVENT_TRACE_FILE_MODE_SEQUENTIAL as u32,
        };
        if self.system_logger {
            mode |= EVENT_TRACE_SYSTEM_LOGGER_MODE as u32;
        }
        if self.private_in_proc {
            mode |= (EVENT_TRACE_PRIVATE_LOGGER_MODE | EVENT_TRACE_PRIVATE_IN_PROC) as u32;
        }
        mode
    }
}

/// What ETW says about a session.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counters {
    pub flush_timer_ms: u32,
    pub kernel_flags: u32,
    pub events_lost: u32,
    pub realtime_buffers_lost: u32,
    pub log_buffers_lost: u32,
    pub buffers_written: u32,
    pub buffers: u32,
    pub free_buffers: u32,
}

/// A trace session, stopped when dropped.
pub struct Session {
    name: String,
    id: CONTROLTRACE_ID,
    private: bool,
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    /// Starts a session of this name; one left behind under it is stopped
    /// first. A system logger turns on the process's
    /// SeSystemProfilePrivilege, which ETW asks of it.
    pub fn start(name: &str, options: &Options) -> Result<Self> {
        if options.system_logger {
            privilege::enable("SeSystemProfilePrivilege")?;
        }
        let wide = wide(name);
        let mut started = start(&wide, options);
        if started == Err(ERROR_ALREADY_EXISTS as u32) {
            stop(name);
            started = start(&wide, options);
        }
        match started {
            Ok(id) => Ok(Self {
                name: name.to_string(),
                id,
                private: options.private_in_proc,
            }),
            Err(code) => Err(Error::new("StartTraceW", code)),
        }
    }

    pub fn enable(&self, provider: u128, enable: Enable) -> Result<()> {
        let params = ENABLE_TRACE_PARAMETERS {
            Version: ENABLE_TRACE_PARAMETERS_VERSION_2 as u32,
            ..Default::default()
        };
        let provider = GUID::from_u128(provider);
        let code = unsafe {
            EnableTraceEx2(
                self.id,
                &provider,
                EVENT_CONTROL_CODE_ENABLE_PROVIDER as u32,
                enable.level,
                enable.keywords,
                0,
                0,
                Some(&params),
            )
        };
        if code != ERROR_SUCCESS as u32 {
            return Err(Error::new("EnableTraceEx2", code));
        }
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whom ETW is asked about this session: its name, since a logger id ETW
    /// freed may since name someone else's session; a private session by
    /// its id, as ETW does not find it by name.
    fn addressed(&self) -> CONTROLTRACE_ID {
        if self.private { self.id } else { CONTROLTRACE_ID::default() }
    }

    /// What ETW says about this session now; None when ETW no longer has it.
    pub fn query(&self) -> Option<Counters> {
        let mut props = control(self.addressed(), &self.name, EVENT_TRACE_CONTROL_QUERY as u32, None).ok()?;
        let props = unsafe { &*(props.as_mut_ptr() as *const EVENT_TRACE_PROPERTIES) };
        Some(Counters {
            flush_timer_ms: props.FlushTimer,
            kernel_flags: props.EnableFlags,
            events_lost: props.EventsLost,
            realtime_buffers_lost: props.RealTimeBuffersLost,
            log_buffers_lost: props.LogBuffersLost,
            buffers_written: props.BuffersWritten,
            buffers: props.NumberOfBuffers,
            free_buffers: props.FreeBuffers,
        })
    }

    /// Sets how many milliseconds ETW holds a buffer that is not full yet.
    /// Everything else the session was started with stays.
    pub fn set_flush_timer(&self, ms: u32) -> Result<()> {
        let mut current = control(self.addressed(), &self.name, EVENT_TRACE_CONTROL_QUERY as u32, None)
            .map_err(|code| Error::new("ControlTraceW(QUERY)", code))?;
        let props = unsafe { &mut *(current.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES) };
        props.FlushTimer = ms;
        control(self.addressed(), &self.name, EVENT_TRACE_CONTROL_UPDATE as u32, Some(current))
            .map_err(|code| Error::new("ControlTraceW(UPDATE)", code))?;
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = control(self.addressed(), &self.name, EVENT_TRACE_CONTROL_STOP as u32, None);
    }
}

/// Stops the session of this name; true when ETW had one.
pub fn stop(name: &str) -> bool {
    control(CONTROLTRACE_ID::default(), name, EVENT_TRACE_CONTROL_STOP as u32, None).is_ok()
}

/// The names of the sessions running now, as many as ETW lists.
pub fn running() -> Vec<String> {
    const MOST: usize = 64;
    let stride = size_of::<EVENT_TRACE_PROPERTIES>() + NAME_BYTES + FILE_BYTES;
    let mut buf = AlignedBuf::zeroed(stride * MOST);
    let base = buf.as_mut_ptr();
    let mut all: Vec<*mut EVENT_TRACE_PROPERTIES> = (0..MOST)
        .map(|i| unsafe {
            let props = base.add(i * stride).cast::<EVENT_TRACE_PROPERTIES>();
            lay_out(&mut *props, stride);
            props
        })
        .collect();
    let mut listed = 0u32;
    let code = unsafe { QueryAllTracesW(all.as_mut_ptr(), MOST as u32, &mut listed) };
    if code != ERROR_SUCCESS as u32 && code != ERROR_MORE_DATA as u32 {
        return Vec::new();
    }
    all[..(listed as usize).min(MOST)]
        .iter()
        .filter_map(|&props| unsafe {
            let name = props.cast::<u8>().add((*props).LoggerNameOffset as usize).cast::<u16>();
            PCWSTR(name).to_string().ok()
        })
        .collect()
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

fn blank() -> AlignedBuf {
    let mut buf = AlignedBuf::zeroed(size_of::<EVENT_TRACE_PROPERTIES>() + NAME_BYTES + FILE_BYTES);
    let size = buf.len();
    lay_out(unsafe { &mut *(buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES) }, size);
    buf
}

fn lay_out(props: &mut EVENT_TRACE_PROPERTIES, size: usize) {
    props.Wnode.BufferSize = size as u32;
    props.LoggerNameOffset = size_of::<EVENT_TRACE_PROPERTIES>() as u32;
    props.LogFileNameOffset = (size_of::<EVENT_TRACE_PROPERTIES>() + NAME_BYTES) as u32;
}

fn start(name: &[u16], options: &Options) -> std::result::Result<CONTROLTRACE_ID, u32> {
    let mut buf = blank();
    if let Some(file) = &options.file {
        let path = wide(&file.path.to_string_lossy());
        if path.len() * 2 > FILE_BYTES {
            return Err(ERROR_FILENAME_EXCED_RANGE as u32);
        }
        unsafe {
            let at = buf.as_mut_ptr().add(size_of::<EVENT_TRACE_PROPERTIES>() + NAME_BYTES).cast::<u16>();
            std::ptr::copy_nonoverlapping(path.as_ptr(), at, path.len());
        }
    }
    let props = unsafe { &mut *(buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES) };
    props.Wnode.Flags = WNODE_FLAG_TRACED_GUID as u32;
    props.Wnode.ClientContext = QPC;
    if options.private_in_proc {
        props.Wnode.Guid = GUID::new().map_err(|error| error.code().0 as u32)?;
    }
    props.LogFileMode = options.log_file_mode();
    props.BufferSize = options.buffer_kb;
    props.MinimumBuffers = options.minimum_buffers;
    props.MaximumBuffers = options.maximum_buffers;
    props.FlushTimer = options.flush_timer_ms;
    props.EnableFlags = options.kernel_flags;
    props.MaximumFileSize = options.file.as_ref().and_then(|file| file.ring_mb).unwrap_or(0);
    let mut id = CONTROLTRACE_ID::default();
    let code = unsafe { StartTraceW(&mut id, PCWSTR(name.as_ptr()), props) };
    if code != ERROR_SUCCESS as u32 {
        return Err(code);
    }
    Ok(id)
}

/// Runs `code` on the session of this name with `props`, or with blank
/// ones, and answers what ETW wrote back.
fn control(
    id: CONTROLTRACE_ID,
    name: &str,
    code: u32,
    props: Option<AlignedBuf>,
) -> std::result::Result<AlignedBuf, u32> {
    let mut buf = props.unwrap_or_else(blank);
    let props = unsafe { &mut *(buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES) };
    let name = wide(name);
    let status = unsafe { ControlTraceW(id, PCWSTR(name.as_ptr()), props, code) };
    if status != ERROR_SUCCESS as u32 {
        return Err(status);
    }
    Ok(buf)
}
