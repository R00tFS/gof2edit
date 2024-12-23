use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use crate::error::Error;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

const TERRAN_CODE: u32 = 0;
const VOSSK_CODE: u32 = 1;
const NIVELIAN_CODE: u32 = 2;
const MIDORIAN_CODE: u32 = 3;
const OCTOPOD_CODE: u32 = 4;
const CYBORG_CODE: u32 = 5;
const BOBOLIAN_CODE: u32 = 6;
const GREY_CODE: u32 = 7;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Faction {
    Terran,
    Vossk,
    Nivelian,
    Midorian,
    Octopod,
    Cyborg,
    Bobolian,
    Grey,
}

impl BinRead for Faction {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        match source.read_u32::<O>()? {
            TERRAN_CODE => Ok(Self::Terran),
            VOSSK_CODE => Ok(Self::Vossk),
            NIVELIAN_CODE => Ok(Self::Nivelian),
            MIDORIAN_CODE => Ok(Self::Midorian),
            OCTOPOD_CODE => Ok(Self::Octopod),
            CYBORG_CODE => Ok(Self::Cyborg),
            BOBOLIAN_CODE => Ok(Self::Bobolian),
            GREY_CODE => Ok(Self::Grey),
            value => Err(Error::FactionParse(value)),
        }
    }
}

impl BinWrite for Faction {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_u32::<O>(match self {
            Self::Terran => TERRAN_CODE,
            Self::Vossk => VOSSK_CODE,
            Self::Nivelian => NIVELIAN_CODE,
            Self::Midorian => MIDORIAN_CODE,
            Self::Octopod => OCTOPOD_CODE,
            Self::Cyborg => CYBORG_CODE,
            Self::Bobolian => BOBOLIAN_CODE,
            Self::Grey => GREY_CODE,
        })?;

        Ok(())
    }
}
