# ARM Versions

#### Version Numbers

ARM CPUs are distributed by name ARM#, and are described as ARMv# in specifications, whereas "#" is NOT the same than "v#", for example, ARM7TDMI is ARMv4TM. That is so confusing, that ARM didn't even attempt to clarify the relationship between the various "#" and "v#" values.

#### Version Variants

Suffixes like "M" (long multiply), "T" (THUMB support), "E" (Enhanced DSP) indicate presence of special features, additionally to the standard instruction set of a given version, or, when preceded by an "x", indicate the absence of that features.

#### ARMv1 aka ARM1

Some sort of a beta version, according to ARM never been used in any commercial products.

#### ARMv2 and up

MUL,MLA

CDP,LDC,MCR,MRC,STC

#### SWP/SWPB (ARMv2a and up only)

#### Two new FIQ registers

#### ARMv3 and up

MRS,MSR opcodes (instead CMP/CMN/TST/TEQ{P} opcodes)

CPSR,SPSR registers (instead PSR bits in R15) Removed never condition, cond=NV no longer valid 32bit addressing (instead 26bit addressing in older versions) 26bit addressing backwards comptibility mode (except v3G) Abt and Und modes (instead handling aborts/undefined in Svc mode) SMLAL,SMULL,UMLAL,UMULL (optionally, INCLUDED in v3M, EXCLUDED in v4xM/v5xM)

#### ARMv4 aka ARM7 and up

LDRH,LDRSB,LDRSH,STRH

#### Sys mode (privileged user mode)

BX (only ARMv4T, and any ARMv5 or ARMv5T and up) THUMB code (only T variants, ie. ARMv4T, ARMv5T)

#### ARMv5 aka ARM9 and up

BKPT,BLX,CLZ (BKPT,BLX also in THUMB mode)

LDM/LDR/POP PC with mode switch (POP PC also in THUMB mode) CDP2,LDC2,MCR2,MRC2,STC2 (new coprocessor opcodes) C-flag unchanged by MUL (instead undefined flag value) changed instruction cycle timings / interlock ??? or not ???

QADD,QDADD,QDSUB,QSUB opcodes, CPSR.Q flag (v5TE and V5TExP only) SMLAxy,SMLALxy,SMLAWy,SMULxy,SMULWy (v5TE and V5TExP only) LDRD,STRD,PLD,MCRR,MRRC (v5TE only, not v5, not v5TExP)

#### ARMv6

No public specifications available.

#### A Milestone in Computer History

Original ARMv2 has been used in the relative rare and expensive Archimedes deluxe home computers in the late eighties, the Archimedes has caught a lot of attention, particularly for being the first home computer that used a BIOS being programmed in BASIC language - which has been a absolutely revolutionary decadency at that time.

Inspired, programmers all over the world have successfully developed even slower and much more inefficient programming languages, which are nowadays consequently used by nearly all ARM programmers, and by most non-ARM programmers as well.
