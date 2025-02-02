// cpu.rs
//

use std::{fs::File, io::Read, path::Path};

use crate::{
    constant::{self, OpcodeSize, RegisterSize},
    memory::Memory,
    opcode::Opcode,
};

enum Data {
    GeneralRegisters,
    SpecialRegisters,
    Ram,
    Rom,
}
pub struct GeneralPurposeRegisters {
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
    pub r19: u64,
    pub r20: u64,
}

pub struct SpecialPurposeRegisters {
    pub pc: u64,
    pub sp: u64,

    pub o1: u64,
    pub o2: u64,
    pub o3: u64,
    pub o4: u64,
    pub o5: u64,
    pub o6: u64,
    pub o7: u64,
    pub o8: u64,
    pub o9: u64,
    pub o10: u64,
}

impl GeneralPurposeRegisters {
    pub fn new() -> Self {
        GeneralPurposeRegisters {
            r1: 0xDEADBEEF,
            r2: 0xDEADBEEF,
            r3: 0xDEADBEEF,
            r4: 0xDEADBEEF,
            r5: 0xDEADBEEF,
            r6: 0xDEADBEEF,
            r7: 0xDEADBEEF,
            r8: 0xDEADBEEF,
            r9: 0xDEADBEEF,
            r10: 0xDEADBEEF,
            r11: 0xDEADBEEF,
            r12: 0xDEADBEEF,
            r13: 0xDEADBEEF,
            r14: 0xDEADBEEF,
            r15: 0xDEADBEEF,
            r16: 0xDEADBEEF,
            r17: 0xDEADBEEF,
            r18: 0xDEADBEEF,
            r19: 0xDEADBEEF,
            r20: 0xDEADBEEF,
        }
    }
}
impl SpecialPurposeRegisters {
    pub fn new() -> Self {
        SpecialPurposeRegisters {
            pc: 0xDEADBEEF,
            sp: 0xDEADBEEF,
            o1: 0xDEADBEEF,
            o2: 0xDEADBEEF,
            o3: 0xDEADBEEF,
            o4: 0xDEADBEEF,
            o5: 0xDEADBEEF,
            o6: 0xDEADBEEF,
            o7: 0xDEADBEEF,
            o8: 0xDEADBEEF,
            o9: 0xDEADBEEF,
            o10: 0xDEADBEEF,
        }
    }
}
pub enum State {
    NoProgramLoaded,
    ProgramLoadedNotStarted,
    ProgramRunning,
    ProgramFatalError,
    ProgramExitedSuccess,
    ProgramHalted,
}
pub struct Runtime {
    pub gpr: GeneralPurposeRegisters,
    pub spr: SpecialPurposeRegisters,
    pub memory: Memory,
    pub state: State,
    pub debug: bool,
}
/// init a new runtime with program loaded
impl Runtime {
    pub fn new(debug: bool) -> Self {
        Runtime {
            memory: Memory::new(),
            gpr: GeneralPurposeRegisters::new(),
            spr: SpecialPurposeRegisters::new(),
            state: State::NoProgramLoaded,
            debug,
        }
    }
    /// returns mutable reference to a register
    fn get_mut_reg(&mut self, reg_bytes: Vec<u8>) -> Result<&mut u64, String> {
        let mut bytes = reg_bytes.clone();
        let reg = bytes_to_u64(&mut bytes)?;
        println!("reg_code: {reg}");
        match reg {
            1 => Ok(&mut self.gpr.r1),
            2 => Ok(&mut self.gpr.r2),
            3 => Ok(&mut self.gpr.r3),
            4 => Ok(&mut self.gpr.r4),
            5 => Ok(&mut self.gpr.r5),
            6 => Ok(&mut self.gpr.r6),
            7 => Ok(&mut self.gpr.r7),
            8 => Ok(&mut self.gpr.r8),
            9 => Ok(&mut self.gpr.r9),
            10 => Ok(&mut self.gpr.r10),
            11 => Ok(&mut self.gpr.r11),
            12 => Ok(&mut self.gpr.r12),
            13 => Ok(&mut self.gpr.r13),
            14 => Ok(&mut self.gpr.r14),
            15 => Ok(&mut self.gpr.r15),
            16 => Ok(&mut self.gpr.r16),
            17 => Ok(&mut self.gpr.r17),
            18 => Ok(&mut self.gpr.r18),
            19 => Ok(&mut self.gpr.r19),
            20 => Ok(&mut self.gpr.r20),
            21 => Ok(&mut self.spr.pc),
            22 => Ok(&mut self.spr.sp),
            23 => Ok(&mut self.spr.o1),
            24 => Ok(&mut self.spr.o2),
            25 => Ok(&mut self.spr.o3),
            26 => Ok(&mut self.spr.o4),
            27 => Ok(&mut self.spr.o5),
            28 => Ok(&mut self.spr.o6),
            29 => Ok(&mut self.spr.o7),
            30 => Ok(&mut self.spr.o8),
            31 => Ok(&mut self.spr.o9),
            32 => Ok(&mut self.spr.o10),
            _ => Err(format!("invalid register code [{reg:#x?}]")),
        }
    }
    /// returns the value inside register
    fn get_reg(&self, reg_bytes: Vec<u8>) -> Result<u64, String> {
        let mut bytes = reg_bytes.clone();
        let reg = bytes_to_u64(&mut bytes)?;
        println!("reg_code: {reg}");
        match reg {
            1 => Ok(self.gpr.r1),
            2 => Ok(self.gpr.r2),
            3 => Ok(self.gpr.r3),
            4 => Ok(self.gpr.r4),
            5 => Ok(self.gpr.r5),
            6 => Ok(self.gpr.r6),
            7 => Ok(self.gpr.r7),
            8 => Ok(self.gpr.r8),
            9 => Ok(self.gpr.r9),
            10 => Ok(self.gpr.r10),
            11 => Ok(self.gpr.r11),
            12 => Ok(self.gpr.r12),
            13 => Ok(self.gpr.r13),
            14 => Ok(self.gpr.r14),
            15 => Ok(self.gpr.r15),
            16 => Ok(self.gpr.r16),
            17 => Ok(self.gpr.r17),
            18 => Ok(self.gpr.r18),
            19 => Ok(self.gpr.r19),
            20 => Ok(self.gpr.r20),
            21 => Ok(self.spr.pc),
            22 => Ok(self.spr.sp),
            23 => Ok(self.spr.o1),
            24 => Ok(self.spr.o2),
            25 => Ok(self.spr.o3),
            26 => Ok(self.spr.o4),
            27 => Ok(self.spr.o5),
            28 => Ok(self.spr.o6),
            29 => Ok(self.spr.o7),
            30 => Ok(self.spr.o8),
            31 => Ok(self.spr.o9),
            32 => Ok(self.spr.o10),
            _ => Err(format!("invalid register code [{reg:#x?}]")),
        }
    }
    /// execute runtime at PC
    pub fn exec(&mut self) -> Result<(), String> {
        // program loop
        match self.state {
            State::NoProgramLoaded => return Err("cannot start execution, no program loaded".to_string()),
            State::ProgramFatalError => return Err("cannot start execution, program suffered fatal error, further execution is unpredictable".to_string()),
            State::ProgramExitedSuccess => return Err("cannot start execution, program finished execution".to_string()),
            State::ProgramRunning => return Err("cannot start execution, program already running. how did you get here?!?!".to_string()),

            _ => {
                println!("executing...");
            },
        };
        self.state = State::ProgramRunning;
        loop {
            match self.state {
                State::ProgramRunning => (),
                _ => break,
            }
            match self.step() {
                Ok(()) => (),
                Err(why) => {
                    let error = format!("error in execution :: {}", why);
                    self.state = State::ProgramFatalError;
                    return Err(error);
                }
            }
        }
        Ok(())
    }
    /// step through one cycle
    pub fn step(&mut self) -> Result<(), String> {
        let opcode = self.decode_opcode()?;

        let operation_result = match opcode {
            Opcode::Nop => self.nop(),
            Opcode::Mov => self.op_mov(),
            Opcode::Movim => self.op_movim(),
            Opcode::Load => self.op_load(),
            Opcode::Store => self.op_store(),
            Opcode::Add => self.op_add(),
            Opcode::Sub => self.op_sub(),
            Opcode::Mult => self.op_mult(),
            Opcode::Div => self.op_div(),
            Opcode::End_of_exec_section => self.op_end_of_exec_section(),
        };
        match operation_result {
            Ok(increment) => self.spr.pc += increment as u64,
            Err(runtime_error) => return Err(format!("runtime error: {}", runtime_error)),
        }
        Ok(())
    }

