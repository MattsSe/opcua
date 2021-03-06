use std::io::{Read, Write};

use encoding::*;
use basic_types::*;
use date_time::*;
use variant::Variant;
use generated::StatusCode;
use generated::StatusCode::*;

/// False if the Value is Null.
const HAS_VALUE: u8 = 0x1;
/// False if the StatusCode is GOOD.
const HAS_STATUS: u8 = 0x2;
/// False if the Source Timestamp is DateTime.MinValue.
const HAS_SOURCE_TIMESTAMP: u8 = 0x4;
/// False if the Server Timestamp is DateTime.MinValue.
const HAS_SERVER_TIMESTAMP: u8 = 0x8;
/// False if the Source Picoseconds is 0.
const HAS_SOURCE_PICOSECONDS: u8 = 0x10;
/// False if the Server Picoseconds is 0.
const HAS_SERVER_PICOSECONDS: u8 = 0x20;

/// Data type ID 23
#[derive(Debug, Clone, PartialEq)]
pub struct DataValue {
    /// The value. BaseDataType
    /// Not present if the Value bit in the EncodingMask is False.
    pub value: Option<Variant>,
    /// The status associated with the value.
    /// Not present (set to GOOD) if the StatusCode bit in the EncodingMask is False
    pub status: Option<StatusCode>,
    /// The source timestamp associated with the value.
    /// Not present if the SourceTimestamp bit in the EncodingMask is False.
    pub source_timestamp: Option<DateTime>,
    /// The number of 10 picosecond intervals for the SourceTimestamp.
    /// Not present if the SourcePicoSeconds bit in the EncodingMask is False.
    /// If the source timestamp is missing the picoseconds are ignored.
    pub source_picoseconds: Option<Int16>,
    /// The Server timestamp associated with the value.
    /// Not present if the ServerTimestamp bit in the EncodingMask is False.
    pub server_timestamp: Option<DateTime>,
    /// The number of 10 picosecond intervals for the ServerTimestamp.
    /// Not present if the ServerPicoSeconds bit in the EncodingMask is False.
    /// If the Server timestamp is missing the picoseconds are ignored.
    pub server_picoseconds: Option<Int16>,
}

impl BinaryEncoder<DataValue> for DataValue {
    fn byte_len(&self) -> usize {
        let mut size = 1;
        let encoding_mask = self.encoding_mask();
        if encoding_mask & HAS_VALUE != 0 {
            size += self.value.as_ref().unwrap().byte_len();
        }
        if encoding_mask & HAS_STATUS != 0 {
            size += self.status.as_ref().unwrap().byte_len();
        }
        if encoding_mask & HAS_SOURCE_TIMESTAMP != 0 {
            size += self.source_timestamp.as_ref().unwrap().byte_len();
            if encoding_mask & HAS_SOURCE_PICOSECONDS != 0 {
                size += self.source_picoseconds.as_ref().unwrap().byte_len();
            }
        }
        if encoding_mask & HAS_SERVER_TIMESTAMP != 0 {
            size += self.server_timestamp.as_ref().unwrap().byte_len();
            if encoding_mask & HAS_SERVER_PICOSECONDS != 0 {
                size += self.server_picoseconds.as_ref().unwrap().byte_len();
            }
        }
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;

        let encoding_mask = self.encoding_mask();
        size += encoding_mask.encode(stream)?;

        if encoding_mask & HAS_VALUE != 0 {
            size += self.value.as_ref().unwrap().encode(stream)?;
        }
        if encoding_mask & HAS_STATUS != 0 {
            size += self.status.as_ref().unwrap().encode(stream)?;
        }
        if encoding_mask & HAS_SOURCE_TIMESTAMP != 0 {
            size += self.source_timestamp.as_ref().unwrap().encode(stream)?;
            if encoding_mask & HAS_SOURCE_PICOSECONDS != 0 {
                size += self.source_picoseconds.as_ref().unwrap().encode(stream)?;
            }
        }
        if encoding_mask & HAS_SERVER_TIMESTAMP != 0 {
            size += self.server_timestamp.as_ref().unwrap().encode(stream)?;
            if encoding_mask & HAS_SERVER_PICOSECONDS != 0 {
                size += self.server_picoseconds.as_ref().unwrap().encode(stream)?;
            }
        }
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let encoding_mask = Byte::decode(stream)?;

        // Value
        let value = if encoding_mask & HAS_VALUE != 0 {
            Some(Variant::decode(stream)?)
        } else {
            None
        };

        // Status
        let status = if encoding_mask & HAS_STATUS != 0 {
            Some(StatusCode::decode(stream)?)
        } else {
            None
        };

        // Source timestamp
        let source_timestamp = if encoding_mask & HAS_SOURCE_TIMESTAMP != 0 {
            Some(DateTime::decode(stream)?)
        } else {
            None
        };
        let source_picoseconds = if encoding_mask & HAS_SOURCE_PICOSECONDS != 0 {
            Some(Int16::decode(stream)?)
        } else {
            None
        };
        // Server timestamp
        let server_timestamp = if encoding_mask & HAS_SERVER_TIMESTAMP != 0 {
            Some(DateTime::decode(stream)?)
        } else {
            None
        };
        let server_picoseconds = if encoding_mask & HAS_SERVER_PICOSECONDS != 0 {
            Some(Int16::decode(stream)?)
        } else {
            None
        };
        // Pico second values are discarded if associated timestamp is not supplied
        Ok(DataValue {
            value: value,
            status: status,
            source_picoseconds: if source_timestamp.is_some() { source_picoseconds } else { None },
            source_timestamp: source_timestamp,
            server_picoseconds: if server_timestamp.is_some() { server_picoseconds } else { None },
            server_timestamp: server_timestamp,
        })
    }
}

