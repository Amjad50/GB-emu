use super::instruction::Condition::*;
use super::instruction::Opcode;
use super::instruction::Opcode::*;
use super::instruction::OperandType;
use super::instruction::OperandType::*;

pub(super) const INSTRUCTIONS: &[(Opcode, (OperandType, OperandType)); 256] = &[
    (Nop, (Implied, Implied)),
    (Ld, (RegBC, Imm16)),
    (Ld, (AddrBC, RegA)),
    (Inc16, (RegBC, RegBC)),
    (Inc, (RegB, RegB)),
    (Dec, (RegB, RegB)),
    (Ld, (RegB, Imm8)),
    (Rlca, (Implied, Implied)),
    (Ld, (Addr16Val16, RegSP)),
    (Add16, (RegHL, RegBC)),
    (Ld, (RegA, AddrBC)),
    (Dec16, (RegBC, RegBC)),
    (Inc, (RegC, RegC)),
    (Dec, (RegC, RegC)),
    (Ld, (RegC, Imm8)),
    (Rrca, (Implied, Implied)),
    (Stop, (Implied, Implied)),
    (Ld, (RegDE, Imm16)),
    (Ld, (AddrDE, RegA)),
    (Inc16, (RegDE, RegDE)),
    (Inc, (RegD, RegD)),
    (Dec, (RegD, RegD)),
    (Ld, (RegD, Imm8)),
    (Rla, (Implied, Implied)),
    (Jr(Unconditional), (Implied, Imm8Signed)),
    (Add16, (RegHL, RegDE)),
    (Ld, (RegA, AddrDE)),
    (Dec16, (RegDE, RegDE)),
    (Inc, (RegE, RegE)),
    (Dec, (RegE, RegE)),
    (Ld, (RegE, Imm8)),
    (Rra, (Implied, Implied)),
    (Jr(NZ), (Implied, Imm8Signed)),
    (Ld, (RegHL, Imm16)),
    (Ld, (AddrHLInc, RegA)),
    (Inc16, (RegHL, RegHL)),
    (Inc, (RegH, RegH)),
    (Dec, (RegH, RegH)),
    (Ld, (RegH, Imm8)),
    (Daa, (Implied, Implied)),
    (Jr(Z), (Implied, Imm8Signed)),
    (Add16, (RegHL, RegHL)),
    (Ld, (RegA, AddrHLInc)),
    (Dec16, (RegHL, RegHL)),
    (Inc, (RegL, RegL)),
    (Dec, (RegL, RegL)),
    (Ld, (RegL, Imm8)),
    (Cpl, (Implied, Implied)),
    (Jr(NC), (Implied, Imm8Signed)),
    (Ld, (RegSP, Imm16)),
    (Ld, (AddrHLDec, RegA)),
    (Inc16, (RegSP, RegSP)),
    (Inc, (AddrHL, AddrHL)),
    (Dec, (AddrHL, AddrHL)),
    (Ld, (AddrHL, Imm8)),
    (Scf, (Implied, Implied)),
    (Jr(C), (Implied, Imm8Signed)),
    (Add16, (RegHL, RegSP)),
    (Ld, (RegA, AddrHLDec)),
    (Dec16, (RegSP, RegSP)),
    (Inc, (RegA, RegA)),
    (Dec, (RegA, RegA)),
    (Ld, (RegA, Imm8)),
    (Ccf, (Implied, Implied)),
    (LdBB, (Implied, Implied)),
    (Ld, (RegB, RegC)),
    (Ld, (RegB, RegD)),
    (Ld, (RegB, RegE)),
    (Ld, (RegB, RegH)),
    (Ld, (RegB, RegL)),
    (Ld, (RegB, AddrHL)),
    (Ld, (RegB, RegA)),
    (Ld, (RegC, RegB)),
    (Ld, (RegC, RegC)),
    (Ld, (RegC, RegD)),
    (Ld, (RegC, RegE)),
    (Ld, (RegC, RegH)),
    (Ld, (RegC, RegL)),
    (Ld, (RegC, AddrHL)),
    (Ld, (RegC, RegA)),
    (Ld, (RegD, RegB)),
    (Ld, (RegD, RegC)),
    (Ld, (RegD, RegD)),
    (Ld, (RegD, RegE)),
    (Ld, (RegD, RegH)),
    (Ld, (RegD, RegL)),
    (Ld, (RegD, AddrHL)),
    (Ld, (RegD, RegA)),
    (Ld, (RegE, RegB)),
    (Ld, (RegE, RegC)),
    (Ld, (RegE, RegD)),
    (Ld, (RegE, RegE)),
    (Ld, (RegE, RegH)),
    (Ld, (RegE, RegL)),
    (Ld, (RegE, AddrHL)),
    (Ld, (RegE, RegA)),
    (Ld, (RegH, RegB)),
    (Ld, (RegH, RegC)),
    (Ld, (RegH, RegD)),
    (Ld, (RegH, RegE)),
    (Ld, (RegH, RegH)),
    (Ld, (RegH, RegL)),
    (Ld, (RegH, AddrHL)),
    (Ld, (RegH, RegA)),
    (Ld, (RegL, RegB)),
    (Ld, (RegL, RegC)),
    (Ld, (RegL, RegD)),
    (Ld, (RegL, RegE)),
    (Ld, (RegL, RegH)),
    (Ld, (RegL, RegL)),
    (Ld, (RegL, AddrHL)),
    (Ld, (RegL, RegA)),
    (Ld, (AddrHL, RegB)),
    (Ld, (AddrHL, RegC)),
    (Ld, (AddrHL, RegD)),
    (Ld, (AddrHL, RegE)),
    (Ld, (AddrHL, RegH)),
    (Ld, (AddrHL, RegL)),
    (Halt, (Implied, Implied)),
    (Ld, (AddrHL, RegA)),
    (Ld, (RegA, RegB)),
    (Ld, (RegA, RegC)),
    (Ld, (RegA, RegD)),
    (Ld, (RegA, RegE)),
    (Ld, (RegA, RegH)),
    (Ld, (RegA, RegL)),
    (Ld, (RegA, AddrHL)),
    (Ld, (RegA, RegA)),
    (Add, (RegA, RegB)),
    (Add, (RegA, RegC)),
    (Add, (RegA, RegD)),
    (Add, (RegA, RegE)),
    (Add, (RegA, RegH)),
    (Add, (RegA, RegL)),
    (Add, (RegA, AddrHL)),
    (Add, (RegA, RegA)),
    (Adc, (RegA, RegB)),
    (Adc, (RegA, RegC)),
    (Adc, (RegA, RegD)),
    (Adc, (RegA, RegE)),
    (Adc, (RegA, RegH)),
    (Adc, (RegA, RegL)),
    (Adc, (RegA, AddrHL)),
    (Adc, (RegA, RegA)),
    (Sub, (RegA, RegB)),
    (Sub, (RegA, RegC)),
    (Sub, (RegA, RegD)),
    (Sub, (RegA, RegE)),
    (Sub, (RegA, RegH)),
    (Sub, (RegA, RegL)),
    (Sub, (RegA, AddrHL)),
    (Sub, (RegA, RegA)),
    (Sbc, (RegA, RegB)),
    (Sbc, (RegA, RegC)),
    (Sbc, (RegA, RegD)),
    (Sbc, (RegA, RegE)),
    (Sbc, (RegA, RegH)),
    (Sbc, (RegA, RegL)),
    (Sbc, (RegA, AddrHL)),
    (Sbc, (RegA, RegA)),
    (And, (RegA, RegB)),
    (And, (RegA, RegC)),
    (And, (RegA, RegD)),
    (And, (RegA, RegE)),
    (And, (RegA, RegH)),
    (And, (RegA, RegL)),
    (And, (RegA, AddrHL)),
    (And, (RegA, RegA)),
    (Xor, (RegA, RegB)),
    (Xor, (RegA, RegC)),
    (Xor, (RegA, RegD)),
    (Xor, (RegA, RegE)),
    (Xor, (RegA, RegH)),
    (Xor, (RegA, RegL)),
    (Xor, (RegA, AddrHL)),
    (Xor, (RegA, RegA)),
    (Or, (RegA, RegB)),
    (Or, (RegA, RegC)),
    (Or, (RegA, RegD)),
    (Or, (RegA, RegE)),
    (Or, (RegA, RegH)),
    (Or, (RegA, RegL)),
    (Or, (RegA, AddrHL)),
    (Or, (RegA, RegA)),
    (Cp, (Implied, RegB)),
    (Cp, (Implied, RegC)),
    (Cp, (Implied, RegD)),
    (Cp, (Implied, RegE)),
    (Cp, (Implied, RegH)),
    (Cp, (Implied, RegL)),
    (Cp, (Implied, AddrHL)),
    (Cp, (Implied, RegA)),
    (Ret(NZ), (Implied, Implied)),
    (Pop, (RegBC, Implied)),
    (Jp(NZ), (Implied, Imm16)),
    (Jp(Unconditional), (Implied, Imm16)),
    (Call(NZ), (Implied, Imm16)),
    (Push, (Implied, RegBC)),
    (Add, (RegA, Imm8)),
    (Rst(0), (Implied, Implied)),
    (Ret(Z), (Implied, Implied)),
    (Ret(Unconditional), (Implied, Implied)),
    (Jp(Z), (Implied, Imm16)),
    (Prefix, (Implied, Implied)),
    (Call(Z), (Implied, Imm16)),
    (Call(Unconditional), (Implied, Imm16)),
    (Adc, (RegA, Imm8)),
    (Rst(8), (Implied, Implied)),
    (Ret(NC), (Implied, Implied)),
    (Pop, (RegDE, Implied)),
    (Jp(NC), (Implied, Imm16)),
    (Illegal, (Implied, Implied)),
    (Call(NC), (Implied, Imm16)),
    (Push, (Implied, RegDE)),
    (Sub, (RegA, Imm8)),
    (Rst(16), (Implied, Implied)),
    (Ret(C), (Implied, Implied)),
    (Reti, (Implied, Implied)),
    (Jp(C), (Implied, Imm16)),
    (Illegal, (Implied, Implied)),
    (Call(C), (Implied, Imm16)),
    (Illegal, (Implied, Implied)),
    (Sbc, (RegA, Imm8)),
    (Rst(24), (Implied, Implied)),
    (Ld, (HighAddr8, RegA)),
    (Pop, (RegHL, Implied)),
    (Ld, (HighAddrC, RegA)),
    (Illegal, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Push, (Implied, RegHL)),
    (And, (RegA, Imm8)),
    (Rst(32), (Implied, Implied)),
    (AddSPSigned8, (RegSP, Imm8Signed)),
    (JpHL, (Implied, RegHL)),
    (Ld, (Addr16, RegA)),
    (Illegal, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Xor, (RegA, Imm8)),
    (Rst(40), (Implied, Implied)),
    (Ld, (RegA, HighAddr8)),
    (Pop, (RegAF, Implied)),
    (Ld, (RegA, HighAddrC)),
    (Di, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Push, (Implied, RegAF)),
    (Or, (RegA, Imm8)),
    (Rst(48), (Implied, Implied)),
    (LdHLSPSigned8, (RegHL, Imm8Signed)),
    (LdSPHL, (Implied, Implied)),
    (Ld, (RegA, Addr16)),
    (Ei, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Illegal, (Implied, Implied)),
    (Cp, (Implied, Imm8)),
    (Rst(56), (Implied, Implied)),
];

