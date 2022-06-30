use crate::ProtocolError;

use super::drive_types::{DocumentField, DocumentFieldType};
use super::DataContract;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// methods required for Drive
pub trait DriveExt {
    fn get_id(&self) -> &[u8; 32];
    fn document_types(&self) -> &BTreeMap<String, DocumentType> {
        todo!()
    }

    fn deserialize(
        serialized_contract: &[u8],
        contract_id: Option<[u8; 32]>,
        encoding: DriveEncoding,
    ) -> Result<Self, ProtocolError>
    where
        Self: Sized,
    {
        todo!("to implement")
    }

    fn from_cbor(contract_cbor: &[u8], contract_id: Option<[u8; 32]>) -> Result<Self, ProtocolError>
    where
        Self: Sized,
    {
        todo!("todo cbot")
    }

    fn root_path(&self) -> [&[u8]; 2];

    fn documents_path(&self) -> [&[u8]; 3];

    fn document_type_path<'a>(&'a self, document_type_name: &'a str) -> [&'a [u8]; 4];

    fn documents_primary_key_path<'a>(&'a self, document_type_name: &'a str) -> [&'a [u8]; 5];

    fn documents_with_history_primary_key_path<'a>(
        &'a self,
        document_type_name: &'a str,
        id: &'a [u8],
    ) -> [&'a [u8]; 6];

    fn document_type_for_name(
        &self,
        document_type_name: &str,
    ) -> Result<&DocumentType, ProtocolError>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct DocumentType {
    pub name: String,
    pub indices: Vec<Index>,
    pub properties: BTreeMap<String, DocumentField>,
    pub required_fields: BTreeSet<String>,
    pub documents_keep_history: bool,
    pub documents_mutable: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Index {
    pub properties: Vec<IndexProperty>,
    pub unique: bool,
}

impl Index {
    // The matches function will take a slice of an array of strings and an optional sort on value.
    // An index matches if all the index_names in the slice are consecutively the index's properties
    // with leftovers permitted.
    // If a sort_on value is provided it must match the last index property.
    // The number returned is the number of unused index properties

    // A case for example if we have an index on person's name and age
    // where we say name == 'Sam' sort by age
    // there is no field operator on age
    // The return value for name == 'Sam' sort by age would be 0
    // The return value for name == 'Sam and age > 5 sort by age would be 0
    // the return value for sort by age would be 1
    pub fn matches(
        &self,
        index_names: &[&str],
        in_field_name: Option<&str>,
        order_by: &[&str],
    ) -> Option<u16> {
        // Here we are trying to figure out if the Index matches the order by
        // To do so we take the index and go backwards as we need the order by clauses to be
        // continuous, but they do not need to be at the end.
        let mut reduced_properties = self.properties.as_slice();
        // let mut should_ignore: Vec<String> = order_by.iter().map(|&str| str.to_string()).collect();
        if !order_by.is_empty() {
            for _ in 0..self.properties.len() {
                if reduced_properties.len() < order_by.len() {
                    return None;
                }
                let matched_ordering = reduced_properties
                    .iter()
                    .rev()
                    .zip(order_by.iter().rev())
                    .all(|(property, &sort)| property.name.as_str() == sort);
                if matched_ordering {
                    break;
                }
                if let Some((_last, elements)) = reduced_properties.split_last() {
                    // should_ignore.push(last.name.clone());
                    reduced_properties = elements;
                } else {
                    return None;
                }
            }
        }

        let last_property = self.properties.last()?;

        // the in field can only be on the last or before last property
        if let Some(in_field_name) = in_field_name {
            if last_property.name.as_str() != in_field_name {
                // it can also be on the before last
                if self.properties.len() == 1 {
                    return None;
                }
                let before_last_property = self.properties.get(self.properties.len() - 2)?;
                if before_last_property.name.as_str() != in_field_name {
                    return None;
                }
            }
        }

        let mut d = self.properties.len();

        for search_name in index_names.iter() {
            if !reduced_properties
                .iter()
                .any(|property| property.name.as_str() == *search_name)
            {
                return None;
            }
            d -= 1;
        }

        Some(d as u16)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct IndexProperty {
    pub name: String,
    pub ascending: bool,
}

pub enum DriveEncoding {
    DriveCbor,
    DriveProtobuf,
}

#[repr(u8)]
pub enum RootTree {
    // Input data errors
    Identities = 0,
    ContractDocuments = 1,
    PublicKeyHashesToIdentities = 2,
    Misc = 3,
}

impl From<RootTree> for u8 {
    fn from(root_tree: RootTree) -> Self {
        root_tree as u8
    }
}

impl From<RootTree> for [u8; 1] {
    fn from(root_tree: RootTree) -> Self {
        [root_tree as u8]
    }
}

impl From<RootTree> for &'static [u8; 1] {
    fn from(root_tree: RootTree) -> Self {
        match root_tree {
            RootTree::Identities => &[0],
            RootTree::ContractDocuments => &[1],
            RootTree::PublicKeyHashesToIdentities => &[2],
            RootTree::Misc => &[3],
        }
    }
}

impl DriveExt for DataContract {
    fn get_id(&self) -> &[u8; 32] {
        &self.id.buffer
    }

    fn root_path(&self) -> [&[u8]; 2] {
        [
            Into::<&[u8; 1]>::into(RootTree::ContractDocuments),
            &self.id.buffer,
        ]
    }

    fn documents_path(&self) -> [&[u8]; 3] {
        [
            Into::<&[u8; 1]>::into(RootTree::ContractDocuments),
            &self.id.buffer,
            &[1],
        ]
    }

    fn document_type_path<'a>(&'a self, document_type_name: &'a str) -> [&'a [u8]; 4] {
        [
            Into::<&[u8; 1]>::into(RootTree::ContractDocuments),
            &self.id.buffer,
            &[1],
            document_type_name.as_bytes(),
        ]
    }

    fn documents_with_history_primary_key_path<'a>(
        &'a self,
        document_type_name: &'a str,
        id: &'a [u8],
    ) -> [&'a [u8]; 6] {
        [
            Into::<&[u8; 1]>::into(RootTree::ContractDocuments),
            &self.id.buffer,
            &[1],
            document_type_name.as_bytes(),
            &[0],
            id,
        ]
    }

    fn document_type_for_name(
        &self,
        document_type_name: &str,
    ) -> Result<&DocumentType, ProtocolError> {
        self.document_types()
            .get(document_type_name)
            .ok_or_else(|| {
                ProtocolError::Generic(format!("no document of type {}", document_type_name))
            })
    }

    fn documents_primary_key_path<'a>(&'a self, document_type_name: &'a str) -> [&'a [u8]; 5] {
        [
            Into::<&[u8; 1]>::into(RootTree::ContractDocuments),
            &self.id.buffer,
            &[1],
            document_type_name.as_bytes(),
            &[0],
        ]
    }
}
