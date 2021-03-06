// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use attribute::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use generated::node_ids::*;
#[allow(unused_imports)]
use generated::status_codes::StatusCode;
#[allow(unused_imports)]
use generated::status_codes::StatusCode::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryReadValueId {
    pub node_id: NodeId,
    pub index_range: UAString,
    pub data_encoding: QualifiedName,
    pub continuation_point: ByteString,
}

impl MessageInfo for HistoryReadValueId {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryReadValueId_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryReadValueId> for HistoryReadValueId {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.index_range.byte_len();
        size += self.data_encoding.byte_len();
        size += self.continuation_point.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.data_encoding.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let index_range = UAString::decode(stream)?;
        let data_encoding = QualifiedName::decode(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        Ok(HistoryReadValueId {
            node_id,
            index_range,
            data_encoding,
            continuation_point,
        })
    }
}
