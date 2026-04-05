# SNES Cart SA-1 Interrupt/Control on SA-1 Side

#### 2209h SA-1 SCNT - SNES CPU Control (W)

```text
  0-3 Message from SA-1 to SNES (4bit value)
  4   NMI Vector for SNES (0=ROM FFEAh, 1=Port 220Ch)
  5   Not used (should be 0)
  6   IRQ Vector for SNES (0=ROM FFEEh, 1=Port 220Eh)
  7   IRQ from SA-1 to SNES   (0=No Change?, 1=Interrupt)
```

#### 220Ah SA-1 CIE - SA-1 CPU Int Enable (W)

```text
  0-3 Not used (should be 0)
  4   NMI Enable (from SNES)  (0=Disable, 1=Enable)
  5   IRQ Enable (from DMA)   (0=Disable, 1=Enable)
  6   IRQ Enable (from Timer) (0=Disable, 1=Enable)
  7   IRQ Enable (from SNES)  (0=Disable, 1=Enable)
```

#### 220Bh SA-1 CIC - SA-1 CPU Int Clear (W)

```text
  0-3 Not used (should be 0)
  4   NMI Acknowledge (from SNES)  (0=No change, 1=Clear)
  5   IRQ Acknowledge (from DMA)   (0=No change, 1=Clear)
  6   IRQ Acknowledge (from Timer) (0=No change, 1=Clear)
  7   IRQ Acknowledge (from SNES)  (0=No change, 1=Clear)
```

220Ch SA-1 SNV - SNES CPU NMI Vector Lsb (W) 220Dh SA-1 SNV - SNES CPU NMI Vector Msb (W) 220Eh SA-1 SIV - SNES CPU IRQ Vector Lsb (W) 220Fh SA-1 SIV - SNES CPU IRQ Vector Msb (W) Exception Vectors on SNES side (these are optionally replacing the normal vectors in ROM; depending on bits in Port 2209h; the "I/O" vectors are used only by Jumpin Derby, all other games are using the normal ROM vectors).

#### 2301h SA-1 CFR - SA-1 CPU Flag Read (R)

```text
  0-3 Message from SNES to SA-1 (4bit value)       (same as 2200h.Bit0-3)
  4   NMI from SNES to SA-1   (0=No, 1=Interrupt)  (triggered by 2200h.Bit4)
  5   IRQ from DMA to SA-1    (0=No, 1=Interrupt)  (triggered by DMA-finished)
  6   IRQ from Timer to SA-1  (0=No, 1=Interrupt)  (triggered by Timer)
  7   IRQ from SNES to SA-1   (0=No, 1=Interrupt)  (triggered by 2200h.Bit7)
```
