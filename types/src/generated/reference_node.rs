// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Specifies a reference which belongs to a node.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceNode {
    pub reference_type_id: NodeId,
    pub is_inverse: Boolean,
    pub target_id: ExpandedNodeId,
}

impl MessageInfo for ReferenceNode {
    fn object_id(&self) -> ObjectId {
        ObjectId::ReferenceNode_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ReferenceNode> for ReferenceNode {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.reference_type_id.byte_len();
        size += self.is_inverse.byte_len();
        size += self.target_id.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_inverse.encode(stream)?;
        size += self.target_id.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let reference_type_id = NodeId::decode(stream)?;
        let is_inverse = Boolean::decode(stream)?;
        let target_id = ExpandedNodeId::decode(stream)?;
        Ok(ReferenceNode {
            reference_type_id: reference_type_id,
            is_inverse: is_inverse,
            target_id: target_id,
        })
    }
}
