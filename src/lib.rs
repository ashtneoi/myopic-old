extern crate destroy;

use data::{Insn, INSN_DESCS, Opd};
use destroy::parse::{
    parse_grammar,
    ParseError,
    Parser,
};
use destroy::string_table::{
    StringTable,
    StringTableEntry,
};

mod data;

static GRAMMAR: &str = r##"
    dec_nzdigit = '1'..'9'
    dec_digit = "0" / dec_nzdigit
    latin_letter = 'a'..'z' / 'A'..'Z'

    comment = "#" (-"\n" %)*

    wso_part = " " / "\t"
    ws_part = wso_part / comment? "\n"
    wso = wso_part*
    ws = ws_part*
    pwso = wso_part+
    pws = ws_part+

    bin_digit = '0'..'1'
    bin_uint = "0n" (bin_digit / "_")+

    dec_uint = "0" / dec_nzdigit (dec_digit / "_")*

    oct_digit = '0'..'7'
    oct_uint = "0c" (oct_digit / "_")+

    hex_digit = dec_digit / 'a'..'f' / 'A'..'F'
    hex_uint = "0x" (hex_digit / "_")+

    str =
        "\""
        ("\\" ("n" / "t" / "\\" / "\"") / -"\"" -"\n" %)[cp]*
        "\""
    ident_initial = latin_letter / "_" / 0x80..0x10FFFF # TODO
    ident = ident_initial (ident_initial / dec_digit)* # TODO

    # same as C precedence except for bit shift operators
    expr = expr2[opd] (wso "|" wso expr2[opd])* # ltr
    expr2 = expr3[opd] (wso "^" wso expr3[opd])* # ltr
    expr3 = expr4[opd] (wso "&" wso expr4[opd])* # ltr
    expr4 = expr5[opd] (wso ("+" / "-")[op] wso expr5[opd])* # ltr
    expr5 = expr6[opd] (wso ("<<" / ">>")[op] wso expr6[opd])* # (!) ltr
    expr6 = expr7[opd] (wso "*" wso expr7[opd])* # ltr
    expr7 = ("-" / "~")[pre]? wso expr8[opd] # rtl
    expr8 =
        (bin_uint / oct_uint / hex_uint / dec_uint)[uint]
        / ident[ident]
        / "(" wso expr[inner] wso ")"

    mod = "++" / "--"
    fsrn = "FSR0" / "FSR1"

    insn =
        # inherent
        (
            "clrw" / "brw" / "callw" / "retfie" / "return" / "clrwdt" / "nop"
            / "reset" / "sleep"
        )[m]

        # f
        / ("clrf" / "movwf")[m] wso expr[a]

        # f, d
        / (
            "addwf" / "addwfc" / "andwf" / "asrf" / "lslf" / "lsrf" / "comf"
            / "decf" / "incf" / "iorwf" / "movf" / "rlf" / "rrf" / "subwf"
            / "subwfb" / "swapf" / "xorwf"
        )[m] wso expr[f] (wso "," wso ("W"/ "F")[d])?

        # f, b
        / (
            "decfsz" / "incfsz" / "bcf" / "bsf" / "btfsc" / "btfss" / "ifc"
            / "ifs"
        )[m] wso expr[f] wso "," wso expr[b]

        # k
        / (
            "addlw" / "andlw" / "iorlw" / "movlb" / "movlp" / "movlw" / "sublw"
            / "xorlw" / "bra" / "call" / "goto" / "retlw"
        )[m] wso expr[k]

        # tris
        / "tris"[m] wso ("TRISA" / "TRISB" / "TRISC")[t]

        # n mm / k[n]
        / ("moviw" / "movwi")[m] wso
            (mod[pre] fsrn[fsrn] / fsrn[fsrn] mod[post])

    line = (ident[label] wso ":" wso)? (insn wso)? comment?
    tr_unit = ws (line[line] "\n" ws)* line[line]?
"##;

#[derive(Debug)]
pub(crate) struct TrUnit<'s>(Vec<(Vec<&'s str>, Insn)>);

pub fn parse_tr_unit(input: &str) -> Result<String, String> {
    let nop_insn =
        INSN_DESCS.iter().find(|desc| desc.mnemonic == "nop").unwrap();

    let mut tab = StringTable::new();
    for (i, desc) in data::INSN_DESCS.iter().enumerate() {
        let &StringTableEntry(_, k) = tab.insert(desc.mnemonic.to_string());
        assert_eq!(i, k);
    }
    let g = parse_grammar(&mut tab, GRAMMAR)
        .map_err(|e| format!("{}", e))?;
    let tr_unit_st = Parser::parse(&g, "tr_unit", input)
        .map_err(|e| format!("{}", e))?;

    let mut tr_unit = TrUnit(vec![]);

    println!("{:?}", tr_unit_st);

    let mut line_sts = tr_unit_st.iter("line").peekable();
    while line_sts.peek().is_some() {
        let mut labels = vec![];
        let mut insn = None;
        while insn.is_none() {
            if let Some(line_st) = line_sts.next() {
                let label = line_st.get_or_empty("label");
                assert!(label.len() <= 1);
                if let Some(label) = label.first() {
                    labels.push(label.raw(input));
                }
                // build insn from 'm', if present
            } else if !labels.is_empty() {
                // nop_insn thing goes here
            }
            insn = Some(Insn {
                desc: nop_insn,
                operands: [
                    Opd { raw: 0 },
                    Opd { raw: 0 },
                ],
            });  // FIXME
        }
        tr_unit.0.push((labels, insn.unwrap()));
    }

    Ok(format!("{:?}", tr_unit))
}

#[cfg(test)]
#[test]
fn parse_empty_string() {
    parse_tr_unit("").unwrap();
}