impl DataValue {
    pub fn new(value: Variant) -> DataValue {
        let now = DateTime::now();
        DataValue {
            value: Some(value),
            status: Some(GOOD),
            source_timestamp: Some(now.clone()),
            source_picoseconds: Some(0),
            server_timestamp: Some(now.clone()),
            server_picoseconds: Some(0),
        }
    }

    pub fn new_bool(value: Boolean) -> DataValue {
        Self::new(Variant::Boolean(value))
    }

    pub fn new_byte(value: Byte) -> DataValue {
        Self::new(Variant::Byte(value))
    }

    pub fn new_i8(value: SByte) -> DataValue {
        Self::new(Variant::SByte(value))
    }

    pub fn new_u16(value: UInt16) -> DataValue {
        Self::new(Variant::UInt16(value))
    }

    pub fn new_i16(value: Int16) -> DataValue {
        Self::new(Variant::Int16(value))
    }

    pub fn new_u32(value: UInt32) -> DataValue {
        Self::new(Variant::UInt32(value))
    }

    pub fn new_i32(value: Int32) -> DataValue {
        Self::new(Variant::Int32(value))
    }

    pub fn new_f64(value: Double) -> DataValue {
        Self::new(Variant::Double(value))
    }

    pub fn new_f32(value: Float) -> DataValue {
        Self::new(Variant::Float(value))
    }

    pub fn new_string(value: UAString) -> DataValue {
        Self::new(Variant::String(value))
    }

    pub fn null() -> DataValue {
        let now = DateTime::now();
        DataValue {
            value: None,
            status: Some(GOOD),
            source_timestamp: Some(now.clone()),
            source_picoseconds: Some(0),
            server_timestamp: Some(now.clone()),
            server_picoseconds: Some(0),
        }
    }

    fn encoding_mask(&self) -> Byte {
        let mut encoding_mask: Byte = 0;
        if self.value.is_some() {
            encoding_mask |= HAS_VALUE;
        }
        if self.status.is_some() {
            encoding_mask |= HAS_STATUS;
        }
        if self.source_timestamp.is_some() {
            encoding_mask |= HAS_SOURCE_TIMESTAMP;
            if self.source_picoseconds.is_some() {
                encoding_mask |= HAS_SOURCE_PICOSECONDS;
            }
        }
        if self.server_timestamp.is_some() {
            encoding_mask |= HAS_SERVER_TIMESTAMP;
            if self.server_picoseconds.is_some() {
                encoding_mask |= HAS_SERVER_PICOSECONDS;
            }
        }
        encoding_mask
    }
}