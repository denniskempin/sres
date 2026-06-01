# SNES DMA and HDMA Start/Enable Registers

```text
  DMA and HDMA Transfer order is Channel 0 first... Channel 7 last
  HDMA has higher prio than DMA
  HDMA is running even during Forced Blank.

420Bh - MDMAEN - Select General Purpose DMA Channel(s) and Start Transfer (W)
  7-0   General Purpose DMA Channel 7-0 Enable (0=Disable, 1=Enable)
```

When writing a non-zero value to this register, general purpose DMA will be started immediately (after a few clk cycles). The CPU is paused during the transfer. The transfer can be interrupted by H-DMA transfers. If more than 1 bit is set in MDMAEN, then the separate transfers will be executed in order channel 0=first through 7=last. The MDMAEN bits are cleared automatically at transfer completion.

Do not use channels for GP-DMA which are activated as H-DMA in HDMAEN.

```text
420Ch - HDMAEN - Select H-Blank DMA (H-DMA) Channel(s) (W)
  7-0   H-DMA Channel 7-0 Enable (0=Disable, 1=Enable)
```

...
