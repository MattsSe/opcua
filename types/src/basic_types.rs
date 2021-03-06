use std;
use std::io::{Read, Write, Cursor};
use std::fmt;

use encoding::*;
use generated::StatusCode;
use generated::StatusCode::*;
use node_id::NodeId;
use constants;

// OPC UA Part 6 - Mappings 1.03 Specification

// These are standard UA types

/// A two-state logical value (true or false).
/// Data type ID 1
pub type Boolean = bool;

impl BinaryEncoder<Boolean> for Boolean {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        // 0, or 1 for true or false, single byte
        write_u8(stream, if *self { 1 } else { 0 })
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let value = if read_u8(stream)? == 1 { true } else { false };
        Ok(value)
    }
}

/// An integer value between −128 and 127.
/// Data type ID 2
pub type SByte = i8;

impl BinaryEncoder<SByte> for SByte {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u8(stream, *self as u8)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        Ok(read_u8(stream)? as i8)
    }
}

/// An integer value between 0 and 255.
/// Data type ID 3
pub type Byte = u8;

impl BinaryEncoder<Byte> for Byte {
    fn byte_len(&self) -> usize {
        1
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u8(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        Ok(read_u8(stream)?)
    }
}

/// An integer value between −32 768 and 32 767.
/// Data type ID 4
pub type Int16 = i16;

impl BinaryEncoder<Int16> for Int16 {
    fn byte_len(&self) -> usize {
        2
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i16(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_i16(stream)
    }
}

/// An integer value between 0 and 65 535.
/// Data type ID 5
pub type UInt16 = u16;

impl BinaryEncoder<UInt16> for UInt16 {
    fn byte_len(&self) -> usize {
        2
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u16(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_u16(stream)
    }
}

/// An integer value between −2 147 483 648 and 2 147 483 647.
/// Data type ID 6
pub type Int32 = i32;

impl BinaryEncoder<Int32> for Int32 {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i32(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_i32(stream)
    }
}

/// An integer value between 0 and 4 294 967 295.
/// Data type ID 7
pub type UInt32 = u32;

impl BinaryEncoder<UInt32> for UInt32 {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u32(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_u32(stream)
    }
}

/// An integer value between −9 223 372 036 854 775 808 and 9 223 372 036 854 775 807
/// Data type ID 8
pub type Int64 = i64;

impl BinaryEncoder<Int64> for Int64 {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_i64(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_i64(stream)
    }
}

/// An integer value between 0 and 18 446 744 073 709 551 615.
/// Data type ID 9
pub type UInt64 = u64;

impl BinaryEncoder<UInt64> for UInt64 {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_u64(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_u64(stream)
    }
}

/// An IEEE single precision (32 bit) floating point value.
/// Data type ID 10
pub type Float = f32;

impl BinaryEncoder<Float> for Float {
    fn byte_len(&self) -> usize {
        4
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_f32(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_f32(stream)
    }
}

/// An IEEE double precision (64 bit) floating point value.
/// Data type ID 11
pub type Double = f64;

impl BinaryEncoder<Double> for Double {
    fn byte_len(&self) -> usize {
        8
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        write_f64(stream, *self)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        read_f64(stream)
    }
}

/// A UTF-8 encoded sequence of Unicode characters.
///
/// A string can hold a null value, so the string value is optional.
/// When there is no string, the value is treated as null
///
/// To avoid naming conflict hell, the String type is named UAString.
///
/// Data type ID 12
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct UAString {
    pub value: Option<String>,
}

impl BinaryEncoder<UAString> for UAString {
    fn byte_len(&self) -> usize {
        // Length plus the actual length of bytes (if not null)
        4 + if self.value.is_none() { 0 } else { self.value.as_ref().unwrap().len() }
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        // Strings are uncoded as UTF8 chars preceded by an Int32 length. A -1 indicates a null string
        if self.value.is_none() {
            write_i32(stream, -1)
        } else {
            let value = self.value.clone().unwrap();
            let mut size: usize = 0;
            size += write_i32(stream, value.len() as i32)?;
            let buf = value.as_bytes();
            size += process_encode_io_result(stream.write(&buf))?;
            assert_eq!(size, self.byte_len());
            Ok(size)
        }
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let buf_len = Int32::decode(stream)?;
        // Null string?
        if buf_len == -1 {
            return Ok(UAString::null());
        } else if buf_len < -1 {
            error!("String buf length is a negative number {}", buf_len);
            return Err(BAD_DECODING_ERROR);
        } else if buf_len > constants::MAX_STRING_LENGTH as i32 {
            error!("String buf length {} is larger than max string length", buf_len);
            return Err(BAD_ENCODING_LIMITS_EXCEEDED);
        }

        // Create the actual UTF8 string
        let mut string_buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        string_buf.resize(buf_len as usize, 0u8);
        process_decode_io_result(stream.read_exact(&mut string_buf))?;
        Ok(UAString {
            value: Some(String::from_utf8(string_buf).unwrap())
        })
    }
}

impl AsRef<str> for UAString {
    fn as_ref(&self) -> &str {
        if self.is_null() { "" } else { self.value.as_ref().unwrap() }
    }
}

impl UAString {
    /// Create a string from a string slice
    pub fn from_str(value: &str) -> UAString {
        UAString { value: Some(value.to_string()) }
    }

