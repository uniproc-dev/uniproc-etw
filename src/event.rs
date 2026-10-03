use windows_core::PWSTR;

use crate::aligned::AlignedBuf;
use crate::bindings::{
    CP_ACP, ERROR_INSUFFICIENT_BUFFER, EVENT_HEADER_EXTENDED_DATA_ITEM, EVENT_HEADER_FLAG_64_BIT_HEADER, EVENT_RECORD,
    MultiByteToWideChar, PROPERTY_DATA_DESCRIPTOR, TDH_INTYPE_COUNTEDANSISTRING, TDH_INTYPE_COUNTEDSTRING,
    TDH_INTYPE_REVERSEDCOUNTEDANSISTRING, TDH_INTYPE_REVERSEDCOUNTEDSTRING, TRACE_EVENT_INFO, TdhGetEventInformation,
    TdhGetProperty, TdhGetPropertySize,
};

const SID_REVISION: u8 = 1;

/// An event as ETW hands it to a callback.
pub struct Event<'a> {
    record: &'a EVENT_RECORD,
}

impl<'a> Event<'a> {
    pub(crate) fn new(record: &'a EVENT_RECORD) -> Self {
        Self { record }
    }

    pub fn provider(&self) -> u128 {
        self.record.EventHeader.ProviderId.to_u128()
    }

    pub fn id(&self) -> u16 {
        self.record.EventHeader.EventDescriptor.Id
    }

    pub fn version(&self) -> u8 {
        self.record.EventHeader.EventDescriptor.Version
    }

    pub fn level(&self) -> u8 {
        self.record.EventHeader.EventDescriptor.Level
    }

    pub fn keywords(&self) -> u64 {
        self.record.EventHeader.EventDescriptor.Keyword
    }

    pub fn opcode(&self) -> u8 {
        self.record.EventHeader.EventDescriptor.Opcode
    }

    pub fn task(&self) -> u16 {
        self.record.EventHeader.EventDescriptor.Task
    }

    pub fn process_id(&self) -> u32 {
        self.record.EventHeader.ProcessId
    }

    pub fn thread_id(&self) -> u32 {
        self.record.EventHeader.ThreadId
    }

    /// The processor the event was written on.
    pub fn processor(&self) -> u8 {
        unsafe { self.record.BufferContext.Anonymous.Anonymous.ProcessorNumber }
    }

    /// In the unit the trace was opened with: see [`crate::Timestamps`].
    pub fn timestamp(&self) -> i64 {
        self.record.EventHeader.TimeStamp
    }

    /// The header's `EVENT_HEADER_FLAG_*`.
    pub fn flags(&self) -> u16 {
        self.record.EventHeader.Flags
    }

    /// Whether the writer was a 64-bit process: its pointers in the payload
    /// are 8 bytes long, 4 otherwise.
    pub fn is_64_bit(&self) -> bool {
        self.flags() as u32 & EVENT_HEADER_FLAG_64_BIT_HEADER as u32 != 0
    }

