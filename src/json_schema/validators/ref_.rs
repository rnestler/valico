use serialize::json;
use url;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Ref {
    pub url: url::Url
}

impl super::Validator for Ref {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, scope: &scope::Scope) -> super::ValidationState {
        let schema = scope.resolve(&self.url);

        if schema.is_some() {
            schema.unwrap().validate_in(val, path)
        } else {
            let mut state = super::ValidationState::new();
            state.missing.push(self.url.clone());
            state
        }
    }
}