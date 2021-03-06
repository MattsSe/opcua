use opcua_core::prelude::*;
use address_space::types::{Object, ObjectType, ReferenceType, Variable, VariableType, View, DataType, Method};

#[derive(Debug)]
pub enum NodeType {
    Object(Object),
    ObjectType(ObjectType),
    ReferenceType(ReferenceType),
    Variable(Variable),
    VariableType(VariableType),
    View(View),
    DataType(DataType),
    Method(Method),
}

impl NodeType {
    pub fn as_node(&self) -> &Node {
        match *self {
            NodeType::Object(ref value) => value,
            NodeType::ObjectType(ref value) => value,
            NodeType::ReferenceType(ref value) => value,
            NodeType::Variable(ref value) => value,
            NodeType::VariableType(ref value) => value,
            NodeType::View(ref value) => value,
            NodeType::DataType(ref value) => value,
            NodeType::Method(ref value) => value,
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.as_node().node_id()
    }
}

/// Implemented by Base and all derived Node types. Functions that return a result in an Option
/// do so because the attribute is optional and not necessarily there.
pub trait Node {
    fn node_class(&self) -> NodeClass;
    fn node_id(&self) -> NodeId;
    fn browse_name(&self) -> QualifiedName;
    fn display_name(&self) -> LocalizedText;
    fn description(&self) -> Option<LocalizedText>;
    fn write_mask(&self) -> Option<UInt32>;
    fn user_write_mask(&self) -> Option<UInt32>;
    fn find_attribute(&self, attribute_id: AttributeId) -> Option<DataValue>;
}
