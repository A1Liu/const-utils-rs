with open('src/base.rs', 'r') as f:
    code = f.read()

for size in ['8', '16', '32', '64', 'size', '128']:
    utype = 'u%s' % size
    itype = 'i%s' % size
    with open(f'src/{utype}_func.rs','w') as f:
        f.write(code.replace('u8', utype))
    with open(f'src/{itype}_func.rs','w') as f:
        f.write(code.replace('u8', itype))

with open('src/usize_func.rs', 'w') as f:
    f.write(code.replace('const_utils::u8', 'const_utils').replace('u8','usize'))
