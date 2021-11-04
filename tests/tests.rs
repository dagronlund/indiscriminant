use no_discrimination::*;

#[test]
fn test_bits() {
    #[no_discrimination_bits(u8, 8)]
    pub enum TestEnum {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
        _Default,
    }

    #[no_discrimination_bits(u8, 2)]
    pub enum TestEnumFull {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
    }

    #[no_discrimination_bits(u16, 8)]
    #[derive(PartialEq)]
    pub enum TestEnum16 {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
        E = 4,
        _Default,
    }

    assert!(TestEnum16::from_int(0) == TestEnum16::A);
    assert!(TestEnum16::from_int(4) == TestEnum16::E);
    assert!(TestEnum16::from_int(5) == TestEnum16::_Default);
    assert!(TestEnum16::from_int(255) == TestEnum16::_Default);
    assert!(TestEnum16::_Default.to_int() == 5);

    #[no_discrimination_bits(u8, 8)]
    #[derive(PartialEq)]
    enum FullEnum {
        B0 = 0,
        B1 = 1,
        B2 = 2,
        B3 = 3,
        B4 = 4,
        B5 = 5,
        B6 = 6,
        B7 = 7,
        B8 = 8,
        B9 = 9,
        B10 = 10,
        B11 = 11,
        B12 = 12,
        B13 = 13,
        B14 = 14,
        B15 = 15,
        B16 = 16,
        B17 = 17,
        B18 = 18,
        B19 = 19,
        B20 = 20,
        B21 = 21,
        B22 = 22,
        B23 = 23,
        B24 = 24,
        B25 = 25,
        B26 = 26,
        B27 = 27,
        B28 = 28,
        B29 = 29,
        B30 = 30,
        B31 = 31,
        B32 = 32,
        B33 = 33,
        B34 = 34,
        B35 = 35,
        B36 = 36,
        B37 = 37,
        B38 = 38,
        B39 = 39,
        B40 = 40,
        B41 = 41,
        B42 = 42,
        B43 = 43,
        B44 = 44,
        B45 = 45,
        B46 = 46,
        B47 = 47,
        B48 = 48,
        B49 = 49,
        B50 = 50,
        B51 = 51,
        B52 = 52,
        B53 = 53,
        B54 = 54,
        B55 = 55,
        B56 = 56,
        B57 = 57,
        B58 = 58,
        B59 = 59,
        B60 = 60,
        B61 = 61,
        B62 = 62,
        B63 = 63,
        B64 = 64,
        B65 = 65,
        B66 = 66,
        B67 = 67,
        B68 = 68,
        B69 = 69,
        B70 = 70,
        B71 = 71,
        B72 = 72,
        B73 = 73,
        B74 = 74,
        B75 = 75,
        B76 = 76,
        B77 = 77,
        B78 = 78,
        B79 = 79,
        B80 = 80,
        B81 = 81,
        B82 = 82,
        B83 = 83,
        B84 = 84,
        B85 = 85,
        B86 = 86,
        B87 = 87,
        B88 = 88,
        B89 = 89,
        B90 = 90,
        B91 = 91,
        B92 = 92,
        B93 = 93,
        B94 = 94,
        B95 = 95,
        B96 = 96,
        B97 = 97,
        B98 = 98,
        B99 = 99,
        B100 = 100,
        B101 = 101,
        B102 = 102,
        B103 = 103,
        B104 = 104,
        B105 = 105,
        B106 = 106,
        B107 = 107,
        B108 = 108,
        B109 = 109,
        B110 = 110,
        B111 = 111,
        B112 = 112,
        B113 = 113,
        B114 = 114,
        B115 = 115,
        B116 = 116,
        B117 = 117,
        B118 = 118,
        B119 = 119,
        B120 = 120,
        B121 = 121,
        B122 = 122,
        B123 = 123,
        B124 = 124,
        B125 = 125,
        B126 = 126,
        B127 = 127,
        B128 = 128,
        B129 = 129,
        B130 = 130,
        B131 = 131,
        B132 = 132,
        B133 = 133,
        B134 = 134,
        B135 = 135,
        B136 = 136,
        B137 = 137,
        B138 = 138,
        B139 = 139,
        B140 = 140,
        B141 = 141,
        B142 = 142,
        B143 = 143,
        B144 = 144,
        B145 = 145,
        B146 = 146,
        B147 = 147,
        B148 = 148,
        B149 = 149,
        B150 = 150,
        B151 = 151,
        B152 = 152,
        B153 = 153,
        B154 = 154,
        B155 = 155,
        B156 = 156,
        B157 = 157,
        B158 = 158,
        B159 = 159,
        B160 = 160,
        B161 = 161,
        B162 = 162,
        B163 = 163,
        B164 = 164,
        B165 = 165,
        B166 = 166,
        B167 = 167,
        B168 = 168,
        B169 = 169,
        B170 = 170,
        B171 = 171,
        B172 = 172,
        B173 = 173,
        B174 = 174,
        B175 = 175,
        B176 = 176,
        B177 = 177,
        B178 = 178,
        B179 = 179,
        B180 = 180,
        B181 = 181,
        B182 = 182,
        B183 = 183,
        B184 = 184,
        B185 = 185,
        B186 = 186,
        B187 = 187,
        B188 = 188,
        B189 = 189,
        B190 = 190,
        B191 = 191,
        B192 = 192,
        B193 = 193,
        B194 = 194,
        B195 = 195,
        B196 = 196,
        B197 = 197,
        B198 = 198,
        B199 = 199,
        B200 = 200,
        B201 = 201,
        B202 = 202,
        B203 = 203,
        B204 = 204,
        B205 = 205,
        B206 = 206,
        B207 = 207,
        B208 = 208,
        B209 = 209,
        B210 = 210,
        B211 = 211,
        B212 = 212,
        B213 = 213,
        B214 = 214,
        B215 = 215,
        B216 = 216,
        B217 = 217,
        B218 = 218,
        B219 = 219,
        B220 = 220,
        B221 = 221,
        B222 = 222,
        B223 = 223,
        B224 = 224,
        B225 = 225,
        B226 = 226,
        B227 = 227,
        B228 = 228,
        B229 = 229,
        B230 = 230,
        B231 = 231,
        B232 = 232,
        B233 = 233,
        B234 = 234,
        B235 = 235,
        B236 = 236,
        B237 = 237,
        B238 = 238,
        B239 = 239,
        B240 = 240,
        B241 = 241,
        B242 = 242,
        B243 = 243,
        B244 = 244,
        B245 = 245,
        B246 = 246,
        B247 = 247,
        B248 = 248,
        B249 = 249,
        B250 = 250,
        B251 = 251,
        B252 = 252,
        B253 = 253,
        // B254 = 254,
        B255 = 255,
        _Default,
    }

    assert!(FullEnum::B0.to_int() == 0);
    assert!(FullEnum::B253.to_int() == 253);
    assert!(FullEnum::_Default.to_int() == 254);
    assert!(FullEnum::B255.to_int() == 255);

    assert!(FullEnum::from_int(0) == FullEnum::B0);
    assert!(FullEnum::from_int(253) == FullEnum::B253);
    assert!(FullEnum::from_int(254) == FullEnum::_Default);
    assert!(FullEnum::from_int(255) == FullEnum::B255);

    #[no_discrimination_bits(u8, 2)]
    #[derive(PartialEq)]
    pub enum TestEnumMixed {
        A = 0,
        B = 1,
        D = 3,
        _Default,
    }

    assert!(TestEnumMixed::A.to_int() == 0);
    assert!(TestEnumMixed::B.to_int() == 1);
    assert!(TestEnumMixed::_Default.to_int() == 2);
    assert!(TestEnumMixed::D.to_int() == 3);

    assert!(TestEnumMixed::from_int(0) == TestEnumMixed::A);
    assert!(TestEnumMixed::from_int(1) == TestEnumMixed::B);
    assert!(TestEnumMixed::from_int(2) == TestEnumMixed::_Default);
    assert!(TestEnumMixed::from_int(3) == TestEnumMixed::D);
}

