#[inline]
pub unsafe fn AdjustTokenPrivileges(
    tokenhandle: HANDLE,
    disableallprivileges: bool,
    newstate: Option<*const TOKEN_PRIVILEGES>,
    bufferlength: u32,
    previousstate: Option<*mut TOKEN_PRIVILEGES>,
    returnlength: Option<*mut u32>,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AdjustTokenPrivileges(tokenhandle : HANDLE, disableallprivileges : windows_core::BOOL, newstate : *const TOKEN_PRIVILEGES, bufferlength : u32, previousstate : *mut TOKEN_PRIVILEGES, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe {
        AdjustTokenPrivileges(
            tokenhandle,
            disableallprivileges.into(),
            newstate.unwrap_or(core::mem::zeroed()) as _,
            bufferlength,
            previousstate.unwrap_or(core::mem::zeroed()) as _,
            returnlength.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn CloseHandle(hobject: HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[inline]
pub unsafe fn CloseTrace(tracehandle: PROCESSTRACE_HANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn CloseTrace(tracehandle : PROCESSTRACE_HANDLE) -> u32);
    unsafe { CloseTrace(tracehandle) }
}
#[inline]
pub unsafe fn ControlTraceW<P1>(
    traceid: CONTROLTRACE_ID,
    instancename: P1,
    properties: *mut EVENT_TRACE_PROPERTIES,
    controlcode: u32,
) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ControlTraceW(traceid : CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES, controlcode : u32) -> u32);
    unsafe {
        ControlTraceW(
            traceid,
            instancename.param().abi(),
            properties as _,
            controlcode,
        )
    }
}
#[inline]
pub unsafe fn EnableTraceEx2(
    traceid: CONTROLTRACE_ID,
    providerid: *const windows_core::GUID,
    controlcode: u32,
    level: u8,
    matchanykeyword: u64,
    matchallkeyword: u64,
    timeout: u32,
    enableparameters: Option<*const ENABLE_TRACE_PARAMETERS>,
) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EnableTraceEx2(traceid : CONTROLTRACE_ID, providerid : *const windows_core::GUID, controlcode : u32, level : u8, matchanykeyword : u64, matchallkeyword : u64, timeout : u32, enableparameters : *const ENABLE_TRACE_PARAMETERS) -> u32);
    unsafe {
        EnableTraceEx2(
            traceid,
            providerid,
            controlcode,
            level,
            matchanykeyword,
            matchallkeyword,
            timeout,
            enableparameters.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetCurrentProcess() -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcess() -> HANDLE);
    unsafe { GetCurrentProcess() }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn LookupPrivilegeValueW<P0, P1>(
    lpsystemname: P0,
    lpname: P1,
    lpluid: *mut LUID,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeValueW(lpsystemname : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpluid : *mut LUID) -> windows_core::BOOL);
    unsafe {
        LookupPrivilegeValueW(
            lpsystemname.param().abi(),
            lpname.param().abi(),
            lpluid as _,
        )
    }
}
#[inline]
pub unsafe fn MultiByteToWideChar(
    codepage: u32,
    dwflags: u32,
    lpmultibytestr: *const i8,
    cbmultibyte: i32,
    lpwidecharstr: Option<windows_core::PWSTR>,
    cchwidechar: i32,
) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : u32, lpmultibytestr : *const i8, cbmultibyte : i32, lpwidecharstr : windows_core::PWSTR, cchwidechar : i32) -> i32);
    unsafe {
        MultiByteToWideChar(
            codepage,
            dwflags,
            lpmultibytestr,
            cbmultibyte,
            lpwidecharstr.unwrap_or(core::mem::zeroed()) as _,
            cchwidechar,
        )
    }
}
#[inline]
pub unsafe fn OpenProcessToken(
    processhandle: HANDLE,
    desiredaccess: u32,
    tokenhandle: *mut HANDLE,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : HANDLE, desiredaccess : u32, tokenhandle : *mut HANDLE) -> windows_core::BOOL);
    unsafe { OpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[inline]
pub unsafe fn OpenTraceW(logfile: *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE {
    windows_core::link!("advapi32.dll" "system" fn OpenTraceW(logfile : *mut EVENT_TRACE_LOGFILEW) -> PROCESSTRACE_HANDLE);
    unsafe { OpenTraceW(logfile as _) }
}
#[inline]
pub unsafe fn ProcessTrace(
    handlearray: &[PROCESSTRACE_HANDLE],
    starttime: Option<*const FILETIME>,
    endtime: Option<*const FILETIME>,
) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn ProcessTrace(handlearray : *const PROCESSTRACE_HANDLE, handlecount : u32, starttime : *const FILETIME, endtime : *const FILETIME) -> u32);
    unsafe {
        ProcessTrace(
            handlearray.as_ptr(),
            handlearray.len().try_into().unwrap(),
            starttime.unwrap_or(core::mem::zeroed()) as _,
            endtime.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn QueryAllTracesW(
    propertyarray: *mut PEVENT_TRACE_PROPERTIES,
    propertyarraycount: u32,
    loggercount: *mut u32,
) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn QueryAllTracesW(propertyarray : *mut PEVENT_TRACE_PROPERTIES, propertyarraycount : u32, loggercount : *mut u32) -> u32);
    unsafe { QueryAllTracesW(propertyarray as _, propertyarraycount, loggercount as _) }
}
#[inline]
pub unsafe fn StartTraceW<P1>(
    traceid: *mut CONTROLTRACE_ID,
    instancename: P1,
    properties: *mut EVENT_TRACE_PROPERTIES,
) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn StartTraceW(traceid : *mut CONTROLTRACE_ID, instancename : windows_core::PCWSTR, properties : *mut EVENT_TRACE_PROPERTIES) -> u32);
    unsafe { StartTraceW(traceid as _, instancename.param().abi(), properties as _) }
}
#[inline]
pub unsafe fn TdhGetEventInformation(
    event: *const EVENT_RECORD,
    tdhcontext: Option<&[TDH_CONTEXT]>,
    buffer: Option<*mut TRACE_EVENT_INFO>,
    buffersize: *mut u32,
) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetEventInformation(event : *const EVENT_RECORD, tdhcontextcount : u32, tdhcontext : *const TDH_CONTEXT, buffer : *mut TRACE_EVENT_INFO, buffersize : *mut u32) -> TDHSTATUS);
    unsafe {
        TdhGetEventInformation(
            event,
            tdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()),
            tdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            buffer.unwrap_or(core::mem::zeroed()) as _,
            buffersize as _,
        )
    }
}
#[inline]
pub unsafe fn TdhGetProperty(
    pevent: *const EVENT_RECORD,
    ptdhcontext: Option<&[TDH_CONTEXT]>,
    ppropertydata: &[PROPERTY_DATA_DESCRIPTOR],
    buffersize: u32,
    pbuffer: *mut u8,
) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetProperty(pevent : *const EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, buffersize : u32, pbuffer : *mut u8) -> TDHSTATUS);
    unsafe {
        TdhGetProperty(
            pevent,
            ptdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()),
            ptdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            ppropertydata.len().try_into().unwrap(),
            ppropertydata.as_ptr(),
            buffersize,
            pbuffer as _,
        )
    }
}
#[inline]
pub unsafe fn TdhGetPropertySize(
    pevent: *const EVENT_RECORD,
    ptdhcontext: Option<&[TDH_CONTEXT]>,
    ppropertydata: &[PROPERTY_DATA_DESCRIPTOR],
    ppropertysize: *mut u32,
) -> TDHSTATUS {
    windows_core::link!("tdh.dll" "system" fn TdhGetPropertySize(pevent : *const EVENT_RECORD, tdhcontextcount : u32, ptdhcontext : *const TDH_CONTEXT, propertydatacount : u32, ppropertydata : *const PROPERTY_DATA_DESCRIPTOR, ppropertysize : *mut u32) -> TDHSTATUS);
    unsafe {
        TdhGetPropertySize(
            pevent,
            ptdhcontext.map_or(0, |slice| slice.len().try_into().unwrap()),
            ptdhcontext.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            ppropertydata.len().try_into().unwrap(),
            ppropertydata.as_ptr(),
            ppropertysize as _,
        )
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONTROLTRACE_ID(pub u64);
pub const CP_ACP: i32 = 0;
pub type DECODING_SOURCE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENABLE_TRACE_PARAMETERS {
    pub Version: u32,
    pub EnableProperty: u32,
    pub ControlFlags: u32,
    pub SourceId: windows_core::GUID,
    pub EnableFilterDesc: PEVENT_FILTER_DESCRIPTOR,
    pub FilterDescCount: u32,
}
pub const ENABLE_TRACE_PARAMETERS_VERSION_2: i32 = 2;
pub const ERROR_ALREADY_EXISTS: i32 = 183;
pub const ERROR_FILENAME_EXCED_RANGE: i32 = 206;
pub const ERROR_INSUFFICIENT_BUFFER: i32 = 122;
pub const ERROR_MORE_DATA: i32 = 234;
pub const ERROR_NOT_ALL_ASSIGNED: i32 = 1300;
pub const ERROR_SUCCESS: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ETW_BUFFER_CONTEXT {
    pub Anonymous: ETW_BUFFER_CONTEXT_0,
    pub LoggerId: u16,
}
impl Default for ETW_BUFFER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ETW_BUFFER_CONTEXT_0 {
    pub Anonymous: ETW_BUFFER_CONTEXT_0_0,
    pub ProcessorIndex: u16,
}
impl Default for ETW_BUFFER_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ETW_BUFFER_CONTEXT_0_0 {
    pub ProcessorNumber: u8,
    pub Alignment: u8,
}
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: windows_core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: windows_core::GUID,
}
impl Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
impl Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
impl EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub fn Linkage(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Linkage(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved2(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved2(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: i32 = 11;
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_PROPERTY_INFO {
    pub Flags: PROPERTY_FLAGS,
    pub NameOffset: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_0,
    pub Anonymous2: EVENT_PROPERTY_INFO_1,
    pub Anonymous3: EVENT_PROPERTY_INFO_2,
    pub Anonymous4: EVENT_PROPERTY_INFO_3,
}
impl Default for EVENT_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_0 {
    pub nonStructType: EVENT_PROPERTY_INFO_0_0,
    pub structType: EVENT_PROPERTY_INFO_0_1,
    pub customSchemaType: EVENT_PROPERTY_INFO_0_2,
}
impl Default for EVENT_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_1 {
    pub count: u16,
    pub countPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_2 {
    pub length: u16,
    pub lengthPropertyIndex: u16,
}
impl Default for EVENT_PROPERTY_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_PROPERTY_INFO_3 {
    pub Reserved: u32,
    pub Anonymous: EVENT_PROPERTY_INFO_3_0,
}
impl Default for EVENT_PROPERTY_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_3_0 {
    pub _bitfield: u32,
}
impl EVENT_PROPERTY_INFO_3_0 {
    pub fn Tags(&self) -> u32 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_Tags(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !268435455) | (value & 268435455);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_0 {
    pub InType: u16,
    pub OutType: u16,
    pub MapNameOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_1 {
    pub StructStartIndex: u16,
    pub NumOfStructMembers: u16,
    pub padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_PROPERTY_INFO_0_2 {
    pub InType: u16,
    pub OutType: u16,
    pub CustomSchemaOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: PEVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut core::ffi::c_void,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE {
    pub Header: EVENT_TRACE_HEADER,
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: windows_core::GUID,
    pub MofData: *mut core::ffi::c_void,
    pub MofLength: u32,
    pub Anonymous: EVENT_TRACE_0,
}
impl Default for EVENT_TRACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_0 {
    pub ClientContext: u32,
    pub BufferContext: ETW_BUFFER_CONTEXT,
}
impl Default for EVENT_TRACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_CONTROL_QUERY: i32 = 0;
pub const EVENT_TRACE_CONTROL_STOP: i32 = 1;
pub const EVENT_TRACE_CONTROL_UPDATE: i32 = 2;
pub const EVENT_TRACE_FILE_MODE_CIRCULAR: i32 = 2;
pub const EVENT_TRACE_FILE_MODE_SEQUENTIAL: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_HEADER {
    pub Size: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0,
    pub Anonymous2: EVENT_TRACE_HEADER_1,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub Anonymous3: EVENT_TRACE_HEADER_2,
    pub Anonymous4: EVENT_TRACE_HEADER_3,
}
impl Default for EVENT_TRACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_0 {
    pub FieldTypeFlags: u16,
    pub Anonymous: EVENT_TRACE_HEADER_0_0,
}
impl Default for EVENT_TRACE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_0_0 {
    pub HeaderType: u8,
    pub MarkerFlags: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_1 {
    pub Version: u32,
    pub Class: EVENT_TRACE_HEADER_1_0,
}
impl Default for EVENT_TRACE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_1_0 {
    pub Type: u8,
    pub Level: u8,
    pub Version: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_2 {
    pub Guid: windows_core::GUID,
    pub GuidPtr: u64,
}
impl Default for EVENT_TRACE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_HEADER_3 {
    pub Anonymous: EVENT_TRACE_HEADER_3_0,
    pub ProcessorTime: u64,
    pub Anonymous2: EVENT_TRACE_HEADER_3_1,
}
impl Default for EVENT_TRACE_HEADER_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_TRACE_HEADER_3_1 {
    pub ClientContext: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: windows_core::PWSTR,
    pub LoggerName: windows_core::PWSTR,
    pub CurrentTime: i64,
    pub BuffersRead: u32,
    pub Anonymous: EVENT_TRACE_LOGFILEW_0,
    pub CurrentEvent: EVENT_TRACE,
    pub LogfileHeader: TRACE_LOGFILE_HEADER,
    pub BufferCallback: PEVENT_TRACE_BUFFER_CALLBACKW,
    pub BufferSize: u32,
    pub Filled: u32,
    pub EventsLost: u32,
    pub Anonymous2: EVENT_TRACE_LOGFILEW_1,
    pub IsKernelTrace: u32,
    pub Context: *mut core::ffi::c_void,
}
impl Default for EVENT_TRACE_LOGFILEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_0 {
    pub LogFileMode: u32,
    pub ProcessTraceMode: u32,
}
impl Default for EVENT_TRACE_LOGFILEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_LOGFILEW_1 {
    pub EventCallback: PEVENT_CALLBACK,
    pub EventRecordCallback: PEVENT_RECORD_CALLBACK,
}
impl Default for EVENT_TRACE_LOGFILEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_PRIVATE_IN_PROC: i32 = 131072;
pub const EVENT_TRACE_PRIVATE_LOGGER_MODE: i32 = 2048;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_TRACE_PROPERTIES {
    pub Wnode: WNODE_HEADER,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub FlushTimer: u32,
    pub EnableFlags: u32,
    pub Anonymous: EVENT_TRACE_PROPERTIES_0,
    pub NumberOfBuffers: u32,
    pub FreeBuffers: u32,
    pub EventsLost: u32,
    pub BuffersWritten: u32,
    pub LogBuffersLost: u32,
    pub RealTimeBuffersLost: u32,
    pub LoggerThreadId: HANDLE,
    pub LogFileNameOffset: u32,
    pub LoggerNameOffset: u32,
}
impl Default for EVENT_TRACE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_TRACE_PROPERTIES_0 {
    pub AgeLimit: i32,
    pub FlushThreshold: i32,
}
impl Default for EVENT_TRACE_PROPERTIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_TRACE_REAL_TIME_MODE: i32 = 256;
pub const EVENT_TRACE_SYSTEM_LOGGER_MODE: i32 = 33554432;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: u32,
}
pub type PEVENT_CALLBACK = Option<unsafe extern "system" fn(pevent: *mut EVENT_TRACE)>;
pub type PEVENT_FILTER_DESCRIPTOR = *mut EVENT_FILTER_DESCRIPTOR;
pub type PEVENT_HEADER_EXTENDED_DATA_ITEM = *mut EVENT_HEADER_EXTENDED_DATA_ITEM;
pub type PEVENT_RECORD_CALLBACK = Option<unsafe extern "system" fn(eventrecord: *mut EVENT_RECORD)>;
pub type PEVENT_TRACE_BUFFER_CALLBACKW =
    Option<unsafe extern "system" fn(logfile: *mut EVENT_TRACE_LOGFILEW) -> u32>;
pub type PEVENT_TRACE_PROPERTIES = *mut EVENT_TRACE_PROPERTIES;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROCESSTRACE_HANDLE(pub u64);
pub const PROCESS_TRACE_MODE_EVENT_RECORD: i32 = 268435456;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: i32 = 4096;
pub const PROCESS_TRACE_MODE_REAL_TIME: i32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTY_DATA_DESCRIPTOR {
    pub PropertyName: u64,
    pub ArrayIndex: u32,
    pub Reserved: u32,
}
pub type PROPERTY_FLAGS = i32;
pub const PropertyStruct: PROPERTY_FLAGS = 1;
pub const SE_PRIVILEGE_ENABLED: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TDHSTATUS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TDH_CONTEXT {
    pub ParameterValue: u64,
    pub ParameterType: TDH_CONTEXT_TYPE,
    pub ParameterSize: u32,
}
pub type TDH_CONTEXT_TYPE = i32;
pub const TDH_INTYPE_COUNTEDANSISTRING: _TDH_IN_TYPE = 301;
pub const TDH_INTYPE_COUNTEDSTRING: _TDH_IN_TYPE = 300;
pub const TDH_INTYPE_REVERSEDCOUNTEDANSISTRING: _TDH_IN_TYPE = 303;
pub const TDH_INTYPE_REVERSEDCOUNTEDSTRING: _TDH_IN_TYPE = 302;
pub type TEMPLATE_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: SYSTEMTIME,
    pub DaylightBias: i32,
}
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOKEN_ADJUST_PRIVILEGES: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
impl Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOKEN_QUERY: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRACE_EVENT_INFO {
    pub ProviderGuid: windows_core::GUID,
    pub EventGuid: windows_core::GUID,
    pub EventDescriptor: EVENT_DESCRIPTOR,
    pub DecodingSource: DECODING_SOURCE,
    pub ProviderNameOffset: u32,
    pub LevelNameOffset: u32,
    pub ChannelNameOffset: u32,
    pub KeywordsNameOffset: u32,
    pub TaskNameOffset: u32,
    pub OpcodeNameOffset: u32,
    pub EventMessageOffset: u32,
    pub ProviderMessageOffset: u32,
    pub BinaryXMLOffset: u32,
    pub BinaryXMLSize: u32,
    pub Anonymous: TRACE_EVENT_INFO_0,
    pub Anonymous2: TRACE_EVENT_INFO_1,
    pub PropertyCount: u32,
    pub TopLevelPropertyCount: u32,
    pub Anonymous3: TRACE_EVENT_INFO_2,
    pub EventPropertyInfoArray: [EVENT_PROPERTY_INFO; 1],
}
impl Default for TRACE_EVENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_0 {
    pub EventNameOffset: u32,
    pub ActivityIDNameOffset: u32,
}
impl Default for TRACE_EVENT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_1 {
    pub EventAttributesOffset: u32,
    pub RelatedActivityIDNameOffset: u32,
}
impl Default for TRACE_EVENT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_EVENT_INFO_2 {
    pub Flags: TEMPLATE_FLAGS,
    pub Anonymous: TRACE_EVENT_INFO_2_0,
}
impl Default for TRACE_EVENT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_EVENT_INFO_2_0 {
    pub _bitfield: u32,
}
impl TRACE_EVENT_INFO_2_0 {
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Tags(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Tags(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRACE_LOGFILE_HEADER {
    pub BufferSize: u32,
    pub Anonymous: TRACE_LOGFILE_HEADER_0,
    pub ProviderVersion: u32,
    pub NumberOfProcessors: u32,
    pub EndTime: i64,
    pub TimerResolution: u32,
    pub MaximumFileSize: u32,
    pub LogFileMode: u32,
    pub BuffersWritten: u32,
    pub Anonymous2: TRACE_LOGFILE_HEADER_1,
    pub LoggerName: windows_core::PWSTR,
    pub LogFileName: windows_core::PWSTR,
    pub TimeZone: TIME_ZONE_INFORMATION,
    pub BootTime: i64,
    pub PerfFreq: i64,
    pub StartTime: i64,
    pub ReservedFlags: u32,
    pub BuffersLost: u32,
}
impl Default for TRACE_LOGFILE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_0 {
    pub Version: u32,
    pub VersionDetail: TRACE_LOGFILE_HEADER_0_0,
}
impl Default for TRACE_LOGFILE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_0_0 {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub SubVersion: u8,
    pub SubMinorVersion: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TRACE_LOGFILE_HEADER_1 {
    pub LogInstanceGuid: windows_core::GUID,
    pub Anonymous: TRACE_LOGFILE_HEADER_1_0,
}
impl Default for TRACE_LOGFILE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRACE_LOGFILE_HEADER_1_0 {
    pub StartBuffers: u32,
    pub PointerSize: u32,
    pub EventsLost: u32,
    pub CpuSpeedInMHz: u32,
}
pub const WNODE_FLAG_TRACED_GUID: i32 = 131072;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: windows_core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
impl Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
impl Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: HANDLE,
    pub TimeStamp: i64,
}
impl Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type _TDH_IN_TYPE = i32;
