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

    addr = addr2[opd] (wso ("+" / "-")[op] wso addr2[opd])*
    addr2 = addr3[opd] (wso "*" wso addr3[opd])*
    addr3 = ("-" / "~")[pre] wso addr4[opd]
    addr4 =
        (bin_uint / dec_uint / oct_uint / hex_uint)[uint]
        / ident[ident]
        / "(" wso addr[inner] wso ")"

    bank = 

    tr_unit = ws (insn wso comment? "\n" ws)* (insn wso comment?)?
"##;
