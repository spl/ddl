// This file is automatically @generated by ddl 0.1.0
// It is not intended for manual editing.

#[derive(Copy, Clone)]
pub struct Foo {
    field_type: ddl_rt::InvalidDataDescription,
    field_true: ddl_rt::InvalidDataDescription,
    field_false: ddl_rt::InvalidDataDescription,
}

impl Foo {
    pub fn field_type(&self) -> ddl_rt::InvalidDataDescription {
        self.field_type
    }

    pub fn field_true(&self) -> ddl_rt::InvalidDataDescription {
        self.field_true
    }

    pub fn field_false(&self) -> ddl_rt::InvalidDataDescription {
        self.field_false
    }
}

impl ddl_rt::Format for Foo {
    type Host = Foo;
}

impl<'data> ddl_rt::ReadFormat<'data> for Foo {
    fn read(reader: &mut ddl_rt::FormatReader<'data>) -> Result<Foo, ddl_rt::ReadError> {
        let field_type = reader.read::<ddl_rt::InvalidDataDescription>()?;
        let field_true = reader.read::<ddl_rt::InvalidDataDescription>()?;
        let field_false = reader.read::<ddl_rt::InvalidDataDescription>()?;

        Ok(Foo {
            field_type,
            field_true,
            field_false,
        })
    }
}
