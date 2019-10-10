// This file is automatically @generated by ddl 0.1.0
// It is not intended for manual editing.

//! Test referring to aliases in struct fields.

#[derive(Copy, Clone)]
pub struct Pair {
    pub first: u8,
    pub second: u8,
}

impl ddl_rt::Binary for Pair {
    type Host = Pair;
}

impl<'data> ddl_rt::ReadBinary<'data> for Pair {
    fn read(ctxt: &mut ddl_rt::ReadCtxt<'data>) -> Result<Pair, ddl_rt::ReadError> {
        let first = ctxt.read::<ddl_rt::U8>()?;
        let second = ctxt.read::<ddl_rt::U8>()?;

        Ok(Pair {
            first,
            second,
        })
    }
}

pub type MyPair = Pair;

#[derive(Copy, Clone)]
pub struct PairPair {
    pub first: Pair,
    pub second: Pair,
}

impl ddl_rt::Binary for PairPair {
    type Host = PairPair;
}

impl<'data> ddl_rt::ReadBinary<'data> for PairPair {
    fn read(ctxt: &mut ddl_rt::ReadCtxt<'data>) -> Result<PairPair, ddl_rt::ReadError> {
        let first = ctxt.read::<Pair>()?;
        let second = ctxt.read::<MyPair>()?;

        Ok(PairPair {
            first,
            second,
        })
    }
}
