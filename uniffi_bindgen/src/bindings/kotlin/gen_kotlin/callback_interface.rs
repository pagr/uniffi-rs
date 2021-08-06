/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;

use crate::bindings::backend::{
    CodeType, LanguageOracle, Literal, MemberDeclaration, TypeIdentifier,
};
use crate::interface::{CallbackInterface, ComponentInterface};
use askama::Template;

use super::filters;
pub struct CallbackInterfaceCodeType {
    id: String,
}

impl CallbackInterfaceCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    fn internals(&self, oracle: &dyn LanguageOracle) -> String {
        format!("{}Internals", self.canonical_name(oracle))
    }
}

impl CodeType for CallbackInterfaceCodeType {
    fn type_label(&self, oracle: &dyn LanguageOracle) -> String {
        oracle.class_name(&self.id)
    }

    fn canonical_name(&self, oracle: &dyn LanguageOracle) -> String {
        format!("CallbackInterface{}", self.type_label(oracle))
    }

    fn literal(&self, _oracle: &dyn LanguageOracle, _literal: &Literal) -> String {
        unreachable!();
    }

    fn lower(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> String {
        format!("{}.lower({})", self.internals(oracle), oracle.var_name(nm))
    }

    fn write(
        &self,
        oracle: &dyn LanguageOracle,
        nm: &dyn fmt::Display,
        target: &dyn fmt::Display,
    ) -> String {
        format!(
            "{}.write({}, {})",
            self.internals(oracle),
            oracle.var_name(nm),
            target
        )
    }

    fn lift(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> String {
        format!(
            "{}.lift({}, {})",
            self.internals(oracle),
            self.type_label(oracle),
            nm
        )
    }

    fn read(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> String {
        format!(
            "{}.read({}, {})",
            self.internals(oracle),
            self.type_label(oracle),
            nm
        )
    }

    fn helper_code(&self, oracle: &dyn LanguageOracle) -> Option<String> {
        Some(format!(
            "// Helper code for {} callback interface is found in CallbackInterfaceTemplate.kt",
            self.type_label(oracle)
        ))
    }
}

#[derive(Template)]
#[template(syntax = "kt", escape = "none", path = "CallbackInterfaceTemplate.kt")]
pub struct KotlinCallbackInterface {
    inner: CallbackInterface,
    contains_unsigned_types: bool,
}

impl KotlinCallbackInterface {
    pub fn new(inner: CallbackInterface, ci: &ComponentInterface) -> Self {
        Self {
            contains_unsigned_types: inner.contains_unsigned_types(ci),
            inner,
        }
    }
    pub fn inner(&self) -> &CallbackInterface {
        &self.inner
    }
    pub fn contains_unsigned_types(&self) -> bool {
        self.contains_unsigned_types
    }
}

impl MemberDeclaration for KotlinCallbackInterface {
    fn type_identifier(&self) -> TypeIdentifier {
        TypeIdentifier::CallbackInterface(self.inner.name().into())
    }

    fn initialization_code(&self, oracle: &dyn LanguageOracle) -> Option<String> {
        let code_type = CallbackInterfaceCodeType::new(self.inner.name().into());
        Some(format!("{}.register(lib)", code_type.internals(oracle)))
    }

    fn definition_code(&self, _oracle: &dyn LanguageOracle) -> Option<String> {
        Some(self.render().unwrap())
    }

    fn import_code(&self, _oracle: &dyn LanguageOracle) -> Option<Vec<String>> {
        Some(
            vec![
                "java.util.concurrent.locks.ReentrantLock",
                "kotlin.concurrent.withLock",
            ]
            .into_iter()
            .map(|s| s.into())
            .collect(),
        )
    }
}