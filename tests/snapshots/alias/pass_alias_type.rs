// This file is automatically @generated by ddl 0.1.0
// It is not intended for manual editing.

/// Test that one can refer to local type aliases in aliases.
pub type Foo = ddl_rt::U32Be;

pub type Bar = Foo;
