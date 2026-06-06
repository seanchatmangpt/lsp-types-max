use lsp_types_max::*;
use serde_json::json;

#[test]
fn test_base_any() {
    let val: BaseAny = json!({"key": "value"});
    assert_eq!(val, json!({"key": "value"}));
}

#[test]
fn test_base_object() {
    let mut obj = BaseObject::new();
    obj.insert("key".to_string(), json!("value"));
    let val: BaseAny = BaseAny::Object(obj);
    assert_eq!(val, json!({"key": "value"}));
}

#[test]
fn test_base_array() {
    let arr: BaseArray = vec![json!(1), json!(2)];
    assert_eq!(arr, vec![json!(1), json!(2)]);
}

#[test]
fn test_document_uri() {
    let uri: DocumentUri = "file:///test".parse().unwrap();
    assert_eq!(uri.to_string(), "file:///test");
}

#[test]
fn test_uri_alias() {
    let uri: URI = "file:///test".parse().unwrap();
    assert_eq!(uri.to_string(), "file:///test");
}

#[test]
fn test_base_any_roundtrip() {
    let val: BaseAny = json!([1, 2, 3]);
    let s = serde_json::to_string(&val).unwrap();
    let b: BaseAny = serde_json::from_str(&s).unwrap();
    assert_eq!(val, b);
}

#[test]
fn test_base_object_roundtrip() {
    let mut obj = BaseObject::new();
    obj.insert("a".to_string(), json!(1));
    let s = serde_json::to_string(&obj).unwrap();
    let b: BaseObject = serde_json::from_str(&s).unwrap();
    assert_eq!(obj, b);
}

#[test]
fn test_base_array_roundtrip() {
    let arr: BaseArray = vec![json!("a"), json!("b")];
    let s = serde_json::to_string(&arr).unwrap();
    let b: BaseArray = serde_json::from_str(&s).unwrap();
    assert_eq!(arr, b);
}

#[test]
fn test_document_uri_roundtrip() {
    let uri: DocumentUri = "file:///a/b/c".parse().unwrap();
    let s = serde_json::to_string(&uri).unwrap();
    let b: DocumentUri = serde_json::from_str(&s).unwrap();
    assert_eq!(uri, b);
}

#[test]
fn test_uri_type_equality() {
    let uri: URI = "file:///test".parse().unwrap();
    let doc_uri: DocumentUri = "file:///test".parse().unwrap();
    assert_eq!(uri, doc_uri);
}
