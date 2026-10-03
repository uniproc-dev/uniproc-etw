use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use windows_core::PCWSTR;

use crate::aligned::AlignedBuf;
use crate::bindings::{
    ERROR_INSUFFICIENT_BUFFER, EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL, EVENT_HEADER_EXTENDED_DATA_ITEM, EVENT_RECORD,
    PropertyStruct, TRACE_EVENT_INFO, TdhGetEventInformation,
};

/// What TDH tells of a kind of event: its names and its top-level fields'
/// in-types.
pub(crate) struct Schema {
    task: Option<String>,
    opcode: Option<String>,
    event: Option<String>,
    fields: Vec<(String, u16)>,
}

impl Schema {
    pub fn of(record: &EVENT_RECORD) -> Option<Self> {
        let mut size = 0u32;
        let asked = unsafe { TdhGetEventInformation(record, None, None, &mut size) };
        if asked.0 != ERROR_INSUFFICIENT_BUFFER as u32 {
            return None;
        }
        let mut buf = AlignedBuf::zeroed(size as usize);
        let info = buf.as_mut_ptr().cast::<TRACE_EVENT_INFO>();
        if unsafe { TdhGetEventInformation(record, None, Some(info), &mut size) }.0 != 0 {
            return None;
        }
        let info = unsafe { &*info };
        let base = buf.as_ptr();
        let name = |offset: u32| {
            if offset == 0 {
                return None;
            }
            let text = unsafe { PCWSTR(base.add(offset as usize).cast()).to_string() }.ok()?;
            let text = text.trim();
            (!text.is_empty()).then(|| text.to_string())
        };
        let properties =
            unsafe { std::slice::from_raw_parts(info.EventPropertyInfoArray.as_ptr(), info.TopLevelPropertyCount as usize) };
        let fields = properties
            .iter()
            .filter(|property| property.Flags & PropertyStruct == 0)
            .filter_map(|property| Some((name(property.NameOffset)?, unsafe { property.Anonymous.nonStructType.InType })))
            .collect();
        Some(Self {
            task: name(info.TaskNameOffset),
            opcode: name(info.OpcodeNameOffset),
            event: name(unsafe { info.Anonymous.EventNameOffset }),
            fields,
        })
    }

    pub fn task(&self) -> Option<&str> {
        self.task.as_deref()
    }

    pub fn opcode(&self) -> Option<&str> {
        self.opcode.as_deref()
    }

    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    pub fn in_type(&self, field: &str) -> Option<u16> {
        self.fields.iter().find(|(name, _)| name == field).map(|&(_, in_type)| in_type)
    }
}

type Kind = (u128, u16, u8, u8, u16);
type Known = Option<Arc<Schema>>;
type Carried = HashMap<Box<[u8]>, Known>;

/// The schemas of the kinds of events one trace has seen, so TDH is asked
/// once per kind and not once per event. A manifest event's kind is its
/// descriptor; every TraceLogging event has id 0, so its kind is the schema
/// it carries.
#[derive(Default)]
pub(crate) struct Schemas {
    manifest: RefCell<HashMap<Kind, Known>>,
    tracelogging: RefCell<HashMap<(u128, u8), Carried>>,
}

impl Schemas {
    pub fn get(&self, record: &EVENT_RECORD) -> Option<Arc<Schema>> {
        let header = &record.EventHeader;
        let descriptor = &header.EventDescriptor;
        let provider = header.ProviderId.to_u128();
        if let Some(carried) = tracelogging_schema(record) {
            let mut kinds = self.tracelogging.borrow_mut();
            let kinds = kinds.entry((provider, descriptor.Opcode)).or_default();
            if let Some(known) = kinds.get(carried) {
                return known.clone();
            }
            let schema = Schema::of(record).map(Arc::new);
            kinds.insert(carried.into(), schema.clone());
            return schema;
        }
        let kind = (provider, descriptor.Id, descriptor.Version, descriptor.Opcode, descriptor.Task);
        self.manifest
            .borrow_mut()
            .entry(kind)
            .or_insert_with(|| Schema::of(record).map(Arc::new))
            .clone()
    }
}

fn tracelogging_schema(record: &EVENT_RECORD) -> Option<&[u8]> {
    if record.ExtendedData.is_null() {
        return None;
    }
    let items: &[EVENT_HEADER_EXTENDED_DATA_ITEM] =
        unsafe { std::slice::from_raw_parts(record.ExtendedData, record.ExtendedDataCount as usize) };
    let item = items.iter().find(|item| item.ExtType as i32 == EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL)?;
    Some(unsafe { std::slice::from_raw_parts(item.DataPtr as *const u8, item.DataSize as usize) })
}