#[test]
fn test_byte_str() {
    #[no_discrimination_byte_str_default()]
    #[derive(PartialEq)]
    pub enum TestEnumDefault {
        A = b"A",
        B = b"B",
        Default,
    }

    assert!(TestEnumDefault::A.to_byte_str() == b"A");
    assert!(TestEnumDefault::B.to_byte_str() == b"B");
    assert!(TestEnumDefault::Default.to_byte_str() == b"");

    assert!(TestEnumDefault::from_byte_str(b"A") == TestEnumDefault::A);
    assert!(TestEnumDefault::from_byte_str(b"B") == TestEnumDefault::B);
    assert!(TestEnumDefault::from_byte_str(b"") == TestEnumDefault::Default);
    assert!(TestEnumDefault::from_byte_str(b"ASDF") == TestEnumDefault::Default);

    #[no_discrimination_byte_str()]
    #[derive(PartialEq)]
    pub enum TestEnum {
        A = b"A",
        B = b"B",
        C = b"",
    }

    assert!(TestEnum::A.to_byte_str() == b"A");
    assert!(TestEnum::B.to_byte_str() == b"B");
    assert!(TestEnum::C.to_byte_str() == b"");

    assert!(TestEnum::from_byte_str(b"A") == Some(TestEnum::A));
    assert!(TestEnum::from_byte_str(b"B") == Some(TestEnum::B));
    assert!(TestEnum::from_byte_str(b"") == Some(TestEnum::C));
    assert!(TestEnum::from_byte_str(b"ASDF") == None);
}

#[test]
fn test_str() {
    #[no_discrimination_str_default()]
    #[derive(PartialEq)]
    pub enum TestEnumDefault {
        A = "A",
        B = "B",
        Default,
    }

    assert!(TestEnumDefault::A.to_str() == "A");
    assert!(TestEnumDefault::B.to_str() == "B");
    assert!(TestEnumDefault::Default.to_str() == "");

    assert!(TestEnumDefault::from_str("A") == TestEnumDefault::A);
    assert!(TestEnumDefault::from_str("B") == TestEnumDefault::B);
    assert!(TestEnumDefault::from_str("") == TestEnumDefault::Default);
    assert!(TestEnumDefault::from_str("ASDF") == TestEnumDefault::Default);

    #[no_discrimination_str()]
    #[derive(PartialEq)]
    pub enum TestEnum {
        A = "A",
        B = "B",
        C = "",
    }

    assert!(TestEnum::A.to_str() == "A");
    assert!(TestEnum::B.to_str() == "B");
    assert!(TestEnum::C.to_str() == "");

    assert!(TestEnum::from_str("A") == Some(TestEnum::A));
    assert!(TestEnum::from_str("B") == Some(TestEnum::B));
    assert!(TestEnum::from_str("") == Some(TestEnum::C));
    assert!(TestEnum::from_str("ASDF") == None);
}
