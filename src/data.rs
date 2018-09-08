struct InsnDesc {
    mnemonic: &'static str,
    syntax: Syntax,
    operands: &'static [Operand], // opcode doesn't count
    opcode: u16,
}

enum Syntax {
    Normal,
    MoviwwiMM,
    MoviwwiOffset,
}

struct Operand {
    field_idx: u8, // lsb to msb
    kind: OperandKind,
}

enum OperandKind {
    F, // register
    D, // destination (F or W)
    B, // bit index
    K(u8), // integer
    UK(u8), // unsigned integer
    SK(u8), // signed integer
    UPK(u16), // program memory address
    SPK(u16), // program memory address
    TRIS, // TRIS register
    FSRn, // FSR register
    MM, // pre/post inc/dec
}

use data::OperandKind::*;

static F_OPERANDS: &[Operand] = &[
    Operand { field_idx: 0, kind: F },
];

static FD_OPERANDS: &[Operand] = &[
    Operand { field_idx: 0, kind: F },
    Operand { field_idx: 1, kind: D },
];

static FB_OPERANDS: &[Operand] = &[
    Operand { field_idx: 0, kind: F },
    Operand { field_idx: 1, kind: B },
];

static K8_OPERANDS: &[Operand] = &[
    Operand { field_idx: 0, kind: K(8) },
];

static INSN_DESCS: &[InsnDesc] = &[
    InsnDesc {
        mnemonic: "addwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0111_0000_0000,
    },
];
