def do_rtype():
    opcode = int(input('Enter opcode: '), 16)
    rs = int(input('Enter rs: '), 16)
    rt = int(input('Enter rt: '), 16)
    rd = int(input('Enter rd: '), 16)
    shamt = int(input('Enter shamt: '), 16)
    funct = int(input('Enter funct: '), 16)

    return make_rtype(opcode, rs, rt, rd, shamt, funct)

def do_itype():
    opcode = int(input('Enter opcode: '), 16)
    rs = int(input('Enter rs: '), 16)
    rt = int(input('Enter rt: '), 16)
    imm = int(input('Enter imm: '), 16)

    return make_itype(opcode, rs, rt, imm)

def do_jtype():
    opcode = int(input('Enter opcode: '), 16)
    addr = int(input('Enter addr: '), 16)

    return make_jtype(opcode, addr)

def make_rtype(opcode, rs, rt, rd, shamt, funct):
    i = mask(opcode, 26, 6)
    i |= mask(rs, 21, 5)
    i |= mask(rt, 16, 5)
    i |= mask(rd, 11, 5)
    i |= mask(shamt, 6, 5)
    i |= mask(funct, 0, 6)

    return i


def make_itype(opcode, rs, rt, imm):
    i = mask(opcode, 26, 6)
    i |= mask(rs, 21, 5)
    i |= mask(rt, 16, 5)
    i |= mask(imm, 0, 16)

    return i

def make_jtype(opcode, addr):
    i = mask(opcode, 26, 6)
    i |= mask(addr, 0, 26)

    return i

def mask(x, offset, length):
    m = (1 << length) - 1
    return (x & m) << offset

INSTR_TYPES = {
    'r': do_rtype,
    'i': do_itype,
    'j': do_jtype
}

if __name__ == '__main__':
    selected = False

    while not selected:
        instr_type = input('What type of instruction? [r/i/j] ')
        selected = instr_type in INSTR_TYPES

    print()
    print('{:08x}'.format(INSTR_TYPES[instr_type]()))