    /// The payload as written, for events read by a layout of their own.
    pub fn user_data(&self) -> &'a [u8] {
        let data = self.record.UserData as *const u8;
        if data.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(data, self.record.UserDataLength as usize) }
    }

    /// The bytes of the field of this name, as TDH finds it by the event's
    /// manifest or TraceLogging schema.
    pub fn bytes(&self, name: &str) -> Option<Vec<u8>> {
        let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        let descriptor = [PROPERTY_DATA_DESCRIPTOR {
            PropertyName: name.as_ptr() as u64,
            ArrayIndex: u32::MAX,
            Reserved: 0,
        }];
        let mut size = 0u32;
        if unsafe { TdhGetPropertySize(self.record, None, &descriptor, &mut size) }.0 != 0 {
            return None;
        }
        let mut bytes = vec![0u8; size as usize];
        if unsafe { TdhGetProperty(self.record, None, &descriptor, size, bytes.as_mut_ptr()) }.0 != 0 {
            return None;
        }
        Some(bytes)
    }

    /// An integer field of up to 8 bytes, zero-extended.
    pub fn number(&self, name: &str) -> Option<u64> {
        let bytes = self.bytes(name)?;
        if bytes.len() > 8 {
            return None;
        }
        let mut le = [0u8; 8];
        le[..bytes.len()].copy_from_slice(&bytes);
        Some(u64::from_le_bytes(le))
    }

    /// A UTF-16 string field, terminated or counted.
    pub fn text(&self, name: &str) -> Option<String> {
        let bytes = self.bytes(name)?;
        let units: Vec<u16> = string_bytes(&bytes, self.in_type(name), 2)
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        Some(String::from_utf16_lossy(&units))
    }

    /// An 8-bit string field in the system's code page, terminated or
    /// counted.
    pub fn ansi(&self, name: &str) -> Option<String> {
        let bytes = self.bytes(name)?;
        Some(from_code_page(string_bytes(&bytes, self.in_type(name), 1)))
    }

    /// The TDH in-type of the top-level field of this name.
    fn in_type(&self, name: &str) -> Option<u16> {
        let mut size = 0u32;
        let asked = unsafe { TdhGetEventInformation(self.record, None, None, &mut size) };
        if asked.0 != ERROR_INSUFFICIENT_BUFFER as u32 {
            return None;
        }
        let mut buf = AlignedBuf::zeroed(size as usize);
        let info = buf.as_mut_ptr().cast::<TRACE_EVENT_INFO>();
        if unsafe { TdhGetEventInformation(self.record, None, Some(info), &mut size) }.0 != 0 {
            return None;
        }
        let base = buf.as_ptr();
        let properties = unsafe {
            std::slice::from_raw_parts(
                (*info).EventPropertyInfoArray.as_ptr(),
                (*info).TopLevelPropertyCount as usize,
            )
        };
        properties
            .iter()
            .find(|property| unsafe { windows_core::PCWSTR(base.add(property.NameOffset as usize).cast()).to_string() }
                .is_ok_and(|named| named == name))
            .map(|property| unsafe { property.Anonymous.nonStructType.InType })
    }

    /// A SID field.
    pub fn sid(&self, name: &str) -> Option<Vec<u8>> {
        self.bytes(name).filter(|sid| sid.first() == Some(&SID_REVISION))
    }

    /// The SID of a WBEMSID field, as kernel events write it: a TOKEN_USER
    /// whose SID follows its pointer and attributes.
    pub fn wbem_sid(&self, name: &str) -> Option<Vec<u8>> {
        token_user_sid(&self.bytes(name)?, self.is_64_bit())
    }

    /// A copy that outlives the callback.
    pub fn to_owned(&self) -> OwnedEvent {
        OwnedEvent::copy(self.record)
    }
}

