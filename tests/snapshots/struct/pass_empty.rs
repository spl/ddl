// This file is automatically @generated by ddl 0.1.0
// It is not intended for manual editing.

//! Test an empty struct.

#[derive(Copy, Clone)]
pub struct Empty {}

impl ddl_rt::Format for Empty {
    type Host = Empty;
}

impl<'data> ddl_rt::ReadFormat<'data> for Empty {
    fn read(_: &mut ddl_rt::FormatReader<'data>) -> Result<Empty, ddl_rt::ReadError> {
        Ok(Empty {})
    }
}
