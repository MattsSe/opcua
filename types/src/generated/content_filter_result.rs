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
use generated::ContentFilterElementResult;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilterResult {
    pub element_results: Option<Vec<ContentFilterElementResult>>,
    pub element_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for ContentFilterResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterResult> for ContentFilterResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.element_results);
        size += byte_len_array(&self.element_diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.element_results)?;
        size += write_array(stream, &self.element_diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let element_results: Option<Vec<ContentFilterElementResult>> = read_array(stream)?;
        let element_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(ContentFilterResult {
            element_results,
            element_diagnostic_infos,
        })
    }
}
