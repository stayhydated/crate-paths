use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// yanked from rustdoc-types (0.46.1)
/// because i needed to derive strum on it
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ItemKind {
    /// A module declaration, e.g. `mod foo;` or `mod foo {}`
    Module,
    /// A crate imported via the `extern crate` syntax.
    ExternCrate,
    /// An import of 1 or more items into scope, using the `use` keyword.
    Use,
    /// A `struct` declaration.
    Struct,
    /// A field of a struct.
    StructField,
    /// A `union` declaration.
    Union,
    /// An `enum` declaration.
    Enum,
    /// A variant of a enum.
    Variant,
    /// A function declaration, e.g. `fn f() {}`
    Function,
    /// A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`
    TypeAlias,
    /// The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`
    Constant,
    /// A `trait` declaration.
    Trait,
    /// A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`
    ///
    /// See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)
    TraitAlias,
    /// An `impl` block.
    Impl,
    /// A `static` declaration.
    Static,
    /// `type`s from an `extern` block.
    ///
    /// See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)
    ExternType,
    /// A macro declaration.
    ///
    /// Corresponds to either `ItemEnum::Macro(_)`
    /// or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`
    Macro,
    /// A procedural macro attribute.
    ///
    /// Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`
    ProcAttribute,
    /// A procedural macro usable in the `#[derive()]` attribute.
    ///
    /// Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`
    ProcDerive,
    /// An associated constant of a trait or a type.
    AssocConst,
    /// An associated type of a trait or a type.
    AssocType,
    /// A primitive type, e.g. `u32`.
    ///
    /// [`Item`]s of this kind only come from the core library.
    Primitive,
    /// A keyword declaration.
    ///
    /// [`Item`]s of this kind only come from the come library and exist solely
    /// to carry documentation for the respective keywords.
    Keyword,
}
