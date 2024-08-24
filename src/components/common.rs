use phf::phf_map;

//format {(type)(count in bits)}
// possible types:
// IMM - Immediate
// E - Extra
// + declared

pub const TYPES: phf::Map<&'static str, phf::Map<&'static str, usize>> = phf_map!{
    "R" => REGISTERS_TO_VAL,
    "C" => CONDITIONS_TO_VAL,
};

pub const REGISTERS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "zero" => 0,
    "acc" => 1,
    "flg" => 2,
    "seg" => 3,
    "tr1" => 4,
    "tr2" => 5,
    "r0" => 0,
    "r1" => 1,
    "r2" => 2,
    "r3" => 3,
    "r4" => 4,
    "r5" => 5,
    "r6" => 6,
    "r7" => 7,
    "r8" => 8,
    "r9" => 9,
    "r10" => 10,
    "r11" => 11,
    "r12" => 12,
    "r13" => 13,
    "r14" => 14,
    "r15" => 15,
    "r16" => 16,
    "r17" => 17,
    "r18" => 18,
    "r19" => 19,
    "r20" => 20,
    "r21" => 21,
    "r22" => 22,
    "r23" => 23,
    "r24" => 24,
    "r25" => 25,
    "r26" => 26,
    "r27" => 27,
    "r28" => 28,
    "r29" => 29,
    "r30" => 30,
    "r31" => 31,
};

pub const CONDITIONS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "false" => 0,
    "true" => 1,
    "na" => 2,
    "a" => 3,
    "nb" => 4,
    "b" => 5,
    "even" => 6,
    "odd" => 7,
    "nzero" => 8,
    "zero" => 9,
    "nsign" => 10,
    "sign" => 11,
    "ncarry" => 12,
    "carry" => 13,
    "noverflow" => 14,
    "overflow" => 15
};


pub const INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "ext"  => "00000  000",
    "swa"  => "{R5}   001",
    "add"  => "{R5}   010",
    "addi" => "{IMM5} 011",
    "nand" => "{R5}   100",
    "ld"   => "{R5}   101",
    "st"   => "{R5}   110",
    "b"    => "{C5}   111"
};

// Closure ops
// +  add
// -  subtract
// *  multuply
// /  divide
// &  bitwise and
// |  bitwise or
// ^  bitwise xor
// << bitshift left
// >> bitshift right

pub const PSEUDO_INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "lim a" => "
        swa zero
        addi (a >> 3)
        add acc
        add acc
        add acc
        addi (a & 0b00000111)
    ",

    "lda src" => "
        swa zero
        add src
    "
};