    /// Returns the length of the string or -1 for null
    pub fn len(&self) -> isize {
        if self.value.is_none() { -1 } else { self.value.as_ref().unwrap().len() as isize }
    }

    /// Create a null string (not the same as an empty string)
    pub fn null() -> UAString {
        UAString { value: None }
    }

    /// Test if the string is null
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
}

// Data type ID 13 - UADateTime is in date_time.rs

/// A 16 byte value that can be used as a globally unique identifier.
/// Data type ID 14
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Guid {
    pub data1: UInt32,
    pub data2: UInt16,
    pub data3: UInt16,
    pub data4: [Byte; 8],
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_hyphenated_string())
    }
}

impl BinaryEncoder<Guid> for Guid {
    fn byte_len(&self) -> usize {
        16
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        let data = [(self.data1 >> 0) as u8,
            (self.data1 >> 8) as u8,
            (self.data1 >> 16) as u8,
            (self.data1 >> 24) as u8,
            (self.data2 >> 0) as u8,
            (self.data2 >> 8) as u8,
            (self.data3 >> 0) as u8,
            (self.data3 >> 8) as u8,
            self.data4[0], self.data4[1], self.data4[2], self.data4[3], self.data4[4], self.data4[5], self.data4[6], self.data4[7]
        ];
        size += process_encode_io_result(stream.write(&data))?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let mut data = [0u8; 16];
        process_decode_io_result(stream.read_exact(&mut data))?;
        let data1: UInt32 = (data[0] as UInt32).wrapping_shl(0) + (data[1] as UInt32).wrapping_shl(8) + (data[2] as UInt32).wrapping_shl(16) + (data[3] as UInt32).wrapping_shl(24);
        let data2: UInt16 = (data[4] as UInt16).wrapping_shl(0) + (data[5] as UInt16).wrapping_shl(8);
        let data3: UInt16 = (data[6] as UInt16).wrapping_shl(0) + (data[7] as UInt16).wrapping_shl(8);
        let data4 = [data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]];
        Ok(Guid { data1: data1, data2: data2, data3: data3, data4: data4 })
    }
}

const SIMPLE_LENGTH: usize = 32;
const HYPHENATED_LENGTH: usize = 36;

// Accumulated length of each hyphenated group in hex digits.
const ACC_GROUP_LENS: [u8; 5] = [8, 12, 16, 20, 32];

impl Guid {
    pub fn parse_str(input: &str) -> std::result::Result<Guid, ()> {
        // Adapted from Uuid::parse_str - https://github.com/rust-lang-nursery/uuid/blob/master/src/lib.rs
        // Main difference is we decode the Guid from the buffer at the end and there are no error
        // codes

        let len = input.len();
        if len != SIMPLE_LENGTH && len != HYPHENATED_LENGTH {
            return Err(());
        }

        let mut digit = 0;
        let mut group = 0;
        let mut acc = 0;
        let mut buffer = [0u8; 16];

        for (_, chr) in input.chars().enumerate() {
            if digit as usize >= SIMPLE_LENGTH && group == 0 {
                return Err(());
            }
            if digit % 2 == 0 {
                // First digit of the byte.
                match chr {
                    // Calculate upper half.
                    '0' ... '9' => acc = chr as u8 - '0' as u8,
                    'a' ... 'f' => acc = chr as u8 - 'a' as u8 + 10,
                    'A' ... 'F' => acc = chr as u8 - 'A' as u8 + 10,
                    // Found a group delimiter
                    '-' => {
                        if ACC_GROUP_LENS[group] != digit {
                            // Calculate how many digits this group consists of in the input.
                            return Err(());
                        }
                        // Next group, decrement digit, it is incremented again at the bottom.
                        group += 1;
                        digit -= 1;
                    }
                    _ => return Err(()),
                }
            } else {
                // Second digit of the byte, shift the upper half.
                acc *= 16;
                match chr {
                    '0' ... '9' => acc += chr as u8 - '0' as u8,
                    'a' ... 'f' => acc += chr as u8 - 'a' as u8 + 10,
                    'A' ... 'F' => acc += chr as u8 - 'A' as u8 + 10,
                    '-' => {
                        // The byte isn't complete yet.
                        return Err(());
                    }
                    _ => return Err(()),
                }
                buffer[(digit / 2) as usize] = acc;
            }
            digit += 1;
        }

        // Now check the last group.
        if group != 0 && group != 4 {
            return Err(());
        } else if ACC_GROUP_LENS[4] != digit {
            return Err(());
        }

        let mut stream = Cursor::new(&buffer);
        Ok(Guid::decode(&mut stream).unwrap())
    }

