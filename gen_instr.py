class Instr:
    def __init__(self, x):
        self.x = x

    def __repr__(self):
        return '{:08x}'.format(self.x)

class RInstr(Instr):
    def __init__(self, opcode, rt, rs, rd, shamt, funct):
        x =  funct & ((1 << 6) - 1)
        x |= (shamt & ((1 << 5) - 1)) << 6
        x |= (rd & ((1 << 5) - 1)) << 11
        x |= (rs & ((1 << 5) - 1)) << 16
        x |= (rt & ((1 << 5) - 1)) << 21
        x |= (opcode & ((1 << 6))) << 26

        super().__init__(x)

class IInstr(Instr):
    def __init__(self, opcode, rt, rs, imm):
        x =  imm & ((1 << 16) - 1)
        x |= (rs & ((1 << 5) - 1)) << 16
        x |= (rt & ((1 << 5) - 1)) << 21
        x |= (opcode & ((1 << 6) - 1)) << 26

        super().__init__(x)

class JInstr(Instr):
    def __init__(self, opcode, addr):
        x =  addr & ((1 << 26) - 1)
        x |= (opcode & ((1 << 6) - 1)) << 26

        super().__init__(x)

if __name__ == '__main__':
    ins = [
        RInstr(0x0, 0x1, 0x2, 0x3, 0x0, 0x20),
        IInstr(0x9, 0x1, 0x2, 0x322),
        JInstr(0x2, 0x00400024)
        ]

    print(ins)
