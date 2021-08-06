/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;

use crate::bindings::backend::{
    CodeType, LanguageOracle, Literal, MemberDeclaration, StringReturn, TypeIdentifier,
};
use crate::interface::{ComponentInterface, Object};
use askama::Template;

use super::filters;
pub struct ObjectCodeType {
    id: String,
}

impl ObjectCodeType {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl CodeType for ObjectCodeType {
    fn type_label(&self, oracle: &dyn LanguageOracle) -> StringReturn {
        oracle.class_name(&self.id)
    }

    fn canonical_name(&self, oracle: &dyn LanguageOracle) -> StringReturn {
        format!("Object{}", self.type_label(oracle))
    }

    fn literal(&self, _oracle: &dyn LanguageOracle, _literal: &Literal) -> StringReturn {
        unreachable!();
    }

    fn lower(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> StringReturn {
        format!("{}.lower()", oracle.var_name(nm))
    }

    fn write(
        &self,
        oracle: &dyn LanguageOracle,
        nm: &dyn fmt::Display,
        target: &dyn fmt::Display,
    ) -> StringReturn {
        format!("{}.write({})", oracle.var_name(nm), target)
    }

    fn lift(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> StringReturn {
        format!("{}.lift({})", self.type_label(oracle), nm)
    }

    fn read(&self, oracle: &dyn LanguageOracle, nm: &dyn fmt::Display) -> StringReturn {
        format!("{}.read({})", self.type_label(oracle), nm)
    }

    fn helper_code(&self, oracle: &dyn LanguageOracle) -> Option<String> {
        Some(format!(
            "// Helper code for {} class is found in ObjectTemplate.kt",
            self.type_label(oracle)
        ))
    }

    fn import_code(&self, _oracle: &dyn LanguageOracle) -> Option<Vec<String>> {
        Some(
            vec![
                "java.util.concurrent.atomic.AtomicLong",
                "java.util.concurrent.atomic.AtomicBoolean",
            ]
            .into_iter()
            .map(|s| s.into())
            .collect(),
        )
    }
}

#[derive(Template)]
#[template(syntax = "kt", escape = "none", path = "ObjectTemplate.kt")]
pub struct KotlinObject {
    inner: Object,
    contains_unsigned_types: bool,
}

impl KotlinObject {
    pub fn new(inner: Object, ci: &ComponentInterface) -> Self {
        Self {
            contains_unsigned_types: inner.contains_unsigned_types(ci),
            inner,
        }
    }
    pub fn inner(&self) -> &Object {
        &self.inner
    }
    pub fn contains_unsigned_types(&self) -> bool {
        self.contains_unsigned_types
    }
}

impl MemberDeclaration for KotlinObject {
    fn type_identifier(&self) -> TypeIdentifier {
        TypeIdentifier::Object(self.inner.name().into())
    }

    fn definition_code(&self, _oracle: &dyn LanguageOracle) -> Option<String> {
        Some(self.render().unwrap())
    }
}
