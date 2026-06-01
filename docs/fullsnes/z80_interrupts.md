# Z80 Interrupts

Interrupt Flip-Flop (IFF1,IFF2)

The IFF1 flag is used to enable/disable INTs (maskable interrupts).

In a raw INT-based system, IFF2 is always having the same state than IFF1.

However, in a NMI-based system the IFF2 flag is used to backup the recent IFF1 state prior to NMI execution, and may be used to restore IFF1 upon NMI completion by RETN opcode.

Beside for the above 'backup' function, IFF2 itself is having no effect.

Neither IFF1 nor IFF2 affect NMIs which are always enabled.

The following opcodes/events are modifying IFF1 and/or IFF2:

```text
  EI     IFF1=1, IFF2=1
  DI     IFF1=0, IFF2=0
  <INT>  IFF1=0, IFF2=0
  <NMI>  IFF1=0
  RETI   IFF1=IFF2
  RETN   IFF1=IFF2
```

When using the EI instruction, the new IFF state isn't applied until the next instruction has completed (this ensures that an interrupt handler which is using the sequence "EI, RET" may return to the main program before the next interrupt is executed).

Interrupts can be disabled by the DI instruction (IFF=0), and are additionally automatically each time when an interrupt is executed.

#### Interrupt Execution

An interrupt is executed when an interrupt is requested by the hardware, and IFF is set. Whenever both conditions are true, the interrupt is executed after the completion of the current opcode.

Note that repeated block commands (such like LDIR) can be interrupted also, the interrupt return address on the stack then points to the interrupted opcode, so that the instruction may continue as normal once the interrupt handler returns.

Interrupt Modes (IM 0,1,2)

The Z80 supports three interrupt modes which can be selected by IM 0, IM 1, and IM 2 instructions. The table below describes the respective operation and execution time in each mode.

```text
  Mode  Cycles  Refresh  Operation
  0     1+var   0+var    IFF1=0,IFF2=0, read and execute opcode from databus
  1     12      1        IFF1=0,IFF2=0, CALL 0038h
  2     18      1        IFF1=0,IFF2=0, CALL [I*100h+databus]
```

Mode 0 requires an opcode to be output to the databus by external hardware, in case that no byte is output, and provided that the 'empty' databus is free of garbage, then the CPU might tend to read a value of FFh (opcode RST 38h, 11 cycles, 1 refresh) - the clock cycles (11+1), refresh cycles (1), and executed operation are then fully identical as in Mode 1.

Mode 1 interrupts always perform a CALL 0038h operation. The downside is that many systems may have ROM located at this address, making it impossible to hook the interrupt handler directly.

Mode 2 calls to a 16bit address which is read from a table in memory, the table pointer is calculated from the "I" register (initialized by LD I,A instruction) multiplied by 100h, plus an index byte which is read from the databus. The following trick may be used to gain stable results in Mode 2 even if no index byte is supplied on the databus: For example, set I=40h the origin of the table will be then at 4000h in memory. Now fill the entire area from 4000h to 4100h (101h bytes, including 4100h) by the value 41h. The CPU will then perform a CALL 4141h upon interrupt execution - regardless of whether the randomized index byte is an even or odd number.

#### Non-Maskable Interrupts (NMIs)

Unlike INTs, NMIs cannot be disabled by the CPU, ie. DI and EI instructions and the state of IFF1 and IFF2 do not have effect on NMIs. The NMI handler address is fixed at 0066h, regardless of the interrupt mode (IM). Upon NMI execution, IFF1 is cleared (disabeling maskable INTs - NMIs remain enabled, which may result in nested execution if the handler does not return before next NMI is requested). IFF2 remains unchanged, thus containing the most recent state of IFF1, which may be used to restore IFF1 if the NMI handler returns by RETN instruction.

Execution time for NMIs is unknown (?).

RETN (return from NMI and restore IFF1) Intended to return from NMI and to restore the old IFF1 state (assuming the old state was IFF1/IFF2 both set or both cleared).

RETI (return from INT with external acknowledge) Intended to return from INT and to notify peripherals about completion of the INT handler, the Z80 itself doesn't send any such acknowledge signal (instead, peripherals like Z80-PIO or Z80-SIO must decode the databus during /M1 cycles, and identify the opcode sequence EDh,4Fh as RETI). Aside from such external handling, internally, RETI is exactly same as RETN, and, like RETN it does set IFF1=IFF2 (though in case of RETI this is a dirt effect without practical use; within INT handlers IFF1 and IFF2 are always both zero, or when EI was used both set). Recommended methods to return from INT are: EI+RETI (when needing the external acknowledge), or EI+RET (faster).
