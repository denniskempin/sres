# Z80 Arithmetic/Logical Commands

8bit Arithmetic/Logical Commands  Instruction    Opcode  Cycles Flags  Notes  daa            27           4 szxp-x decimal adjust akku  cpl            2F           4 --1-1- A = A xor FF  neg            ED 44        8 szho1c A = 00-A  <arit>  r      xx           4 szhonc see below  <arit>  i      pD xx        8 szhonc see below, UNDOCUMENTED  <arit>  n      xx nn        7 szhonc see below  <arit>  (HL)   xx           7 szhonc see below  <arit>  (ii+d) pD xx dd    19 szhonc see below  <cnt>   r      xx           4 szhon- see below  <cnt>   i      pD xx        8 szhon- see below, UNDOCUMENTED  <cnt>   (HL)   xx          11 szhon- see below  <cnt>   (ii+d) pD xx dd    23 szhon- see below  <logi>  r      xx           4 szhp00 see below  <logi>  i      pD xx        8 szhp00 see below, UNDOCUMENTED  <logi>  n      xx nn        7 szhp00 see below  <logi>  (HL)   xx           7 szhp00 see below  <logi>  (ii+d) pD xx dd    19 szhp00 see below Arithmetic <arit> commands:

add   A,op     see above 4-19 szho0c A=A+op  adc   A,op     see above 4-19 szho0c A=A+op+cy  sub   op       see above 4-19 szho1c A=A-op  sbc   A,op     see above 4-19 szho1c A=A-op-cy  cp    op       see above 4-19 szho1c compare, ie. VOID=A-op Increment/Decrement <cnt> commands:

inc   op       see above 4-23 szho0- op=op+1  dec   op       see above 4-23 szho1- op=op-1 Logical <logi> commands:

and   op       see above 4-19 sz1p00 A=A & op  xor   op       see above 4-19 sz0p00 A=A XOR op  or    op       see above 4-19 sz0p00 A=A | op

16bit Arithmetic Commands  Instruction    Opcode  Cycles Flags  Notes  add  HL,rr     x9          11 --h-0c HL = HL+rr    ;rr may be BC,DE,HL,SP  add  ii,rr     pD x9       15 --h-0c ii = ii+rr    ;rr may be BC,DE,ii,SP (!)  adc  HL,rr     ED xA       15 szho0c HL = HL+rr+cy ;rr may be BC,DE,HL,SP  sbc  HL,rr     ED x2       15 szho1c HL = HL-rr-cy ;rr may be BC,DE,HL,SP  inc  rr        x3           6 ------ rr = rr+1     ;rr may be BC,DE,HL,SP  inc  ii        pD 23       10 ------ ii = ii+1  dec  rr        xB           6 ------ rr = rr-1     ;rr may be BC,DE,HL,SP  dec  ii        pD 2B       10 ------ ii = ii-1

Searchcommands  Instruction    Opcode  Cycles Flags  Notes  cpi            ED A1       16 szhe1- compare A-(HL), HL=HL+1, DE=DE+1, BC=BC-1  cpd            ED A9       16 szhe1- compare A-(HL), HL=HL-1, DE=DE-1, BC=BC-1  cpir           ED B1   x*21-5 szhe1- cpi-repeat until BC=0 or compare fits  cpdr           ED B9   x*21-5 szhe1- cpd-repeat until BC=0 or compare fits
