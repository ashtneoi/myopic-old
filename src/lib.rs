extern crate destroy;

mod data;

static GRAMMAR: &str = r##"
    nzdigit = '1'..'9'
    digit = "0" / nzdigit
    latin_letter = 'a'..'z' / 'A'..'Z'

    comment = "#" (-"\n" %)*

    wso_part = " " / "\t"
    ws_part = wso_part / comment? "\n"
    wso = wso_part*
    ws = ws_part*
    pwso = wso_part+
    pws = ws_part+

    bin_digit = '0'..'1'
    bin_uint = "0n" bin_digit+

    dec_uint = "0" / nzdigit digit*

    oct_digit = '0'..'7'
    oct_uint = "0c" oct_digit+

    hex_digit = digit / 'a'..'f' / 'A'..'F'
    hex_uint = "0x" hex_digit+

    str =
        "\""
        ("\\" ("n" / "t" / "\\" / "\"") / -"\"" -"\n" %)[cp]*
        "\""
    ident_initial = latin_letter / "_" / 0x80..0x10FFFF # TODO
    ident = ident_initial (ident_initial / digit)* # TODO

    expr = expr2[opd] (wso ("+" / "-")[op] wso expr2[opd])*
    expr2 = expr3[opd] (wso "*" wso expr3[opd])*
    expr3 = ("-" / "~")[pre] wso expr4[opd]
    expr4 =
        (bin_uint / dec_uint / oct_uint / hex_uint)[uint]
        / ident[ident]
        / "(" wso expr[inner] wso ")"

    bank = ident

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
        )[m] wso expr[f] wso "," wso bit[b]

        # k
        / (
            "addlw" / "andlw" / "iorlw" / "movlb" / "movlp" / "movlw" / "sublw"
            / "xorlw" / "bra" / "call" / "goto" / "retlw"
        )[m] wso expr[k]

        # tris
        / "tris"[m] wso ("TRISA" / "TRISB" / "TRISC")[t]

        # n mm / k[n]
        / ("moviw" / "movwi") wso (mod[pre] fsrn[fsrn] / fsrn



    tr_unit = ws (insn wso comment? "\n" ws)* (insn wso comment?)?
"##;
