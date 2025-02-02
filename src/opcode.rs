// opcode.rs
// opcode defintitions
// pub type OpcodeRegistry = HashMap<String, Opcode>;

// #[derive(Debug, Clone)]
// pub struct Opcode {
//     name: String,
//     code: u16,
//     fields: usize,
// }
//
use crate::constant::OpcodeSize;
macro_rules! decode {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<OpcodeSize> for $name {
            type Error = ();

            fn try_from(v: OpcodeSize) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as OpcodeSize => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}
decode!(
    #[derive(Debug)]
    pub enum Opcode {
        Nop = 0x00 as isize,
        Mov = 0x01 as isize,
        Movim = 0x02 as isize,
        Load = 0x03 as isize,
        Store = 0x04 as isize,
        Add = 0x05 as isize,
        Sub = 0x06 as isize,
        Mult = 0x07 as isize,
        Div = 0x08 as isize,

        End_of_exec_section = 0xFFFF as isize,
    }
);
