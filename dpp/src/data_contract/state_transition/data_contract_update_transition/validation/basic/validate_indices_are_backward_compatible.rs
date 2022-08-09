use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::anyhow;

use crate::{
    consensus::basic::BasicError,
    data_contract::extra::IndexProperty,
    util::{
        json_schema::{Index, JsonSchemaExt},
        json_value::JsonValueExt,
    },
    validation::ValidationResult,
    ProtocolError,
};

type IndexName = String;
type DocumentType = String;
type JsonSchema = serde_json::Value;

pub fn validate_indices_are_backward_compatible<'a>(
    existing_documents: impl IntoIterator<Item = (&'a DocumentType, &'a JsonSchema)>,
    new_documents: impl IntoIterator<Item = (&'a DocumentType, &'a JsonSchema)>,
) -> Result<ValidationResult, ProtocolError> {
    let mut result = ValidationResult::default();
    let new_documents_by_type: HashMap<&DocumentType, &JsonSchema> =
        new_documents.into_iter().collect();

    for (document_type, existing_schema) in existing_documents.into_iter() {
        let name_index_map = to_index_by_name(
            new_documents_by_type
                .get(document_type)
                .ok_or_else(|| {
                    anyhow!(
                        "the document '{}' type doesn't exist in new definitions",
                        document_type
                    )
                })?
                .get_indices()?,
        );

        let existing_schema_indices = existing_schema.get_indices()?;

        let changed_unique_existing_index =
            get_changed_old_unique_index(&name_index_map, &existing_schema_indices);
        if let Some(changed_index) = changed_unique_existing_index {
            result.add_error(BasicError::DataContractUniqueIndicesChangedError {
                document_type: document_type.to_owned(),
                index_name: changed_index.name.clone(),
            });
        }

        let wrongly_updated_index =
            get_wrongly_updated_non_unique_index(&name_index_map, document_type, existing_schema);
        if let Some(index) = wrongly_updated_index {
            result.add_error(BasicError::DataContractInvalidIndexDefinitionUpdateError {
                document_type: document_type.to_owned(),
                index_name: index.name.clone(),
            })
        }

        let new_unique_index =
            get_new_unique_index(document_type, existing_schema, &new_documents_by_type)?;
        if let Some(index) = new_unique_index {
            result.add_error(BasicError::DataContractHaveNewUniqueIndexError {
                document_type: document_type.to_owned(),
                index_name: index.name.clone(),
            })
        }

        let wrongly_constructed_new_index = get_wrongly_constructed_new_index(
            document_type,
            existing_schema,
            &new_documents_by_type,
        )?;
        if let Some(index) = wrongly_constructed_new_index {
            result.add_error(BasicError::DataContractInvalidIndexDefinitionUpdateError {
                document_type: document_type.to_owned(),
                index_name: index.name.clone(),
            })
        }
    }

    Ok(result)
}

// Get one of the new indices that have old properties in them in the wrong order
// Explanation:
// Lets say we have two EXISTING Indexes: IndexA and IndexB.
// IndexA has properties: a,b,c
// IndexB has properties: b,c
// The function checks if a NEW index (i.e IndexC) contains one of possible sequences of properties.
// In the example, all possible sequences are: [a], [a,b], [a,b,c], [b], [b,c].
fn get_wrongly_constructed_new_index(
    document_type: &String,
    existing_schema: &JsonSchema,
    new_document_definitions: &HashMap<&DocumentType, &JsonSchema>,
) -> Result<Option<Index>, ProtocolError> {
    let new_schema_indices = new_document_definitions
        .get(document_type)
        .ok_or_else(|| {
            anyhow!(
            "getting wrongly constructed index failed: document {} doesn't exist in new schemas",
            document_type
        )
        })?
        .get_indices()
        .unwrap_or_default();
    let existing_schema_indices = existing_schema.get_indices().unwrap_or_default();
    let existing_index_names: Vec<&String> =
        existing_schema_indices.iter().map(|i| &i.name).collect();

    let existing_indexed_properties: Vec<&String> = existing_schema_indices
        .iter()
        .flat_map(|i| &i.properties)
        .map(|d| &d.name)
        .collect();

    let existing_index_snapshots =
        get_all_possible_sequences_of_properties(existing_schema_indices.iter());

    let new_indices = new_schema_indices
        .into_iter()
        .filter(|index| !existing_index_names.contains(&&index.name));

    for index_definition in new_indices {
        let existing_properties_len = index_definition
            .properties
            .iter()
            .filter(|prop| existing_indexed_properties.contains(&&prop.name))
            .count();
        if existing_properties_len == 0 {
            continue;
        }

        let properties_set = &index_definition.properties[..existing_properties_len];
        if !existing_index_snapshots.contains(properties_set) {
            return Ok(Some(index_definition));
        }
    }

    Ok(None)
}

