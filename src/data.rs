#[derive(Clone)]
pub(crate) struct Insn {
    desc: &'static InsnDesc,
    operands: [Opd; 2],
}

impl Insn {
    fn encode(&self) -> u16 {
        // TODO: Do we want to precompute or at least cache this?
        let mut fields_desc = self.desc.operands.to_vec();
        if fields_desc[0].field_idx == 1 {
            fields_desc.swap(0, 1);
        }

        let mut word = self.desc.opcode;
        for (field_desc, opd) in fields_desc.iter().zip(&self.operands) {
            let width = field_desc.kind.width();
            assert_eq!(((1 << width) - 1) & word, 0);
            word |= opd.raw;
            word <<= width;
        }
        word
    }
}

#[cfg(test)]
#[test]
fn round_trip() {
}

#[derive(Clone, Default)]
pub(crate) struct Opd {
    // TODO: Figure this out.
    raw: u16,
}

#[derive(Clone)]
pub(crate) struct InsnDesc {
    pub(crate) mnemonic: &'static str,
    pub(crate) syntax: Syntax,
    pub(crate) operands: &'static [OpdDesc], // opcode doesn't count
    pub(crate) opcode: u16,
}

#[derive(Clone, Copy)]
pub(crate) enum Syntax {
    Normal,
    MoviwwiMM,
    MoviwwiOffset,
    Tris,
}

impl Syntax {
    // TODO: This doesn't belong here.
    fn grammar_str(&self) -> &'static str {
        use self::Syntax::*;
        match *self {
            Normal =>
                panic!("you were supposed to use GrammarType::grammar_str()"),
            MoviwwiMM => r#"mod[pre] fsrn[fsrn] / fsrn[fsrn] mod[post]"#,
            MoviwwiOffset => r#"int ws "[" ws fsrn ws "]""#,
            Tris => "tris",
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct OpdDesc {
    field_idx: u8, // lsb to msb
    kind: OpdDescKind,
}

/// Tells the assembler how to turn an operand into bits.
#[derive(Clone, Copy)]
pub(crate) enum OpdDescKind {
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
    FSRn, // FSR register
    MM, // pre/post inc/dec
}

impl OpdDescKind {
    pub(crate) fn width(&self) -> usize {
        match *self {
            DC(n)
            | K(n)
            | UK(n)
            | SK(n)
            | APK(n)
            | RPK(n) => n as usize,
            F => 7,
            D => 1,
            B => 3,
            A => 5,
            PCLATH => 7,
            FSRn => 1,
            MM => 2,
        }
    }

    pub(crate) fn data_type(&self) -> DataType {
        match *self {
            DC(_) => DataType::Invisible,
            F => DataType::DataAddr,
            D => DataType::Dest,
            B => DataType::Bit,
            K(_)
            | UK(_)
            | SK(_) => DataType::Int,
            A => DataType::DataAddr, // watch out!
            PCLATH
            | APK(_)
            | RPK(_) => DataType::ProgAddr,
            FSRn => DataType::FSRn,
            MM => DataType::MM,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum DataType {
    Invisible,
    DataAddr,
    ProgAddr,
    Tris,
    Dest,
    Bit,
    Int,
    FSRn,
    MM,
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

struct InsnDescTable {
    table: Vec<&'static InsnDesc>,
}

impl InsnDescTable {
    fn new() -> Self {
        let mut table = vec![&INVALID_INSN_DESC; 0b100_0000_0000_0000];
        for desc in INSN_DESCS {
            let total_opd_width: usize =
                desc.operands.iter().map(|opd| opd.kind.width()).sum();
            let mask: usize = (1 << total_opd_width) - 1;
            let start: usize = desc.opcode as usize;
            let end: usize = desc.opcode as usize | mask;
            println!("{} {:b} {:b} {:b}", desc.mnemonic, mask, start, end);
            for opcode in start..=end {
                table[opcode] = desc;
            }
        }

        Self { table }
    }

    fn decode(&self, word: u16) -> Insn {
        let insn_desc = self.table[word as usize];

        // TODO: Do we want to precompute or at least cache this?
        let mut fields_desc = insn_desc.operands.to_vec();
        fields_desc.sort_unstable_by_key(|field_desc| field_desc.field_idx);

        let mut word = word;
        let mut operands = [Opd::default(), Opd::default()];
        for (field_desc, opd) in fields_desc.iter().zip(operands.iter_mut()) {
            let width = field_desc.kind.width();
            let field = ((1 << width) - 1) & word;
            word <<= width;
            opd.raw = field;
        }

        Insn { desc: insn_desc, operands }
    }
}

#[cfg(test)]
#[test]
fn test_get_insn_desc_table() {
    let table = InsnDescTable::new();
    assert_eq!(table.table.len(), 0b100_0000_0000_0000); // ?
    assert_eq!(table.decode(0b00_1001_0000_0000).desc.mnemonic, "comf");
    assert_eq!(table.decode(0b00_1001_0111_1111).desc.mnemonic, "comf");
    assert_eq!(table.decode(0b00_1001_1111_1111).desc.mnemonic, "comf");
    assert_eq!(table.decode(0b00_0000_0000_0010).desc.mnemonic, "_invalid_");
}

static INVALID_INSN_DESC: InsnDesc = InsnDesc {
    mnemonic: "_invalid_",
    syntax: Syntax::Normal,
    operands: &[],
    opcode: 0,
};

pub(crate) static INSN_DESCS: &[InsnDesc] = &[
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
        syntax: Syntax::Tris,
        operands: &[],
        opcode: 0b00_0000_0110_0101,
    },
    InsnDesc {
        mnemonic: "tris_b",
        syntax: Syntax::Tris,
        operands: &[],
        opcode: 0b00_0000_0110_0110,
    },
    InsnDesc {
        mnemonic: "tris_c",
        syntax: Syntax::Tris,
        operands: &[],
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