    /// dumps state
    pub fn dump(&self, data: Data) -> String {
        todo!()
    }
    pub fn throw_runtime_error(self, why: &str) {
        println!("runtime error!! :: {}", why)
    }
    pub fn load(&mut self, binary: &Path) -> Result<(), String> {
        // verify signature
        // locate start and end of exec
        // mark start of execution section
        //

        let mut binary_file: File = match File::open(binary) {
            Ok(file) => file,
            Err(why) => return Err(why.to_string()),
        };

        let mut program_signature_buffer = vec![0; constant::SIGNATURE.len()];
        match binary_file.read_exact(&mut program_signature_buffer) {
            Ok(_) => (),
            Err(why) => {
                let error = format!("could not read signature :: {}", why);
                return Err(error);
            }
        };

        let program_signature = match String::from_utf8(program_signature_buffer) {
            Ok(string) => string,
            Err(why) => {
                let error = format!("could not convert signature to string :: {}", why);
                return Err(error);
            }
        };

        if constant::SIGNATURE != program_signature {
            let why = format!(
                "exec format error: signature not valid, {} != {}",
                constant::SIGNATURE,
                program_signature
            );
            return Err(why);
        } else {
            println!("valid exec format");
        }

        // signature verified -- gonna move this to after loading into memory

        let mut binary_image: Vec<u8> = vec![];
        match binary_file.read_to_end(&mut binary_image) {
            Ok(_) => (),
            Err(why) => {
                let error = format!("failed to read file into rom :: {}", why);
                return Err(error);
            }
        };
        let header_len = 8 * 2;
        if binary_image.len() < header_len {
            return Err(format!(
                "{} formatted file has incomplete header {} bytes, expected {} bytes ",
                constant::SIGNATURE,
                binary_image.len(),
                header_len,
            ));
        }

        // image loaded into memory and verified to have a full header

        let mut head = 0;
        // read header data -- VVV --
        //
        // read data length
        // first u64 after the signature is size of data section in bytes

        let data_rom_length = u64::from_le_bytes(match &binary_image[head..head + 8].try_into() {
            Ok(array) => *array,
            Err(why) => {
                let error = format!("failed to read datarom length :: {}", why);
                return Err(error);
            }
        });
        println!("data_rom_length = {}", data_rom_length);
        head += 8; // pass the datarom length
                   // read exec length
                   // next 8 bytes after datarom length

        let program_length = u64::from_le_bytes(match &binary_image[head..head + 8].try_into() {
            Ok(array) => *array,
            Err(why) => {
                let error = format!("failed to read execrom length :: {}", why);
                return Err(error);
            }
        });
        head += 8;
        // data image and program image length u64s read successfully

        println!("data_length/initram_size = {}", data_rom_length);
        println!("program_length = {}", program_length);
        println!("rom_base = {:#x?}", self.memory.program_base);
        println!("ram_base = {:#x?}", self.memory.ram_base);

        self.memory.program = binary_image[head + data_rom_length as usize
            ..head + data_rom_length as usize + program_length as usize]
            .to_vec();
        self.memory.ram = binary_image[head..head + data_rom_length as usize].to_vec();
        // program and ram now loaded

        self.memory.ram_base = constant::MMIO_ADDRESS_SPACE as u64 + program_length; // program/ram address boundary
        self.spr.pc = self.memory.program_base;
        self.state = State::ProgramLoadedNotStarted;
        Ok(())
    }

