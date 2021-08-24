from building import *

cwd     = GetCurrentDir()
src     = ["rust_hello.c"]
CPPPATH = [cwd]

LIBS = ["libhello.a"]
LIBPATH = [GetCurrentDir() + '/example']

group = DefineGroup('Applications', src, depend = [''], CPPPATH = CPPPATH, LIBS = LIBS, LIBPATH = LIBPATH)

Return('group')