    pub fn as_hyphenated_string(&self) -> String {
        format!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                self.data1, self.data2, self.data3, self.data4[0], self.data4[1], self.data4[2], self.data4[3], self.data4[4], self.data4[5], self.data4[6], self.data4[7])
    }
}

/// A sequence of octets.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ByteString {
    pub value: Option<Vec<u8>>,
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        if self.value.is_none() { &[] } else { self.value.as_ref().unwrap() }
    }
}

impl BinaryEncoder<ByteString> for ByteString {
    fn byte_len(&self) -> usize {
        // Length plus the actual length of bytes (if not null)
        4 + if self.value.is_none() { 0 } else { self.value.as_ref().unwrap().len() }
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        // Strings are uncoded as UTF8 chars preceded by an Int32 length. A -1 indicates a null string
        if self.value.is_none() {
            write_i32(stream, -1)
        } else {
            let mut size: usize = 0;
            let value = self.value.as_ref().unwrap();
            size += write_i32(stream, value.len() as i32)?;
            size += process_encode_io_result(stream.write(value))?;
            assert_eq!(size, self.byte_len());
            Ok(size)
        }
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let buf_len = Int32::decode(stream)?;
        // Null string?
        if buf_len == -1 {
            return Ok(ByteString::null());
        } else if buf_len < -1 {
            error!("ByteString buf length is a negative number {}", buf_len);
            return Err(BAD_DECODING_ERROR);
        } else if buf_len > constants::MAX_BYTE_STRING_LENGTH as i32 {
            error!("ByteString buf length {} is longer than max byte string length", buf_len);
            return Err(BAD_ENCODING_LIMITS_EXCEEDED);
        }

        // Create the actual UTF8 string
        let mut string_buf: Vec<u8> = Vec::with_capacity(buf_len as usize);
        string_buf.resize(buf_len as usize, 0u8);
        process_decode_io_result(stream.read_exact(&mut string_buf))?;
        Ok(ByteString {
            value: Some(string_buf)
        })
    }
}

impl ByteString {
    /// Create a null string (not the same as an empty string)
    pub fn null() -> ByteString {
        ByteString { value: None }
    }

    /// Test if the string is null
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }

    /// Create a byte string with a number of random characters. Can be used to create a nonce or
    /// a similar reason.
    pub fn random(number_of_bytes: usize) -> ByteString {
        use rand::{self, Rng};
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; number_of_bytes];
        rng.fill_bytes(&mut bytes);
        ByteString::from_bytes(&bytes)
    }

    /// Create a byte string from an array of bytes
    pub fn from_bytes(v: &[u8]) -> ByteString {
        ByteString { value: Some(v.to_vec()) }
    }
}

/// An XML element.
/// Data type ID 16
pub type XmlElement = UAString;

// NodeId and ExtendedNodeId are in node_id.rs

/// A numeric identifier for a error or condition that is associated with a value or an operation.
/// Data type ID 19

