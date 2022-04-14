use core::mem::transmute;

use crate::rsp::dmem_writer::DMEMWriter;

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub enum GPR {
    R0 = 0, AT = 1, V0 = 2, V1 = 3, A0 = 4, A1 = 5, R2 = 6, R3 = 7,
    T0 = 8, T1 = 9, T2 = 10, T3 = 11, T4 = 12, T5 = 13, T6 = 14, T7 = 15,
    S0 = 16, S1 = 17, S2 = 18, S3 = 19, S4 = 20, S5 = 21, S6 = 22, S7 = 23,
    T8 = 24, T9 = 25, K0 = 26, K1 = 27, GP = 28, SP = 29, S8 = 30, RA = 31,
}
// @formatter:on

impl GPR {
    pub fn from_index(index: usize) -> Option<Self> {
        if index <= 31 {
            Some(unsafe { transmute(index as u8) })
        } else {
            None
        }
    }
}

impl core::iter::Step for GPR {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if (*start as usize) < (*end as usize) {
            Some(*end as usize - *start as usize)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Self::from_index(start as usize + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Self::from_index(start as usize - count)
    }
}

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum VR {
    V0 = 0, V1 = 1, V2 = 2, V3 = 3, V4 = 4, V5 = 5, V6 = 6, V7 = 7,
    V8 = 8, V9 = 9, V10 = 10, V11 = 11, V12 = 12, V13 = 13, V14 = 14, V15 = 15,
    V16 = 16, V17 = 17, V18 = 18, V19 = 19, V20 = 20, V21 = 21, V22 = 22, V23 = 23,
    V24 = 24, V25 = 25, V26 = 26, V27 = 27, V28 = 28, V29 = 29, V30 = 30, V31 = 31,
}
// @formatter:on

impl VR {
    pub fn from_index(i: u32) -> Self {
        assert!(i < 32);
        unsafe { transmute(i as u8) }
    }
}

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Element {
    All = 0, All1 = 1,
    Q0 = 2, Q1 = 3,
    H0 = 4, H1 = 5, H2 = 6, H3 = 7,
    _0 = 8, _1 = 9, _2 = 10, _3 = 11, _4 = 12, _5 = 13, _6 = 14, _7 = 15,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum E {
    _0 = 0, _1 = 1, _2 = 2, _3 = 3, _4 = 4, _5=5, _6=6, _7=7,
    _8 = 8, _9 = 9, _10 = 10, _11 = 11, _12 = 12, _13 = 13, _14 = 14, _15 = 15,
}
// @formatter:on

impl E {
    pub fn from_index(i: u32) -> Self {
        assert!(i < 32);
        unsafe { transmute(i as u8) }
    }
}

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum OP {
    SPECIAL = 0, REGIMM = 1, J = 2, JAL = 3, BEQ = 4, BNE = 5, BLEZ = 6, BGTZ = 7,
    ADDI = 8, ADDIU = 9, SLTI = 10, SLTIU = 11, ANDI = 12, ORI = 13, XORI = 14, LUI = 15,
    COP0 = 16, COP2 = 18,
    LB = 32, LH = 33, LW = 35, LBU = 36, LHU = 37, LWU = 39, SB = 40, SH = 41, SW = 43,
    LWC2 = 50, SWC2 = 58,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum SpecialOP {
    SLL = 0, SRL = 2, SRA = 3, SLLV = 4, SRLV = 6, SRAV = 7,
    JR = 8, JALR = 9,
    BREAK = 13,
    ADD = 32, ADDU = 33, SUB = 34, SUBU = 35,
    AND = 36, OR = 37, XOR = 38, NOR = 39,
    SLT = 42, SLTU = 43,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum CP0OP {
    MFC0 = 0, MTC0 = 4,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
pub enum CP0Register {
    SPAddress = 0, DRAMAddress = 1, ReadLength = 2, WriteLength = 3, SPStatus = 4, Semaphore = 7,
    DPStart = 8, DPEnd = 9, DPStatus = 11, DPClock = 12
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum WC2OP {
    BV = 0, SV = 1, LV = 2, DV = 3, QV = 4, RV = 5, PV = 6, UV = 7, HV = 8, FV = 9, WV = 10, TV = 11,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum CP2OP {
    MFC2 = 0, CFC2 = 2, MTC2 = 4, CTC2 = 6, VECTOR = 16,
}
// @formatter:on

// @formatter:off
#[allow(dead_code)]
#[repr(u8)]
enum VectorOp {
    VMULF = 0, VMULU = 1, VRNDP = 2, VMULQ = 3, VMUDL = 4, VMUDM = 5, VMUDN = 6, VMUDH = 7, VMACF = 8, VMACU = 9, VRNDN = 10, VMACQ = 11, VMADL = 12, VMADM = 13, VMADN = 14, VMADH = 15,
    VADD = 16, VSUB = 17, VSUT = 18, VABS = 19, VADDC = 20, VSUBC = 21, VADDB = 22, VSUBB = 23, VACCB = 24, VSUCB = 25, VSAD = 26, VSAC = 27, VSUM = 28, VSAR = 29,
    VLT = 32, VEQ = 33, VNE = 34, VGE = 35, VCL = 36, VCH = 37, VCR = 38, VMRG = 39, VAND = 40, VNAND = 41, VOR = 42, VNOR = 43, VXOR = 44, VNXOR = 45,
    VRCP = 48, VRCPL = 49, VRCPH = 50, VMOV = 51, VRSQ = 52, VRSQL = 53, VRSQH = 54, VNOOP = 55, VEXTT = 56, VEXTQ = 57, VEXTN = 58, VINST = 60, VINSQ = 61, VINSN = 62,
}
// @formatter:on

pub struct RSMAssemblerJumpTarget {
    offset: usize,
}

impl RSMAssemblerJumpTarget {
    pub fn new(offset: usize) -> Self { Self { offset } }
}

pub struct RSPAssembler {
    writer: DMEMWriter,
}

impl RSPAssembler {
    pub const fn new(start_offset: usize) -> Self {
        // IMEM starts at 0x1000
        Self { writer: DMEMWriter::new(start_offset) }
    }

    pub fn get_jump_target(&self) -> RSMAssemblerJumpTarget {
        RSMAssemblerJumpTarget::new(self.writer.offset())
    }

    fn write_main_immediate(&mut self, op: OP, rt: GPR, rs: GPR, imm: u16) {
        let instruction: u32 =
            (imm as u32) |
                ((rt as u32) << 16) |
                ((rs as u32) << 21) |
                ((op as u32) << 26);
        self.writer.write(instruction);
    }

    fn write_main_jump(&mut self, op: OP, jump_target_shifted_by_2: u32) {
        assert!(jump_target_shifted_by_2 < (1 << 26));
        let instruction: u32 =
            jump_target_shifted_by_2 |
                ((op as u32) << 26);
        self.writer.write(instruction);
    }

    fn write_special(&mut self, function: SpecialOP, sa: u32, rd: GPR, rs: GPR, rt: GPR) {
        assert!(sa <= 0b11111);
        self.writer.write((function as u32) |
            (sa << 6) |
            ((rd as u32) << 11) |
            ((rt as u32) << 16) |
            ((rs as u32) << 21) |
            ((OP::SPECIAL as u32) << 26));
    }

    fn write_cop0(&mut self, cp0op: CP0OP, cp0register: CP0Register, rt: GPR) {
        let instruction: u32 =
            ((cp0register as u32) << 11) |
                ((rt as u32) << 16) |
                ((cp0op as u32) << 21) |
                ((OP::COP0 as u32) << 26);
        self.writer.write(instruction);
    }

    fn write_wc2(&mut self, op: OP, wc2op: WC2OP, vt: VR, element: E, imm7: i32, base: GPR) {
        assert!(imm7 <= 63 && imm7 >= -64);
        let instruction: u32 =
            ((imm7 as u32) & 0b111_1111) |
                ((element as u32) << 7) |
                ((wc2op as u32) << 11) |
                ((vt as u32) << 16) |
                ((base as u32) << 21) |
                ((op as u32) << 26);
        self.writer.write(instruction);
    }

    fn write_vector(&mut self, vector_op: VectorOp, vd: VR, vt: VR, vs: VR, e: Element) {
        // CP2OP::VECTOR has a bunch of 0 bits at the bottom, which are being reused for e. That explains the strange encoding
        let instruction: u32 =
            (vector_op as u32) |
                ((vd as u32) << 6) |
                ((vs as u32) << 11) |
                ((vt as u32) << 16) |
                ((e as u32) << 21) |
                ((CP2OP::VECTOR as u32) << 21) |
                ((OP::COP2 as u32) << 26);
        self.writer.write(instruction);
    }

    fn write_vector_e(&mut self, vector_op: VectorOp, vd: VR, vt: VR, vs: VR, e: E) {
        // CP2OP::VECTOR has a bunch of 0 bits at the bottom, which are being reused for e. That explains the strange encoding
        let instruction: u32 =
            (vector_op as u32) |
                ((vd as u32) << 6) |
                ((vs as u32) << 11) |
                ((vt as u32) << 16) |
                ((e as u32) << 21) |
                ((CP2OP::VECTOR as u32) << 21) |
                ((OP::COP2 as u32) << 26);
        self.writer.write(instruction);
    }

    // Main instructions
    pub fn write_addi(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::ADDI, rt, rs, imm as u16);
    }

    pub fn write_addiu(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::ADDIU, rt, rs, imm as u16);
    }

    pub fn write_andi(&mut self, rt: GPR, rs: GPR, imm: u16) {
        self.write_main_immediate(OP::ANDI, rt, rs, imm);
    }

    pub fn write_lb(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LB, rt, rs, imm as u16);
    }

    pub fn write_lbu(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LBU, rt, rs, imm as u16);
    }

    pub fn write_lh(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LH, rt, rs, imm as u16);
    }

    pub fn write_lhu(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LHU, rt, rs, imm as u16);
    }

    pub fn write_lw(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LW, rt, rs, imm as u16);
    }

