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
    "swa"  => "{R4}   0 001",
    "add"  => "{R4}   0 010",
    "addi" => "{IMM4} 0 011",
    "nand" => "{R4}   0 100",
    "ld"   => "{R4}   0 101",

    "swa.f"  => "{R4}   1 001",
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
        swa zero
        addi (((imm >> 4)+(( imm & 8 ) << 1)) & 0b00001111)
        add acc
        add acc
        add acc
        add acc
        addi (imm & 0b00001111)
    ",
    "lda src" => "
        swa zero
        add src
    ",
    "sta dest" => "
        swa dest
        swa zero
        add dest
    ",
    "mov dest, src" => "
        swa dest
        swa zero
        add src
        swa dest
    ",
    "not src" => "
        swa zero
        add src
        nand acc
    ",
    "and src" => "
        nand src
        nand acc
    ",
    "andi imm" => "    
        swa tr1
        lim imm
        and tr1
    ",
    "or src" => "   
        nand acc
        swa tr1
        not src
        nand tr1
    ",
    "ori imm" => "
        nand acc
        swa tr1
        lim imm
        nand tr1
    ",
    "xor src" => "    
        swa tr1
        swa zero
        add src
        nand tr1
        swa tr1
        nand tr1
        swa tr1
        nand src
        nand tr1
    ",
    "xori imm" => "
        swa tr2
        lim imm
        xor tr2
    ",
    "sub src" => "
        swa tr1
        nand src
        addi 1
        add tr1
    ",
    "suba src" => "
        nand acc
        addi 1
        add src
    ",
    "brc cond, addr" => "
        lim ((addr >> 8) & 0xFF)
        swa seg
        lim (addr & 0xFF)
        b cond
    ",
    "jmp addr" => "
        brc true, addr
    ",

};
