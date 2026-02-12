use std::str::FromStr;

use crate::*;

#[test]
fn test_template_config_new() {
    let config: TemplateConfig =
        TemplateConfig::new(TemplateType::Controller, "test".to_string(), None);
    assert_eq!(config.template_type, TemplateType::Controller);
    assert_eq!(config.component_name, "test");
    assert_eq!(config.model_sub_type, None);
    assert_eq!(config.base_directory, "./application");
}

#[test]
fn test_template_config_with_model_sub_type() {
    let config: TemplateConfig = TemplateConfig::new(
        TemplateType::Model,
        "test".to_string(),
        Some(ModelSubType::Request),
    );
    assert_eq!(config.template_type, TemplateType::Model);
    assert_eq!(config.model_sub_type, Some(ModelSubType::Request));
}

#[test]
fn test_template_config_clone() {
    let config: TemplateConfig =
        TemplateConfig::new(TemplateType::Service, "test".to_string(), None);
    let cloned: TemplateConfig = config.clone();
    assert_eq!(cloned.template_type, config.template_type);
    assert_eq!(cloned.component_name, config.component_name);
    assert_eq!(cloned.model_sub_type, config.model_sub_type);
    assert_eq!(cloned.base_directory, config.base_directory);
}

#[test]
fn test_template_config_debug() {
    let config: TemplateConfig =
        TemplateConfig::new(TemplateType::Controller, "test".to_string(), None);
    let debug_str: String = format!("{config:?}");
    assert!(debug_str.contains("Controller"));
    assert!(debug_str.contains("test"));
}

#[test]
fn test_template_error_display() {
    let error1: TemplateError = TemplateError::InvalidModelSubType("bad".to_string());
    assert!(error1.to_string().contains("bad"));

    let error2: TemplateError = TemplateError::DirectoryExists("/path".to_string());
    assert!(error2.to_string().contains("/path"));
}

#[test]
fn test_template_error_from_io() {
    let io_error: std::io::Error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
    let template_error: TemplateError = TemplateError::from(io_error);
    assert!(template_error.to_string().contains("IO error"));
}

#[test]
fn test_template_error_debug() {
    let error: TemplateError = TemplateError::InvalidModelSubType("test".to_string());
    let debug_str: String = format!("{error:?}");
    assert!(debug_str.contains("InvalidModelSubType"));
}

#[test]
fn test_template_type_equality() {
    assert_eq!(TemplateType::Controller, TemplateType::Controller);
    assert_ne!(TemplateType::Controller, TemplateType::Service);
}

#[test]
fn test_model_sub_type_equality() {
    assert_eq!(ModelSubType::Request, ModelSubType::Request);
    assert_ne!(ModelSubType::Request, ModelSubType::Response);
}

#[test]
fn test_template_type_debug() {
    let ty: TemplateType = TemplateType::Controller;
    let debug_str: String = format!("{ty:?}");
    assert_eq!(debug_str, "Controller");
}

#[test]
fn test_model_sub_type_debug() {
    let ty: ModelSubType = ModelSubType::Application;
    let debug_str: String = format!("{ty:?}");
    assert_eq!(debug_str, "Application");
}

#[test]
fn test_all_template_types() {
    let _ = TemplateType::Controller;
    let _ = TemplateType::Domain;
    let _ = TemplateType::Exception;
    let _ = TemplateType::Mapper;
    let _ = TemplateType::Model;
    let _ = TemplateType::Repository;
    let _ = TemplateType::Service;
    let _ = TemplateType::Utils;
    let _ = TemplateType::View;
}

#[test]
fn test_all_model_sub_types() {
    let _ = ModelSubType::Application;
    let _ = ModelSubType::Request;
    let _ = ModelSubType::Response;
}

#[test]
fn test_parse_template_type_valid() {
    assert_eq!(
        TemplateType::from_str("controller").ok(),
        Some(TemplateType::Controller)
    );
    assert_eq!(
        TemplateType::from_str("Controller").ok(),
        Some(TemplateType::Controller)
    );
    assert_eq!(
        TemplateType::from_str("CONTROLLER").ok(),
        Some(TemplateType::Controller)
    );
    assert_eq!(
        TemplateType::from_str("domain").ok(),
        Some(TemplateType::Domain)
    );
    assert_eq!(
        TemplateType::from_str("exception").ok(),
        Some(TemplateType::Exception)
    );
    assert_eq!(
        TemplateType::from_str("mapper").ok(),
        Some(TemplateType::Mapper)
    );
    assert_eq!(
        TemplateType::from_str("model").ok(),
        Some(TemplateType::Model)
    );
    assert_eq!(
        TemplateType::from_str("repository").ok(),
        Some(TemplateType::Repository)
    );
    assert_eq!(
        TemplateType::from_str("service").ok(),
        Some(TemplateType::Service)
    );
    assert_eq!(
        TemplateType::from_str("utils").ok(),
        Some(TemplateType::Utils)
    );
    assert_eq!(
        TemplateType::from_str("view").ok(),
        Some(TemplateType::View)
    );
}

#[test]
fn test_parse_template_type_invalid() {
    assert_eq!(TemplateType::from_str("invalid").ok(), None);
    assert_eq!(TemplateType::from_str("").ok(), None);
    assert_eq!(TemplateType::from_str("unknown").ok(), None);
}

#[test]
fn test_parse_model_sub_type_valid() {
    assert_eq!(
        ModelSubType::from_str("application").ok(),
        Some(ModelSubType::Application)
    );
    assert_eq!(
        ModelSubType::from_str("Application").ok(),
        Some(ModelSubType::Application)
    );
    assert_eq!(
        ModelSubType::from_str("APPLICATION").ok(),
        Some(ModelSubType::Application)
    );
    assert_eq!(
        ModelSubType::from_str("request").ok(),
        Some(ModelSubType::Request)
    );
    assert_eq!(
        ModelSubType::from_str("response").ok(),
        Some(ModelSubType::Response)
    );
}

#[test]
fn test_parse_model_sub_type_invalid() {
    assert_eq!(ModelSubType::from_str("invalid").ok(), None);
    assert_eq!(ModelSubType::from_str("").ok(), None);
    assert_eq!(ModelSubType::from_str("unknown").ok(), None);
}
