import struct
import sys

if len(sys.argv) != 2:
    print(f"USO python3 {sys.argv[0]} [NOME]")
    quit()

registroCEP = struct.Struct("72s72s72s72s2s8s2s")
NomeColumn = 0

print(f"Tamanho da Estrutura: {registroCEP.size}")
with open("cep.dat", "rb") as f:
    line = f.read(registroCEP.size)
    counter = 0
    sys.argv[1] = sys.argv[1].upper()
    while len(line) > 0:
        record = registroCEP.unpack(line)
        if sys.argv[1] in record[NomeColumn].decode('latin1'):
            counter += 1
            for i in range(len(record)):
                print(record[i].decode('latin1'))
        line = f.read(registroCEP.size)
    f.close()
    print(f"total de registros coincidindo: {counter}")

"""
python cep.py "Renato campos"
Tamanho da Estrutura: 300
RUA RENATO CAMPOS                                                       
CAMPESTRE I                                                             
ITABIRA                                                                 
MINAS GERAIS                                                            
MG
35900078
 
total de registros coincidindo: 1
"""