/// A name qualified by a namespace.
/// Data type ID 20
#[derive(PartialEq, Debug, Clone)]
pub struct QualifiedName {
    /// The namespace index.
    pub namespace_index: UInt16,
    /// The name.
    pub name: UAString,
}

impl BinaryEncoder<QualifiedName> for QualifiedName {
    fn byte_len(&self) -> usize {
        let mut size: usize = 0;
        size += self.namespace_index.byte_len();
        size += self.name.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        size += self.namespace_index.encode(stream)?;
        size += self.name.encode(stream)?;
        assert_eq!(size, self.byte_len());
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let namespace_index = UInt16::decode(stream)?;
        let name = UAString::decode(stream)?;
        Ok(QualifiedName {
            namespace_index: namespace_index,
            name: name,
        })
    }
}

impl QualifiedName {
    pub fn new(namespace_index: UInt16, name: &str) -> QualifiedName {
        QualifiedName {
            namespace_index: namespace_index,
            name: UAString::from_str(name),
        }
    }

    pub fn null() -> QualifiedName {
        QualifiedName {
            namespace_index: 0,
            name: UAString::null(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.namespace_index == 0 && self.name.is_null()
    }
}

/// Human readable text with an optional locale identifier
/// Data type ID 21
#[derive(PartialEq, Debug, Clone)]
pub struct LocalizedText {
    /// The locale. Omitted from stream if null or empty
    pub locale: UAString,
    /// The text in the specified locale. Omitted frmo stream if null or empty.
    pub text: UAString,
}

impl BinaryEncoder<LocalizedText> for LocalizedText {
    fn byte_len(&self) -> usize {
        let mut size = 1;
        if self.locale.len() > 0 {
            size += self.locale.byte_len();
        }
        if self.text.len() > 0 {
            size += self.text.byte_len();
        }
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        // A bit mask that indicates which fields are present in the stream.
        // The mask has the following bits:
        // 0x01    Locale
        // 0x02    Text
        let mut encoding_mask: Byte = 0;
        if self.locale.len() > 0 {
            encoding_mask |= 0x1;
        }
        if self.text.len() > 0 {
            encoding_mask |= 0x2;
        }
        size += encoding_mask.encode(stream)?;
        if self.locale.len() > 0 {
            size += self.locale.encode(stream)?;
        }
        if self.text.len() > 0 {
            size += self.text.encode(stream)?;
        }
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let encoding_mask = Byte::decode(stream)?;
        let locale = if encoding_mask & 0x1 != 0 {
            UAString::decode(stream)?
        } else {
            UAString::null()
        };
        let text = if encoding_mask & 0x2 != 0 {
            UAString::decode(stream)?
        } else {
            UAString::null()
        };
        Ok(LocalizedText {
            locale: locale,
            text: text,
        })
    }
}

impl LocalizedText {
    pub fn new(locale: &str, text: &str) -> LocalizedText {
        LocalizedText {
            locale: UAString::from_str(locale),
            text: UAString::from_str(text),
        }
    }

    pub fn null() -> LocalizedText {
        LocalizedText {
            locale: UAString::null(),
            text: UAString::null(),
        }
    }
}

/// Enumeration that holds the kinds of encoding that an ExtensionObject data may be encoded with.
#[derive(PartialEq, Debug, Clone)]
pub enum ExtensionObjectEncoding {
    /// For an extension object with nothing encoded with it
    None,
    /// For an extension object with data encoded in a ByteString
    ByteString(ByteString),
    /// For an extension object with data encoded in an XML string
    XmlElement(XmlElement),
}

/// A structure that contains an application specific data type that may not be recognized by the receiver.
/// Data type ID 22
#[derive(PartialEq, Debug, Clone)]
pub struct ExtensionObject {
    pub node_id: NodeId,
    pub body: ExtensionObjectEncoding,
}

impl BinaryEncoder<ExtensionObject> for ExtensionObject {
    fn byte_len(&self) -> usize {
        let mut size = self.node_id.byte_len();
        size += match self.body {
            ExtensionObjectEncoding::None => 1,
            ExtensionObjectEncoding::ByteString(ref value) => {
                // Encoding mask + data
                1 + value.byte_len()
            }
            ExtensionObjectEncoding::XmlElement(ref value) => {
                // Encoding mask + data
                1 + value.byte_len()
            }
        };
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        match self.body {
            ExtensionObjectEncoding::None => {
                size += write_u8(stream, 0x0)?;
            }
            ExtensionObjectEncoding::ByteString(ref value) => {
                // Encoding mask + data
                size += write_u8(stream, 0x1)?;
                size += value.encode(stream)?;
            }
            ExtensionObjectEncoding::XmlElement(ref value) => {
                // Encoding mask + data
                size += write_u8(stream, 0x2)?;
                size += value.encode(stream)?;
            }
        }
        assert_eq!(size, self.byte_len());
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let encoding_type = Byte::decode(stream)?;
        let body = match encoding_type {
            0x0 => {
                ExtensionObjectEncoding::None
            }
            0x1 => {
                let value = ByteString::decode(stream);
                if value.is_err() {
                    return Err(value.unwrap_err());
                }
                ExtensionObjectEncoding::ByteString(value.unwrap())
            }
            0x2 => {
                let value = XmlElement::decode(stream);
                if value.is_err() {
                    return Err(value.unwrap_err());
                }
                ExtensionObjectEncoding::XmlElement(value.unwrap())
            }
            _ => {
                error!("Invalid encoding type {} in stream", encoding_type);
                return Err(BAD_DECODING_ERROR);
            }
        };
        Ok(ExtensionObject {
            node_id: node_id,
            body: body,
        })
    }
}

impl ExtensionObject {
    /// Creates a null extension object, i.e. one with no value or payload
    pub fn null() -> ExtensionObject {
        ExtensionObject {
            node_id: NodeId::null(),
            body: ExtensionObjectEncoding::None,
        }
    }

    /// Creates an extension object with the specified node id and the encodable object as its payload.
    /// The body is set to a byte string containing the encoded struct.
    pub fn from_encodable<T: BinaryEncoder<T>>(node_id: NodeId, encodable: T) -> ExtensionObject {
        // Serialize to extension object
        let mut stream = Cursor::new(vec![0u8; encodable.byte_len()]);
        let _ = encodable.encode(&mut stream);
        ExtensionObject {
            node_id: node_id,
            body: ExtensionObjectEncoding::ByteString(ByteString::from_bytes(&stream.into_inner())),
        }
    }

    /// Decodes the inner content of the extension object and returns it. The node id is ignored
    /// for decoding. The caller supplies the binary encoder impl that should be used to extract
    /// the data. Errors result in a decoding error.
    pub fn decode_inner<T: BinaryEncoder<T>>(&self) -> EncodingResult<T> {
        if let ExtensionObjectEncoding::ByteString(ref byte_string) = self.body {
            if let Some(ref value) = byte_string.value {
                let value = value.clone();
                let mut stream = Cursor::new(value);
                return T::decode(&mut stream);
            }
        }
        Err(BAD_DECODING_ERROR)
    }
}

// Data type ID 23 is in data_value.rs

// Data type ID 24 is in variant.rs

#[allow(non_snake_case)]
mod DiagnosticInfoMask {
    pub const HAS_SYMBOLIC_ID: u8 = 0x01;
    pub const HAS_NAMESPACE: u8 = 0x02;
    pub const HAS_LOCALIZED_TEXT: u8 = 0x04;
    pub const HAS_LOCALE: u8 = 0x08;
    pub const HAS_ADDITIONAL_INFO: u8 = 0x10;
    pub const HAS_INNER_STATUS_CODE: u8 = 0x20;
    pub const HAS_INNER_DIAGNOSTIC_INFO: u8 = 0x40;
}

/// Data type ID 25
#[derive(PartialEq, Debug, Clone)]
pub struct DiagnosticInfo {
    /// A symbolic name for the status code.
    pub symbolic_id: Option<Int32>,
    /// A namespace that qualifies the symbolic id.
    pub namespace_uri: Option<Int32>,
    /// The locale used for the localized text.
    pub locale: Option<Int32>,
    /// A human readable summary of the status code.
    pub localized_text: Option<Int32>,
    /// Detailed application specific diagnostic information.
    pub additional_info: Option<UAString>,

    /// A status code provided by an underlying system.
    pub inner_status_code: Option<StatusCode>,
    /// Diagnostic info associated with the inner status code.
    pub inner_diagnostic_info: Option<Box<DiagnosticInfo>>,
}

impl BinaryEncoder<DiagnosticInfo> for DiagnosticInfo {
    fn byte_len(&self) -> usize {
        let mut size: usize = 0;
        size += 1; // self.encoding_mask())
        if let Some(ref symbolic_id) = self.symbolic_id {
            // Write symbolic id
            size += symbolic_id.byte_len();
        }
        if let Some(ref namespace_uri) = self.namespace_uri {
            // Write namespace
            size += namespace_uri.byte_len()
        }
        if let Some(ref locale) = self.locale {
            // Write locale
            size += locale.byte_len()
        }
        if let Some(ref localized_text) = self.localized_text {
            // Write localized text
            size += localized_text.byte_len()
        }
        if let Some(ref additional_info) = self.additional_info {
            // Write Additional info
            size += additional_info.byte_len()
        }
        if let Some(ref inner_status_code) = self.inner_status_code {
            // Write inner status code
            size += inner_status_code.byte_len()
        }
        if let Some(ref inner_diagnostic_info) = self.inner_diagnostic_info {
            // Write inner diagnostic info
            size += inner_diagnostic_info.byte_len()
        }
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;
        size += write_u8(stream, self.encoding_mask())?;
        if let Some(ref symbolic_id) = self.symbolic_id {
            // Write symbolic id
            size += write_i32(stream, *symbolic_id)?;
        }
        if let Some(ref namespace_uri) = self.namespace_uri {
            // Write namespace
            size += namespace_uri.encode(stream)?;
        }
        if let Some(ref locale) = self.locale {
            // Write locale
            size += locale.encode(stream)?;
        }
        if let Some(ref localized_text) = self.localized_text {
            // Write localized text
            size += localized_text.encode(stream)?;
        }
        if let Some(ref additional_info) = self.additional_info {
            // Write Additional info
            size += additional_info.encode(stream)?;
        }
        if let Some(ref inner_status_code) = self.inner_status_code {
            // Write inner status code
            size += inner_status_code.encode(stream)?;
        }
        if let Some(ref inner_diagnostic_info) = self.inner_diagnostic_info {
            // Write inner diagnostic info
            size += inner_diagnostic_info.clone().encode(stream)?;
        }
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let encoding_mask = Byte::decode(stream)?;
        let mut diagnostic_info = DiagnosticInfo::new();
        if encoding_mask & DiagnosticInfoMask::HAS_SYMBOLIC_ID != 0 {
            // Read symbolic id
            diagnostic_info.symbolic_id = Some(Int32::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_NAMESPACE != 0 {
            // Read namespace
            diagnostic_info.namespace_uri = Some(Int32::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_LOCALE != 0 {
            // Read locale
            diagnostic_info.locale = Some(Int32::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_LOCALIZED_TEXT != 0 {
            // Read localized text
            diagnostic_info.localized_text = Some(Int32::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_ADDITIONAL_INFO != 0 {
            // Read Additional info
            diagnostic_info.additional_info = Some(UAString::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_INNER_STATUS_CODE != 0 {
            // Read inner status code
            diagnostic_info.inner_status_code = Some(StatusCode::decode(stream)?);
        }
        if encoding_mask & DiagnosticInfoMask::HAS_INNER_DIAGNOSTIC_INFO != 0 {
            // Read inner diagnostic info
            diagnostic_info.inner_diagnostic_info = Some(Box::new(DiagnosticInfo::decode(stream)?));
        }
        Ok(diagnostic_info)
    }
}

impl DiagnosticInfo {
    pub fn new() -> DiagnosticInfo {
        DiagnosticInfo {
            symbolic_id: None,
            namespace_uri: None,
            locale: None,
            localized_text: None,
            additional_info: None,
            inner_status_code: None,
            inner_diagnostic_info: None,
        }
    }

    pub fn encoding_mask(&self) -> u8 {
        let mut encoding_mask: u8 = 0;
        if self.symbolic_id.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_SYMBOLIC_ID;
        }
        if self.namespace_uri.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_NAMESPACE;
        }
        if self.locale.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_LOCALE;
        }
        if self.localized_text.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_LOCALIZED_TEXT;
        }
        if self.additional_info.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_ADDITIONAL_INFO;
        }
        if self.inner_status_code.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_INNER_STATUS_CODE;
        }
        if self.inner_diagnostic_info.is_some() {
            encoding_mask |= DiagnosticInfoMask::HAS_INNER_DIAGNOSTIC_INFO;
        }
        encoding_mask
    }
}
