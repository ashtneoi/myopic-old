#[derive(Clone)]
struct Insn {
    desc: &'static InsnDesc,
    operands: [Opd; 2],
}

#[derive(Clone)]
enum Opd {
    K(u16),
    // TODO: what else?
}

#[derive(Clone)]
struct InsnDesc {
    mnemonic: &'static str,
    syntax: Syntax,
    operands: &'static [OpdDesc], // opcode doesn't count
    opcode: u16,
}

#[derive(Clone, Copy)]
enum Syntax {
    Normal,
    MoviwwiMM,
    MoviwwiOffset,
}

#[derive(Clone, Copy)]
struct OpdDesc {
    field_idx: u8, // lsb to msb
    kind: OpdDescKind,
}

/// Tells the assembler how to turn an operand into bits.
#[derive(Clone, Copy)]
enum OpdDescKind {
    DC(u8), // don't care (default value is zero)
    F, // register
    D, // destination (F or W)
    B, // bit index
    K(u8), // integer (K(n): -(1<<n)+1..(1<<n)-1)
    UK(u8), // unsigned integer (UK(n): 0..(1<<n)-1)
    SK(u8), // signed integer (SK(n): -(1<<(n-1))..(1<<(n-1))-1)
    A, // register bank
    PCLATH,
    APK(u8), // absolute program memory address
    RPK(u8), // relative program memory address
    TRIS, // TRIS register
    FSRn, // FSR register
    MM, // pre/post inc/dec
}

use self::OpdDescKind::*;

static F_OPERANDS: &[OpdDesc] = &[
    OpdDesc { field_idx: 0, kind: F },
];

static FD_OPERANDS: &[OpdDesc] = &[
    OpdDesc { field_idx: 0, kind: F },
    OpdDesc { field_idx: 1, kind: D },
];

static FB_OPERANDS: &[OpdDesc] = &[
    OpdDesc { field_idx: 0, kind: F },
    OpdDesc { field_idx: 1, kind: B },
];

static K8_OPERANDS: &[OpdDesc] = &[
    OpdDesc { field_idx: 0, kind: K(8) },
];

