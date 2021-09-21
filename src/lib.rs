pub use bitsmart_enum_impl::bitsmart_enum;

/*
Normal rust enums can be created with specified with custom discriminants like
such:

enum MyEnum {
    A = 1,
    B = 2
}

The backing integer type can even be specified specifically:

#[repr(u8)]
enum MyByteEnum {
    A = 1,
    B = 2
}

However conversion between integers and the enum can be clunky and could
potentially result in an error if the enum cases are not completely covered.
Even for u8 enums, covering all 256 possibilities is untenable.

To fix this you can apply the bitsmart_enum attribute to enums:

#[bitsmart_enum(u8, 1)]
enum MyBitsmartEnum {
    A = 0,
    B = 1
}

where the first argument is the backing integer type and the second argument is
how many least-significant bits of the backing integer to interpret as the enum.

You can also specify if an integer is supposed to be zero'd except for the enum
fields when converting it to the enum:

#[bitsmart_enum_safe(u8, 1)]
enum MyBitsmartEnum {
    A = 0,
    B = 1
}

You do not have to specify every possible value for the enum you are specifying,
but if not you need to add an un-valued field named Default

#[bitsmart_enum_safe(u8, 2)]
enum MyBitsmartEnum {
    A = 0,
    B = 1,
    Default
}

The default variant must be last if it is going to exist at all. The
bitsmart_enum attribute also provides two functions for each enum it is applied
to

let a: u8 = MyBitsmartEnum::A.to_int();
let b: MyBitsmartEnum = MyBitsmartEnum::from_int(a);

The functions are guaranteed to succeed, hence the requirements on Default
fields when necessary as the enum is defined.
*/

#[test]
fn bitsmart_test() {
    #[bitsmart_enum(u8, 8)]
    pub enum TestStruct {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
        Default,
    }

    #[bitsmart_enum(u8, 2)]
    pub enum TestStructFull {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
    }

    #[bitsmart_enum(u16, 8)]
    pub enum TestStruct16 {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
        Default,
    }

    #[bitsmart_enum(u8, 8)]
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
        B254 = 254,
        Default,
        // B255 = 255,
    }

    let _temp = TestStruct::A;
    let _temp = _temp.to_int();
    let _temp = TestStruct::from_int(_temp);

    let _temp = TestStruct16::A;

    println!("Testing!!!");
}
