use ry26::LibraryError;

#[test]
fn test_library_error_display() {
    let error = LibraryError::InvalidValue("test error".to_string());
    let error_message = format!("{}", error);
    assert_eq!(error_message, "Invalid value: test error");
}

#[test]
fn test_library_error_debug() {
    let error = LibraryError::InvalidValue("test".to_string());
    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("InvalidValue"));
}