fn get_all_possible_sequences_of_properties<'a>(
    indices: impl IntoIterator<Item = &'a Index>,
) -> HashSet<&'a [IndexProperty]> {
    let mut existing_index_snapshots: HashSet<&[IndexProperty]> = Default::default();
    for index in indices {
        for i in 0..index.properties.len() {
            existing_index_snapshots.insert(&index.properties[..i + 1]);
        }
    }
    existing_index_snapshots
}

fn get_new_unique_index(
    document_type: &String,
    existing_schema: &JsonSchema,
    new_schema: &HashMap<&DocumentType, &JsonSchema>,
) -> Result<Option<Index>, ProtocolError> {
    let new_document_schema = new_schema
        .get(document_type)
        .ok_or_else(|| anyhow!("the schema for document {} doesn't exist", document_type))?;
    let new_schema_indices = new_document_schema.get_indices().unwrap_or_default();
    let existing_index_names: HashMap<String, ()> = existing_schema
        .get_indices()
        .unwrap_or_default()
        .into_iter()
        .map(|i| (i.name, ()))
        .collect();

    // Gather only new defined indexes
    let maybe_new_unique_index = new_schema_indices
        .into_iter()
        .filter(|i| !existing_index_names.contains_key(&i.name))
        .find(|i| i.unique);

    Ok(maybe_new_unique_index)
}

fn get_wrongly_updated_non_unique_index<'a>(
    name_index_map: &'a HashMap<String, Index>,
    _document_type: &str,
    existing_schema: &'a JsonSchema,
) -> Option<Index> {
    // Checking every existing non-unique index, and it's respective new index
    // if they are changed per spec
    let existing_schema_indices = existing_schema.get_indices().unwrap_or_default();
    for index_definition in existing_schema_indices.into_iter().filter(|i| !i.unique) {
        let maybe_new_index_definition = name_index_map.get(&index_definition.name);
        if let Some(new_index_definition) = maybe_new_index_definition {
            // non-unique index can be ONLY updated by appending. The 'old' properties in the new
            // index must remain intact.
            let index_properties_len = index_definition.properties.len();
            if new_index_definition.properties[0..index_properties_len]
                != index_definition.properties
            {
                return Some(index_definition);
            }

            // check if the rest of new indexes are defined in the existing schema
            for property in
                new_index_definition.properties[index_definition.properties.len()..].iter()
            {
                if existing_schema.get_value(&property.name).is_ok() {
                    return Some(index_definition);
                }
            }
        }
    }
    None
}

fn to_index_by_name(indices: Vec<Index>) -> HashMap<String, Index> {
    let mut indices_by_name: HashMap<String, Index> = HashMap::new();
    for index in indices.into_iter() {
        // apparently there is an assumption that the name for the index must be unique
        indices_by_name.insert(index.name.clone(), index);
    }
    indices_by_name
}

// The old and *UNIQUE* indices cannot be modified
// returns first unique index that has changed when comparing to the `new_indices`
fn get_changed_old_unique_index<'a>(
    new_indices: &'a HashMap<IndexName, Index>,
    existing_indices: &'a [Index],
) -> Option<&'a Index> {
    existing_indices
        .iter()
        .find(|i| indexes_are_not_equal(i, new_indices.get(&i.name)) && i.unique)
}

fn indexes_are_not_equal(index_a: &Index, index_b: Option<&Index>) -> bool {
    match index_b {
        None => true,
        Some(index) => index_a != index,
    }
}

#[test]
fn test_collect_all_possible_sequences() {
    let indices: Vec<Index> = vec![
        Index {
            name: "bravo_index".to_string(),
            unique: false,
            properties: vec![
                IndexProperty {
                    name: "bravo_index_property_1".to_string(),
                    ascending: true,
                },
                IndexProperty {
                    name: "bravo_index_property_2".to_string(),
                    ascending: true,
                },
            ],
        },
        Index {
            name: "alpha_index".to_string(),
            unique: false,
            properties: vec![
                IndexProperty {
                    name: "alpha_index_property_1".to_string(),
                    ascending: true,
                },
                IndexProperty {
                    name: "alpha_index_property_2".to_string(),
                    ascending: true,
                },
                IndexProperty {
                    name: "alpha_index_property_3".to_string(),
                    ascending: true,
                },
            ],
        },
    ];

    let sequences = get_all_possible_sequences_of_properties(indices.iter());
    assert_eq!(5, sequences.len());
    assert!(sequences.contains(&indices[0].properties[..1]));
    assert!(sequences.contains(&indices[0].properties[..2]));
    assert!(sequences.contains(&indices[1].properties[..1]));
    assert!(sequences.contains(&indices[1].properties[..2]));
    assert!(sequences.contains(&indices[1].properties[..3]));
}
