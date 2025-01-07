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
};

pub const CONDITIONS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "false" => 0,
    "true" => 1,
    "na" => 2,
    "a" => 3,
    "nb" => 4,
    "b" => 5,
    "noverflow" => 6,
    "overflow" => 7,
    "nsign" => 8,
    "sign" => 9,
    "even" => 10,
    "odd" => 11,
    "zero" => 12,
    "nzero" => 13,
    "ncarry" => 14,
    "carry" => 15,
};


pub const INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "ext"  => "0000   1000",
    "sta"  => "{R4}   0000",
    "lda"  => "{R4}   0 001",
    "add"  => "{R4}   0 010",
    "addi" => "{IMM4} 0 011",
    "nand" => "{R4}   0 100",
    "ld"   => "{R4}   0 101",

    "lda.f"  => "{R4}   1 001",
    "add.f"  => "{R4}   1 010",
    "addi.f" => "{IMM4} 1 011",
    "nand.f" => "{R4}   1 100",
    "ld.f"   => "{R4}   1 101",
    
    "st"   => "{R4}   0 110",
    "b"    => "{C4}   0 111"
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
    "nop" => "b false",
    "lim imm" => "
        lda zero
        addi (((imm >> 4)+(( imm & 8 ) >> 3)) & 0b00001111)
        add acc
        add acc
        add acc
        add acc
        addi (imm & 0b00001111)
    ",
    "mov dest, src" => "
        sta tr1
        lda src
        sta dest
        lda tr1
    ",
    "mov.f dest, src" => "
        sta tr1
        lda.f src
        sta dest
        lda tr1
    ",
    "swa src" => "
        sta tr1
        lda src
        sta tr2
        lda tr1
        sta src
        lda tr2
    ",
    "not src" => "
        lda src
        nand acc
    ",
    "not.f src" => "
        lda src
        nand.f acc
    ",
    "and src" => "
        nand src
        nand acc
    ",
    "and.f src" => "
        nand src
        nand.f acc
    ",
    "or src" => "
        nand acc
        sta tr1
        not src
        nand tr1
    ",
    "or.f src" => "
        nand acc
        sta tr1
        not src
        nand.f tr1
    ",
    // TODO: This throws an error "You can only have one name per pseudoinstruction".
    // TODO: Maybe add support for 2-arg pseudo-instructions?
    // "andi src imm" => "
    //     lim imm
    //     and src
    // ",
    "xor src" => "
        sta tr1
        nand src
        sta tr2
        nand tr1
        sta tr1
        lda tr2
        nand src
        nand tr1
    ",
    "xor.f src" => "
        sta tr1
        nand src
        sta tr2
        nand tr1
        sta tr1
        lda tr2
        nand src
        nand.f tr1
    ",
    "sub src" => "
        sta tr1
        nand src
        addi 1
        add tr1
    ",
    "sub.f src" => "
        sta tr1
        nand src
        addi 1
        add.f tr1
    ",
    "suba src" => "
        nand acc
        addi 1
        add src
    ",
    "brc cond, addr" => "
        lim ((addr >> 8) & 0xFF)
        sta seg
        lim (addr & 0xFF)
        b cond
    ",
    "jmp addr" => "
        brc true, addr
    ",
};
