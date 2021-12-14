import struct
import sys

if len(sys.argv) != 2:
    print("USO python3 {} [NOME]".format(sys.argv[0]))
    quit()

registroCEP = struct.Struct("72s72s72s72s2s8s2s")
NomeColumn = 0

print("Tamanho da Estrutura: {}".format(registroCEP.size))
with open("cep.dat", "rb") as f:
    line = f.read(registroCEP.size)
    counter = 0
    while len(line) > 0:
        record = registroCEP.unpack(line)
        if sys.argv[1].upper() in record[NomeColumn].decode('latin1'):
            counter += 1
            for i in range(len(record)):
                print(record[i].decode('latin1'))
        line = f.read(registroCEP.size)
    f.close()
    print(f"total de registros coincidindo: {counter}")