pub(super) const PREFIXED_INSTRUCTIONS: [(Opcode, (OperandType, OperandType)); 256] = [
    (Rlc, (RegB, RegB)),
    (Rlc, (RegC, RegC)),
    (Rlc, (RegD, RegD)),
    (Rlc, (RegE, RegE)),
    (Rlc, (RegH, RegH)),
    (Rlc, (RegL, RegL)),
    (Rlc, (AddrHL, AddrHL)),
    (Rlc, (RegA, RegA)),
    (Rrc, (RegB, RegB)),
    (Rrc, (RegC, RegC)),
    (Rrc, (RegD, RegD)),
    (Rrc, (RegE, RegE)),
    (Rrc, (RegH, RegH)),
    (Rrc, (RegL, RegL)),
    (Rrc, (AddrHL, AddrHL)),
    (Rrc, (RegA, RegA)),
    (Rl, (RegB, RegB)),
    (Rl, (RegC, RegC)),
    (Rl, (RegD, RegD)),
    (Rl, (RegE, RegE)),
    (Rl, (RegH, RegH)),
    (Rl, (RegL, RegL)),
    (Rl, (AddrHL, AddrHL)),
    (Rl, (RegA, RegA)),
    (Rr, (RegB, RegB)),
    (Rr, (RegC, RegC)),
    (Rr, (RegD, RegD)),
    (Rr, (RegE, RegE)),
    (Rr, (RegH, RegH)),
    (Rr, (RegL, RegL)),
    (Rr, (AddrHL, AddrHL)),
    (Rr, (RegA, RegA)),
    (Sla, (RegB, RegB)),
    (Sla, (RegC, RegC)),
    (Sla, (RegD, RegD)),
    (Sla, (RegE, RegE)),
    (Sla, (RegH, RegH)),
    (Sla, (RegL, RegL)),
    (Sla, (AddrHL, AddrHL)),
    (Sla, (RegA, RegA)),
    (Sra, (RegB, RegB)),
    (Sra, (RegC, RegC)),
    (Sra, (RegD, RegD)),
    (Sra, (RegE, RegE)),
    (Sra, (RegH, RegH)),
    (Sra, (RegL, RegL)),
    (Sra, (AddrHL, AddrHL)),
    (Sra, (RegA, RegA)),
    (Swap, (RegB, RegB)),
    (Swap, (RegC, RegC)),
    (Swap, (RegD, RegD)),
    (Swap, (RegE, RegE)),
    (Swap, (RegH, RegH)),
    (Swap, (RegL, RegL)),
    (Swap, (AddrHL, AddrHL)),
    (Swap, (RegA, RegA)),
    (Srl, (RegB, RegB)),
    (Srl, (RegC, RegC)),
    (Srl, (RegD, RegD)),
    (Srl, (RegE, RegE)),
    (Srl, (RegH, RegH)),
    (Srl, (RegL, RegL)),
    (Srl, (AddrHL, AddrHL)),
    (Srl, (RegA, RegA)),
    (Bit(0), (Implied, RegB)),
    (Bit(0), (Implied, RegC)),
    (Bit(0), (Implied, RegD)),
    (Bit(0), (Implied, RegE)),
    (Bit(0), (Implied, RegH)),
    (Bit(0), (Implied, RegL)),
    (Bit(0), (Implied, AddrHL)),
    (Bit(0), (Implied, RegA)),
    (Bit(1), (Implied, RegB)),
    (Bit(1), (Implied, RegC)),
    (Bit(1), (Implied, RegD)),
    (Bit(1), (Implied, RegE)),
    (Bit(1), (Implied, RegH)),
    (Bit(1), (Implied, RegL)),
    (Bit(1), (Implied, AddrHL)),
    (Bit(1), (Implied, RegA)),
    (Bit(2), (Implied, RegB)),
    (Bit(2), (Implied, RegC)),
    (Bit(2), (Implied, RegD)),
    (Bit(2), (Implied, RegE)),
    (Bit(2), (Implied, RegH)),
    (Bit(2), (Implied, RegL)),
    (Bit(2), (Implied, AddrHL)),
    (Bit(2), (Implied, RegA)),
    (Bit(3), (Implied, RegB)),
    (Bit(3), (Implied, RegC)),
    (Bit(3), (Implied, RegD)),
    (Bit(3), (Implied, RegE)),
    (Bit(3), (Implied, RegH)),
    (Bit(3), (Implied, RegL)),
    (Bit(3), (Implied, AddrHL)),
    (Bit(3), (Implied, RegA)),
    (Bit(4), (Implied, RegB)),
    (Bit(4), (Implied, RegC)),
    (Bit(4), (Implied, RegD)),
    (Bit(4), (Implied, RegE)),
    (Bit(4), (Implied, RegH)),
    (Bit(4), (Implied, RegL)),
    (Bit(4), (Implied, AddrHL)),
    (Bit(4), (Implied, RegA)),
    (Bit(5), (Implied, RegB)),
    (Bit(5), (Implied, RegC)),
    (Bit(5), (Implied, RegD)),
    (Bit(5), (Implied, RegE)),
    (Bit(5), (Implied, RegH)),
    (Bit(5), (Implied, RegL)),
    (Bit(5), (Implied, AddrHL)),
    (Bit(5), (Implied, RegA)),
    (Bit(6), (Implied, RegB)),
    (Bit(6), (Implied, RegC)),
    (Bit(6), (Implied, RegD)),
    (Bit(6), (Implied, RegE)),
    (Bit(6), (Implied, RegH)),
    (Bit(6), (Implied, RegL)),
    (Bit(6), (Implied, AddrHL)),
    (Bit(6), (Implied, RegA)),
    (Bit(7), (Implied, RegB)),
    (Bit(7), (Implied, RegC)),
    (Bit(7), (Implied, RegD)),
    (Bit(7), (Implied, RegE)),
    (Bit(7), (Implied, RegH)),
    (Bit(7), (Implied, RegL)),
    (Bit(7), (Implied, AddrHL)),
    (Bit(7), (Implied, RegA)),
    (Res(0), (RegB, RegB)),
    (Res(0), (RegC, RegC)),
    (Res(0), (RegD, RegD)),
    (Res(0), (RegE, RegE)),
    (Res(0), (RegH, RegH)),
    (Res(0), (RegL, RegL)),
    (Res(0), (AddrHL, AddrHL)),
    (Res(0), (RegA, RegA)),
    (Res(1), (RegB, RegB)),
    (Res(1), (RegC, RegC)),
    (Res(1), (RegD, RegD)),
    (Res(1), (RegE, RegE)),
    (Res(1), (RegH, RegH)),
    (Res(1), (RegL, RegL)),
    (Res(1), (AddrHL, AddrHL)),
    (Res(1), (RegA, RegA)),
    (Res(2), (RegB, RegB)),
    (Res(2), (RegC, RegC)),
    (Res(2), (RegD, RegD)),
    (Res(2), (RegE, RegE)),
    (Res(2), (RegH, RegH)),
    (Res(2), (RegL, RegL)),
    (Res(2), (AddrHL, AddrHL)),
    (Res(2), (RegA, RegA)),
    (Res(3), (RegB, RegB)),
    (Res(3), (RegC, RegC)),
    (Res(3), (RegD, RegD)),
    (Res(3), (RegE, RegE)),
    (Res(3), (RegH, RegH)),
    (Res(3), (RegL, RegL)),
    (Res(3), (AddrHL, AddrHL)),
    (Res(3), (RegA, RegA)),
    (Res(4), (RegB, RegB)),
    (Res(4), (RegC, RegC)),
    (Res(4), (RegD, RegD)),
    (Res(4), (RegE, RegE)),
    (Res(4), (RegH, RegH)),
    (Res(4), (RegL, RegL)),
    (Res(4), (AddrHL, AddrHL)),
    (Res(4), (RegA, RegA)),
    (Res(5), (RegB, RegB)),
    (Res(5), (RegC, RegC)),
    (Res(5), (RegD, RegD)),
    (Res(5), (RegE, RegE)),
    (Res(5), (RegH, RegH)),
    (Res(5), (RegL, RegL)),
    (Res(5), (AddrHL, AddrHL)),
    (Res(5), (RegA, RegA)),
    (Res(6), (RegB, RegB)),
    (Res(6), (RegC, RegC)),
    (Res(6), (RegD, RegD)),
    (Res(6), (RegE, RegE)),
    (Res(6), (RegH, RegH)),
    (Res(6), (RegL, RegL)),
    (Res(6), (AddrHL, AddrHL)),
    (Res(6), (RegA, RegA)),
    (Res(7), (RegB, RegB)),
    (Res(7), (RegC, RegC)),
    (Res(7), (RegD, RegD)),
    (Res(7), (RegE, RegE)),
    (Res(7), (RegH, RegH)),
    (Res(7), (RegL, RegL)),
    (Res(7), (AddrHL, AddrHL)),
    (Res(7), (RegA, RegA)),
    (Set(0), (RegB, RegB)),
    (Set(0), (RegC, RegC)),
    (Set(0), (RegD, RegD)),
    (Set(0), (RegE, RegE)),
    (Set(0), (RegH, RegH)),
    (Set(0), (RegL, RegL)),
    (Set(0), (AddrHL, AddrHL)),
    (Set(0), (RegA, RegA)),
    (Set(1), (RegB, RegB)),
    (Set(1), (RegC, RegC)),
    (Set(1), (RegD, RegD)),
    (Set(1), (RegE, RegE)),
    (Set(1), (RegH, RegH)),
    (Set(1), (RegL, RegL)),
    (Set(1), (AddrHL, AddrHL)),
    (Set(1), (RegA, RegA)),
    (Set(2), (RegB, RegB)),
    (Set(2), (RegC, RegC)),
    (Set(2), (RegD, RegD)),
    (Set(2), (RegE, RegE)),
    (Set(2), (RegH, RegH)),
    (Set(2), (RegL, RegL)),
    (Set(2), (AddrHL, AddrHL)),
    (Set(2), (RegA, RegA)),
    (Set(3), (RegB, RegB)),
    (Set(3), (RegC, RegC)),
    (Set(3), (RegD, RegD)),
    (Set(3), (RegE, RegE)),
    (Set(3), (RegH, RegH)),
    (Set(3), (RegL, RegL)),
    (Set(3), (AddrHL, AddrHL)),
    (Set(3), (RegA, RegA)),
    (Set(4), (RegB, RegB)),
    (Set(4), (RegC, RegC)),
    (Set(4), (RegD, RegD)),
    (Set(4), (RegE, RegE)),
    (Set(4), (RegH, RegH)),
    (Set(4), (RegL, RegL)),
    (Set(4), (AddrHL, AddrHL)),
    (Set(4), (RegA, RegA)),
    (Set(5), (RegB, RegB)),
    (Set(5), (RegC, RegC)),
    (Set(5), (RegD, RegD)),
    (Set(5), (RegE, RegE)),
    (Set(5), (RegH, RegH)),
    (Set(5), (RegL, RegL)),
    (Set(5), (AddrHL, AddrHL)),
    (Set(5), (RegA, RegA)),
    (Set(6), (RegB, RegB)),
    (Set(6), (RegC, RegC)),
    (Set(6), (RegD, RegD)),
    (Set(6), (RegE, RegE)),
    (Set(6), (RegH, RegH)),
    (Set(6), (RegL, RegL)),
    (Set(6), (AddrHL, AddrHL)),
    (Set(6), (RegA, RegA)),
    (Set(7), (RegB, RegB)),
    (Set(7), (RegC, RegC)),
    (Set(7), (RegD, RegD)),
    (Set(7), (RegE, RegE)),
    (Set(7), (RegH, RegH)),
    (Set(7), (RegL, RegL)),
    (Set(7), (AddrHL, AddrHL)),
    (Set(7), (RegA, RegA)),
];