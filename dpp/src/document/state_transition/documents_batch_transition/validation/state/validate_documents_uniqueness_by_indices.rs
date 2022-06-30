use crate::{
    document::{
        document_transition::{Action, DocumentTransition},
        Document,
    },
    prelude::{DataContract, Identifier},
    state_repository::StateRepositoryLike,
    util::{
        json_schema::{Index, JsonSchemaExt},
        json_value::JsonValueExt,
        string_encoding::Encoding,
    },
    validation::ValidationResult,
    ProtocolError, StateError,
};
use futures::future::join_all;
use itertools::Itertools;
use serde_json::{json, Value as JsonValue};

struct QueryDefinition<'a> {
    document_type: &'a str,
    where_query: Vec<JsonValue>,
    index_definition: &'a Index,
    document_transition: &'a DocumentTransition,
}

async fn validate_documents_uniqueness_by_indices<SR>(
    state_repository: SR,
    owner_id: &Identifier,
    document_transitions: &[DocumentTransition],
    data_contract: &DataContract,
) -> Result<ValidationResult, ProtocolError>
where
    SR: StateRepositoryLike,
{
    let mut validation_result = ValidationResult::default();

    // 1. Prepare fetchDocuments queries from indexed properties
    for transition in document_transitions.iter() {
        let document_schema =
            data_contract.get_document_schema(&transition.base().document_type)?;

        let document_indices = document_schema.get_indices()?;
        if document_indices.is_empty() {
            continue;
        }

        let document_index_queries =
            generate_document_index_queries(&document_indices, transition, owner_id);

        let queries = document_index_queries
            .filter(|query| !query.where_query.is_empty())
            .map(|query| {
                (
                    state_repository.fetch_documents::<Document>(
                        &data_contract.id,
                        query.document_type,
                        JsonValue::Array(query.where_query),
                    ),
                    (query.index_definition, query.document_transition),
                )
            });

        let (futures, futures_meta) = unzip_iter_and_collect(queries);
        let results = join_all(futures).await;

        let result = validate_uniqueness(futures_meta, results)?;
        validation_result.merge(result);
    }

    Ok(validation_result)
}

fn generate_document_index_queries<'a>(
    indices: &'a [Index],
    transition: &'a DocumentTransition,
    owner_id: &'a Identifier,
) -> impl Iterator<Item = QueryDefinition<'a>> {
    indices
        .iter()
        .filter(|index| index.unique)
        .map(move |index| {
            let where_query = build_query_for_index_definition(index, transition, owner_id);
            QueryDefinition {
                document_type: &transition.base().document_type,
                index_definition: index,
                document_transition: transition,
                where_query,
            }
        })
}

fn validate_uniqueness<'a>(
    futures_meta: Vec<(&'a Index, &'a DocumentTransition)>,
    results: Vec<Result<Vec<Document>, anyhow::Error>>,
) -> Result<ValidationResult, ProtocolError> {
    let mut validation_result = ValidationResult::default();
    for (i, result) in results.into_iter().enumerate() {
        let documents = result?;
        let only_origin_document =
            documents.len() == 1 && documents[0].id == futures_meta[i].1.base().id;
        if documents.is_empty() || only_origin_document {
            continue;
        }

        validation_result.add_error(StateError::DuplicateUniqueIndexError {
            document_id: futures_meta[i].1.base().id.clone(),
            duplicating_properties: futures_meta[i]
                .0
                .properties
                .iter()
                .map(|map| map.keys().next().unwrap().clone())
                .collect_vec(),
        })
    }
    Ok(validation_result)
}

fn build_query_for_index_definition(
    index_definition: &Index,
    transition: &DocumentTransition,
    owner_id: &Identifier,
) -> Vec<JsonValue> {
    let mut query = vec![];
    for index_property in index_definition.properties.iter() {
        let index_entry = index_property.iter().next();
        if index_entry.is_none() {
            continue;
        }
        let property_name = index_entry.unwrap().0;

        match property_name.as_str() {
            "$ownerId" => {
                let id = owner_id.to_string(Encoding::Base58);
                query.push(json!([property_name, "==", id]))
            }
            "$createdAt" => {
                if transition.base().action == Action::Create {
                    if let Some(transition_create) = transition.as_transition_create() {
                        if let Some(created_at) = transition_create.created_at.map(|v| json!(v)) {
                            query.push(json!([property_name, "==", created_at]));
                        }
                    }
                }
            }
            "$updatedAt" => {
                if transition.base().action == Action::Create {
                    if let Some(updated_at) = get_updated_at(transition).map(|v| json!(v)) {
                        query.push(json!([property_name, "==", updated_at]))
                    }
                }
            }

            _ => {
                if let Some(value) = get_property(property_name, transition) {
                    query.push(json!([property_name, "==", value]))
                }
            }
        }
    }
    query
}

fn get_updated_at(transition: &DocumentTransition) -> Option<i64> {
    match transition {
        DocumentTransition::Create(t) => t.updated_at,
        DocumentTransition::Replace(t) => t.updated_at,
        DocumentTransition::Delete(_) => None,
    }
}

fn get_property<'a>(path: &str, transition: &'a DocumentTransition) -> Option<&'a JsonValue> {
    match transition {
        DocumentTransition::Create(t) => {
            if let Some(ref data) = t.data {
                data.get_value(path).ok()
            } else {
                None
            }
        }
        DocumentTransition::Replace(t) => {
            if let Some(ref data) = t.data {
                data.get_value(path).ok()
            } else {
                None
            }
        }
        DocumentTransition::Delete(t) => None,
    }
}

fn unzip_iter_and_collect<A, B>(iter: impl Iterator<Item = (A, B)>) -> (Vec<A>, Vec<B>) {
    let mut list_a = vec![];
    let mut list_b = vec![];

    for item in iter {
        list_a.push(item.0);
        list_b.push(item.1);
    }
    (list_a, list_b)
}