    fn decode_opcode(&self) -> Result<Opcode, String> {
        let opcode_bytes = self
            .memory
            .read_bytes(self.spr.pc, constant::OPCODE_BYTES)?;
        let opcode_code = OpcodeSize::from_le_bytes(match opcode_bytes.try_into() {
            Ok(array) => array,
            Err(why) => {
                let error = format!("failed to read datarom length :: {:?}", why);
                return Err(error);
            }
        });
        match opcode_code.try_into() {
            Ok(opcode) => {
                println!("decoded {opcode:?}");
                Ok(opcode)
            }
            Err(()) => return Err(format!("opcode {:#x?} not recognized", opcode_code)),
        }
    }
}
// converts a vector of bytes into a u64 and pads if theres not enough errors if too many bytes are passed
fn bytes_to_u64(bytes: &mut Vec<u8>) -> Result<u64, String> {
    if bytes.len() > 8 {
        return Err(format!("too many bytes to pack into a 64bit integer",));
    }
    bytes.resize(8, 0x0);
    let bytes_array: [u8; 8] = match bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(why) => {
            return Err(format!(
                "error building 64bit integer from bytes :: {why:?}"
            ))
        }
    };
    Ok(u64::from_le_bytes(bytes_array))
}
// return of all instructions are Ok(increment program counter),Err(instruction Error)
impl Runtime {
    fn nop(&self) -> Result<usize, String> {
        println!("nop");

        Ok(constant::OPCODE_BYTES)
    }

