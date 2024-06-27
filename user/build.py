import os

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('src/bin')
apps.sort()
for app in apps:
    app = app[:app.find('.')]  # 去掉后缀
    lines = []  # lines_after
    lines_before = []  # lines_before
    with open(linker, 'r') as f:
        for line in f.readlines():  # line 是 linker.ld 的每一行
            lines_before.append(line)
            line = line.replace(hex(base_address), hex(base_address+ step * app_id))  # 如果是 base_address 那么就替换成 base_address + step * app_id
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)
    os.system('cargo build --bin %s --release' % app)
    print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
    app_id = app_id + 1