fn from_code_page(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let source = bytes.as_ptr().cast::<i8>();
    let length = bytes.len() as i32;
    let wide = unsafe { MultiByteToWideChar(CP_ACP as u32, 0, source, length, None, 0) };
    if wide <= 0 {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    let mut units = vec![0u16; wide as usize];
    let target = Some(PWSTR(units.as_mut_ptr()));
    let written = unsafe { MultiByteToWideChar(CP_ACP as u32, 0, source, length, target, wide) };
    units.truncate(written.max(0) as usize);
    String::from_utf16_lossy(&units)
}

/// The bytes of a string field without its count or terminator, by the
/// field's TDH in-type; `unit` is the width of one character.
fn string_bytes(bytes: &[u8], in_type: Option<u16>, unit: usize) -> &[u8] {
    let count = |count: [u8; 2]| match in_type.map(i32::from) {
        Some(TDH_INTYPE_COUNTEDSTRING | TDH_INTYPE_COUNTEDANSISTRING) => Some(u16::from_le_bytes(count)),
        Some(TDH_INTYPE_REVERSEDCOUNTEDSTRING | TDH_INTYPE_REVERSEDCOUNTEDANSISTRING) => Some(u16::from_be_bytes(count)),
        _ => None,
    };
    if let [first, second, rest @ ..] = bytes
        && let Some(count) = count([*first, *second])
    {
        return &rest[..(count as usize).min(rest.len())];
    }
    let end = bytes
        .chunks_exact(unit)
        .position(|character| character.iter().all(|&byte| byte == 0))
        .map_or(bytes.len(), |at| at * unit);
    &bytes[..end]
}

fn token_user_sid(bytes: &[u8], is_64_bit: bool) -> Option<Vec<u8>> {
    let pointer = if is_64_bit { 8 } else { 4 };
    bytes
        .get(2 * pointer..)
        .filter(|sid| sid.first() == Some(&SID_REVISION))
        .map(<[u8]>::to_vec)
}

/// An event copied out of the callback, payload and extended data with it,
/// to be read on another thread.
pub struct OwnedEvent {
    record: EVENT_RECORD,
    _data: Vec<u8>,
    _items: Vec<EVENT_HEADER_EXTENDED_DATA_ITEM>,
    _extended: Vec<Vec<u8>>,
}

unsafe impl Send for OwnedEvent {}
unsafe impl Sync for OwnedEvent {}

impl OwnedEvent {
    fn copy(record: &EVENT_RECORD) -> Self {
        let source = Event::new(record);
        let data = source.user_data().to_vec();
        let mut items: Vec<EVENT_HEADER_EXTENDED_DATA_ITEM> = if record.ExtendedData.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(record.ExtendedData, record.ExtendedDataCount as usize) }.to_vec()
        };
        let extended: Vec<Vec<u8>> = items
            .iter()
            .map(|item| unsafe { std::slice::from_raw_parts(item.DataPtr as *const u8, item.DataSize as usize) }.to_vec())
            .collect();
        for (item, bytes) in items.iter_mut().zip(&extended) {
            item.DataPtr = bytes.as_ptr() as u64;
        }
        let mut record = *record;
        record.UserData = data.as_ptr() as *mut _;
        record.ExtendedData = items.as_mut_ptr();
        record.UserContext = std::ptr::null_mut();
        Self {
            record,
            _data: data,
            _items: items,
            _extended: extended,
        }
    }

    pub fn event(&self) -> Event<'_> {
        Event::new(&self.record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SID: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];

    fn token_user(pointer: usize) -> Vec<u8> {
        let mut bytes = vec![0xAA; 2 * pointer];
        bytes.extend_from_slice(&SID);
        bytes
    }

    #[test]
    fn a_token_user_s_sid_follows_two_pointers_of_the_writer_s_width() {
        assert_eq!(token_user_sid(&token_user(8), true).as_deref(), Some(&SID[..]));
        assert_eq!(token_user_sid(&token_user(4), false).as_deref(), Some(&SID[..]));
    }

    #[test]
    fn a_token_user_read_at_the_wrong_width_has_no_sid() {
        assert_eq!(token_user_sid(&token_user(4), true), None);
        assert_eq!(token_user_sid(&[1, 2, 3], false), None);
    }

    #[test]
    fn a_terminated_string_ends_at_its_first_nul_unit() {
        let utf16 = [b'h', 0, b'i', 0, 0, 0, b'x', 0];
        assert_eq!(string_bytes(&utf16, None, 2), &utf16[..4]);
        assert_eq!(string_bytes(b"hi\0x", None, 1), b"hi");
        assert_eq!(string_bytes(b"hi", None, 1), b"hi");
    }

    #[test]
    fn a_counted_string_is_as_long_as_its_count_says() {
        let counted = [4, 0, b'h', 0, b'i', 0, b'x', 0];
        let reversed = [0, 4, b'h', 0, b'i', 0, b'x', 0];
        assert_eq!(string_bytes(&counted, Some(TDH_INTYPE_COUNTEDSTRING as u16), 2), &counted[2..6]);
        assert_eq!(string_bytes(&reversed, Some(TDH_INTYPE_REVERSEDCOUNTEDSTRING as u16), 2), &reversed[2..6]);
        assert_eq!(string_bytes(&[2, 0, b'h', b'i'], Some(TDH_INTYPE_COUNTEDANSISTRING as u16), 1), b"hi");
        assert_eq!(string_bytes(&[9, 0, b'h'], Some(TDH_INTYPE_COUNTEDANSISTRING as u16), 1), b"h");
    }

    #[test]
    fn an_ansi_byte_outside_ascii_is_read_in_the_code_page_not_replaced() {
        let read = from_code_page(b"caf\xe9");
        assert!(read.starts_with("caf"), "{read:?}");
        assert_eq!(read.chars().count(), 4, "{read:?}");
        assert!(!read.contains('\u{FFFD}'), "{read:?}");
    }
}