    fn op_mov(&mut self) -> Result<usize, String> {
        // let operand_bytes = self.fetch_operand_bytes(constant::REGISTER_BYTES * 2)?;
        let bytes_read = constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2;
        let operand_bytes = self.memory.read_bytes(self.spr.pc, bytes_read)?;

        let src_reg = self.get_reg(
            operand_bytes[constant::OPCODE_BYTES + constant::REGISTER_BYTES
                ..constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2]
                .to_vec(),
        )?;
        println!("src_reg: {src_reg:#x?}");
        let dest_reg = self.get_mut_reg(
            operand_bytes
                [constant::OPCODE_BYTES..constant::OPCODE_BYTES + constant::REGISTER_BYTES]
                .to_vec(),
        )?;
        println!("dest_reg: {dest_reg:#x?}");
        *dest_reg = src_reg;

        Ok(bytes_read)
    }
    // movim dest_reg,imm (assembler places a byte before imm to indicate its size)
    fn op_movim(&mut self) -> Result<usize, String> {
        let bytes_read = constant::OPCODE_BYTES + constant::REGISTER_BYTES + 1;
        let operand_bytes = self.memory.read_bytes(self.spr.pc, bytes_read)?;
        let size: usize = *operand_bytes
            .get(constant::OPCODE_BYTES + constant::REGISTER_BYTES)
            .ok_or("could not read immediate size")? as usize;
        if size > 8 || size == 0 {
            return Err(format!(
                "immediate is too large to load into register :: {}",
                size
            ));
        }
        let mut immediate = self.memory.read_bytes(
            self.spr.pc + constant::OPCODE_BYTES as u64 + constant::REGISTER_BYTES as u64 + 1,
            size,
        )?;
        println!("imm {immediate:?}");
        let immediate_u64 = bytes_to_u64(&mut immediate)?;
        println!("imm_u64 {immediate_u64}");
        let dest_bytes = operand_bytes
            [constant::OPCODE_BYTES..constant::OPCODE_BYTES + constant::REGISTER_BYTES]
            .to_vec();
        let dest_reg = self.get_mut_reg(dest_bytes)?;
        *dest_reg = immediate_u64;
        Ok(bytes_read + size)
    }
    /// `load r1,r2,buffer`
    /// - r1 -- dest register
    /// - r2 -- size to read in (up to 8 bytes)
    /// - buffer -- start of memory address range
    fn op_load(&mut self) -> Result<usize, String> {
        let bytes_read =
            constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2 + constant::ADDRESS_BYTES; // reg,reg,addr
        let operand_bytes = self.memory.read_bytes(self.spr.pc, bytes_read)?;
        let address: u64 = Memory::address_from_bytes(
            &operand_bytes[constant::REGISTER_BYTES * 2
                ..constant::REGISTER_BYTES * 2 + constant::ADDRESS_BYTES], // 4..12
        )?;
        let size = self.get_reg(
            operand_bytes[constant::REGISTER_BYTES..constant::REGISTER_BYTES * 2].to_vec(), // 2..4
        )? as usize;
        if size > 8 {
            return Err(format!(
                "requested bytes are too large to read into register :: {size} bytes cannot fit into an 8 byte register"
            ));
        }
        let src_val =
            Memory::address_from_bytes(self.memory.read_bytes(address, size)?.as_slice())?;
        // not an address but does the same thing basically
        let dest_reg = self.get_mut_reg(operand_bytes[0..constant::REGISTER_BYTES].to_vec())?; // 0..2

        *dest_reg = src_val;
        Ok(bytes_read)
    }
    fn op_store(&mut self) -> Result<usize, String> {
        let bytes_read = constant::ADDRESS_BYTES + constant::REGISTER_BYTES * 2;
        // addr,reg,reg
        let operand_bytes = self.memory.read_bytes(self.spr.pc, bytes_read)?;
        let size = self.get_reg(
            operand_bytes[constant::OPCODE_BYTES + constant::REGISTER_BYTES
                ..constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2] // 2+2..2+2+2 4..6
                .to_vec(),
        );
        let address = Memory::address_from_bytes(
            &operand_bytes[constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2
                ..constant::OPCODE_BYTES + constant::REGISTER_BYTES * 2 + constant::ADDRESS_BYTES],
        )?; // 2+(2*2).. 6..14
        let src =
            self.get_reg(operand_bytes[constant::OPCODE_BYTES..constant::REGISTER_BYTES].to_vec())?; // 0..2
        let src_bytes = u64::to_le_bytes(src);
        self.memory.write_bytes(address, &src_bytes)?;
        Ok(bytes_read)
    }

    fn op_add(&mut self) -> Result<usize, String> {
        todo!()
    }

    fn op_sub(&mut self) -> Result<usize, String> {
        todo!()
    }

    fn op_mult(&mut self) -> Result<usize, String> {
        todo!()
    }

    fn op_div(&mut self) -> Result<usize, String> {
        todo!()
    }
    fn op_end_of_exec_section(&mut self) -> Result<usize, String> {
        println!("end_of_exec_section");
        self.state = State::ProgramExitedSuccess;
        Ok(0)
    }
}