    pub fn write_lwu(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::LWU, rt, rs, imm as u16);
    }

    pub fn write_sb(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::SB, rt, rs, imm as u16);
    }

    pub fn write_sh(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::SH, rt, rs, imm as u16);
    }

    pub fn write_slti(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::SLTI, rt, rs, imm as u16);
    }

    pub fn write_sltiu(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::SLTIU, rt, rs, imm as u16);
    }

    pub fn write_sw(&mut self, rt: GPR, rs: GPR, imm: i16) {
        self.write_main_immediate(OP::SW, rt, rs, imm as u16);
    }

    /// Writes a 32 bit value into the target register. Uses 1 or 2 instructions
    pub fn write_li(&mut self, rt: GPR, imm: u32) {
        if (imm & 0xFFFF0000) != 0 {
            self.write_lui(rt, (imm >> 16) as u16);
            if (imm & 0xFFFF) != 0 {
                self.write_ori(rt, rt, imm as u16);
            }
        } else {
            self.write_ori(rt, GPR::R0, imm as u16);
        }
    }

    pub fn write_lui(&mut self, rt: GPR, imm: u16) {
        self.write_main_immediate(OP::LUI, rt, GPR::R0, imm);
    }

    pub fn write_ori(&mut self, rt: GPR, rs: GPR, imm: u16) {
        self.write_main_immediate(OP::ORI, rt, rs, imm);
    }

    pub fn write_xori(&mut self, rt: GPR, rs: GPR, imm: u16) {
        self.write_main_immediate(OP::XORI, rt, rs, imm);
    }

    pub fn write_j(&mut self, destination: u32) {
        assert!((destination & 3) == 0);
        self.write_main_jump(OP::J, destination >> 2);
    }

    pub fn write_jal(&mut self, destination: u32) {
        assert!((destination & 3) == 0);
        self.write_main_jump(OP::JAL, destination >> 2);
    }

    pub fn write_bgtz(&mut self, rs: GPR, offset: i16) {
        self.write_main_immediate(OP::BGTZ, GPR::R0, rs, offset as u16);
    }

    pub fn write_bgtz_backwards(&mut self, rs: GPR, target: &RSMAssemblerJumpTarget) {
        let offset = (((target.offset - self.writer.offset()) & 0xFFF) >> 2) - 1;
        self.write_bgtz(rs, offset as i16);
    }

    pub fn write_mtc0(&mut self, cp0register: CP0Register, rt: GPR) {
        self.write_cop0(CP0OP::MTC0, cp0register, rt);
    }

    // Special instructions
    pub fn write_sll(&mut self, rd: GPR, rt: GPR, sa: u32) {
        self.write_special(SpecialOP::SLL, sa, rd, GPR::R0, rt);
    }
    pub fn write_nop(&mut self) {
        self.write_sll(GPR::R0, GPR::R0, 0);
    }

    pub fn write_break(&mut self) {
        self.write_special(SpecialOP::BREAK, 0, GPR::R0, GPR::R0, GPR::R0);
    }

    pub fn write_or(&mut self, rd: GPR, rs: GPR, rt: GPR) {
        self.write_special(SpecialOP::OR, 0, rd, rs, rt);
    }

    // Vector load/store instructions
    pub fn write_lqv(&mut self, vt: VR, element: E, offset: i32, base: GPR) {
        assert!((offset & 0b1111) == 0);
        self.write_wc2(OP::LWC2, WC2OP::QV, vt, element, offset >> 4, base);
    }

    pub fn write_sqv(&mut self, vt: VR, element: E, offset: i32, base: GPR) {
        assert!((offset & 0b1111) == 0);
        self.write_wc2(OP::SWC2, WC2OP::QV, vt, element, offset >> 4, base);
    }

    pub fn write_vmacf(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMACF, vd, vt, vs, e);
    }

    pub fn write_vmadh(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMADH, vd, vt, vs, e);
    }

    pub fn write_vmadm(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMADM, vd, vt, vs, e);
    }

    pub fn write_vmadn(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMADN, vd, vt, vs, e);
    }

    pub fn write_vmudh(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMUDH, vd, vt, vs, e);
    }

    pub fn write_vmudn(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMUDN, vd, vt, vs, e);
    }

    pub fn write_vmudm(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMUDM, vd, vt, vs, e);
    }

    pub fn write_vmulf(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VMULF, vd, vt, vs, e);
    }

    pub fn write_vsar(&mut self, vd: VR, vt: VR, vs: VR, e: E) {
        self.write_vector_e(VectorOp::VSAR, vd, vt, vs, e);
    }

    pub fn write_vxor(&mut self, vd: VR, vt: VR, vs: VR, e: Element) {
        self.write_vector(VectorOp::VXOR, vd, vt, vs, e);
    }
}