static INSN_DESCS: &[InsnDesc] = &[
    InsnDesc {
        mnemonic: "addwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0111_0000_0000,
    },
    InsnDesc {
        mnemonic: "addwfc",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b11_1101_0000_0000,
    },
    InsnDesc {
        mnemonic: "andwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0101_0000_0000,
    },
    InsnDesc {
        mnemonic: "asrf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b11_0111_0000_0000,
    },
    InsnDesc {
        mnemonic: "lslf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b11_0101_0000_0000,
    },
    InsnDesc {
        mnemonic: "lsrf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b11_0110_0000_0000,
    },
    InsnDesc {
        mnemonic: "clrf",
        syntax: Syntax::Normal,
        operands: F_OPERANDS,
        opcode: 0b00_0001_1000_0000,
    },
    InsnDesc {
        mnemonic: "clrw",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: DC(2) },
        ],
        opcode: 0b00_0001_0000_0000,
    },
    InsnDesc {
        mnemonic: "comf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1001_0000_0000,
    },
    InsnDesc {
        mnemonic: "decf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0011_0000_0000,
    },
    InsnDesc {
        mnemonic: "incf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1010_0000_0000,
    },
    InsnDesc {
        mnemonic: "iorwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0100_0000_0000,
    },
    InsnDesc {
        mnemonic: "movf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1000_0000_0000,
    },
    InsnDesc {
        mnemonic: "movwf",
        syntax: Syntax::Normal,
        operands: F_OPERANDS,
        opcode: 0b00_0000_1000_0000,
    },
    InsnDesc {
        mnemonic: "rlf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1101_0000_0000,
    },
    InsnDesc {
        mnemonic: "rrf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1100_0000_0000,
    },
    InsnDesc {
        mnemonic: "subwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0010_0000_0000,
    },
    InsnDesc {
        mnemonic: "subwfb",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b11_1011_0000_0000,
    },
    InsnDesc {
        mnemonic: "swapf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1110_0000_0000,
    },
    InsnDesc {
        mnemonic: "xorwf",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_0110_0000_0000,
    },

    InsnDesc {
        mnemonic: "decfsz",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1011_0000_0000,
    },
    InsnDesc {
        mnemonic: "incfsz",
        syntax: Syntax::Normal,
        operands: FD_OPERANDS,
        opcode: 0b00_1111_0000_0000,
    },

    InsnDesc {
        mnemonic: "bcf",
        syntax: Syntax::Normal,
        operands: FB_OPERANDS,
        opcode: 0b01_0000_0000_0000,
    },
    InsnDesc {
        mnemonic: "bsf",
        syntax: Syntax::Normal,
        operands: FB_OPERANDS,
        opcode: 0b01_0100_0000_0000,
    },

    InsnDesc {
        mnemonic: "btfsc",
        syntax: Syntax::Normal,
        operands: FB_OPERANDS,
        opcode: 0b01_1000_0000_0000,
    },
    InsnDesc {
        mnemonic: "btfss",
        syntax: Syntax::Normal,
        operands: FB_OPERANDS,
        opcode: 0b01_1100_0000_0000,
    },

    InsnDesc {
        mnemonic: "addlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_1110_0000_0000,
    },
    InsnDesc {
        mnemonic: "andlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_1001_0000_0000,
    },
    InsnDesc {
        mnemonic: "iorlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_1000_0000_0000,
    },
    InsnDesc {
        mnemonic: "movlb",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: A },
        ],
        opcode: 0b00_0000_0010_0000,
    },
    InsnDesc {
        mnemonic: "movlp",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: PCLATH },
        ],
        opcode: 0b11_0001_1000_0000,
    },
    InsnDesc {
        mnemonic: "movlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_0000_0000_0000,
    },
    InsnDesc {
        mnemonic: "sublw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_1100_0000_0000,
    },
    InsnDesc {
        mnemonic: "xorlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_1010_0000_0000,
    },

    InsnDesc {
        mnemonic: "bra",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: RPK(9) },
        ],
        opcode: 0b11_0010_0000_0000,
    },
    InsnDesc {
        mnemonic: "brw",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_1011,
    },
    InsnDesc {
        mnemonic: "call",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: APK(11) },
        ],
        opcode: 0b10_0000_0000_0000,
    },
    InsnDesc {
        mnemonic: "callw",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_1010,
    },
    InsnDesc {
        mnemonic: "goto",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: APK(11) },
        ],
        opcode: 0b10_1000_0000_0000,
    },
    InsnDesc {
        mnemonic: "retfie",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_1001,
    },
    InsnDesc {
        mnemonic: "retlw",
        syntax: Syntax::Normal,
        operands: K8_OPERANDS,
        opcode: 0b11_0100_0000_0000,
    },
    InsnDesc {
        mnemonic: "return",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_1000,
    },

    InsnDesc {
        mnemonic: "clrwdt",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0110_0100,
    },
    InsnDesc {
        mnemonic: "nop",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_0000,
    },
    InsnDesc {
        mnemonic: "reset",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0000_0001,
    },
    InsnDesc {
        mnemonic: "sleep",
        syntax: Syntax::Normal,
        operands: &[],
        opcode: 0b00_0000_0110_0011,
    },
    InsnDesc {
        mnemonic: "tris_a",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: TRIS },
        ],
        opcode: 0b00_0000_0110_0101,
    },
    InsnDesc {
        mnemonic: "tris_b",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: TRIS },
        ],
        opcode: 0b00_0000_0110_0110,
    },
    InsnDesc {
        mnemonic: "tris_c",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 0, kind: TRIS },
        ],
        opcode: 0b00_0000_0110_0111,
    },

    InsnDesc {
        mnemonic: "addfsr",
        syntax: Syntax::Normal,
        operands: &[
            OpdDesc { field_idx: 1, kind: FSRn }, // !!
            OpdDesc { field_idx: 0, kind: SK(6) }, // !!
        ],
        opcode: 0b11_0001_0000_0000,
    },
    InsnDesc {
        mnemonic: "moviw_mm",
        syntax: Syntax::MoviwwiMM,
        operands: &[
            OpdDesc { field_idx: 1, kind: FSRn }, // !!
            OpdDesc { field_idx: 0, kind: MM }, // !!
        ],
        opcode: 0b00_0000_0001_0000,
    },
    InsnDesc {
        mnemonic: "moviw_off",
        syntax: Syntax::MoviwwiOffset,
        operands: &[
            OpdDesc { field_idx: 0, kind: SK(6) },
            OpdDesc { field_idx: 1, kind: FSRn },
        ],
        opcode: 0b11_1111_0000_0000,
    },
    InsnDesc {
        mnemonic: "movwi_mm",
        syntax: Syntax::MoviwwiMM,
        operands: &[
            OpdDesc { field_idx: 1, kind: FSRn }, // !!
            OpdDesc { field_idx: 0, kind: MM }, // !!
        ],
        opcode: 0b00_0000_0001_1000,
    },
    InsnDesc {
        mnemonic: "movwi_off",
        syntax: Syntax::MoviwwiOffset,
        operands: &[
            OpdDesc { field_idx: 0, kind: SK(6) },
            OpdDesc { field_idx: 1, kind: FSRn },
        ],
        opcode: 0b11_1111_1000_0000,
    },
];
