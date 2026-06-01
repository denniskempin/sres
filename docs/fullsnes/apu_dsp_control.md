# SNES APU DSP Control Registers

4Ch - KON - Key On Flags for Voice 0..7 (R/W) (W)

```text
  0-7  Flags for Voice 0..7 (0=No change, 1=Key On)

        Writing 1 to the KON bit will set the envelope to 0, the state to
        Attack, and will start the channel from the beginning (see DIR and
        VxSRCN). Note that this happens even if the channel is already playing
        (which may cause a click/pop), and that there are 5 'empty' samples
        before envelope updates and BRR decoding actually begin.
```

5Ch - KOFF - Key Off Flags for Voice 0..7 (R/W) (W)

```text
  0-7  Flags for Voice 0..7 (0=No change, 1=Key Off)

        Setting 1 to the KOFF bit will transition the voice to the Release
        state. Thus, the envelope will decrease by 8 every sample (regardless
        of the VxADSR and VxGAIN settings) until it reaches 0, where it will
        stay until the next KON.
```

6Ch - FLG - Reset, Mute, Echo-Write flags and Noise Clock (R/W) The initial value on Reset is E0h (KeyedOff/Muted/EchoWriteOff/NoiseStopped), although reading the FLG register after reset will return a garbage value.

```text
  0-4  Noise frequency    (0=Stop, 1=16Hz, 2=21Hz, ..., 1Eh=16kHz, 1Fh=32kHz)
  5    Echo Buffer Writes (0=Enable, 1=Disable) (doesn't disable echo-reads)
  6    Mute Amplifier     (0=Normal, 1=Mute) (doesn't stop internal processing)
  7    Soft Reset         (0=Normal, 1=KeyOff all voices, and set Envelopes=0)
```

Disabling Echo-Writes doesn't affect Echo-Reads (ie. echo buffer is still output, unless Echo Volume L+R or FIR0..7 are set to zero). Mute affects only the external amplifier (internally all sound/echo generation is kept operating).

7Ch - ENDX - Voice End Flags for Voice 0..7 (R) (W=Ack)

```text
  0-7  Flags for Voice 0..7 (0=Keyed ON, 1=BRR-End-Bit encountered)
```

Any write to this register will clear ALL bits, no matter what value is written. On reset, all bits are cleared. Though bits may get set shortly after reset or shortly after manual-write (once when the BRR decoder reaches an End-code).

```text
        Note that the bit is set at the START of decoding the BRR block, not
        at the end. Recall that BRR processing, and therefore the setting of
        bits in this register, continues even for voices in the Release state.
```

3Dh - NON - Noise Enable Flags for Voice 0..7 (R/W) Allows to enable Noise on one or more channels, however, there is only one noise-generator (and only one noise frequency) shared for all voices. The 5bit noise frequency is defined in FLG register (see above), and works same as the 5bit ADSR rates (see ADSR chapter for details). The NON bits are:

```text
  0-7  Flags for Voice 0..7 (0=Output BRR Samples, 1=Output Noise)
```

The noise generator produces 15bit output level (range -4000h..+3FFFh; initially -4000h after reset). At the selected noise rate, the level is updated as follows:

```text
  Level = ((Level SHR 1) AND 3FFFh) OR ((Level.Bit0 XOR Level.Bit1) SHL 14)
```

Caution: Even in noise mode, the hardware keeps decoding BRR sample blocks, and, when hitting a BRR block with End Code 1 (End+Mute), it will switch to Release state with Envelope=0, thus immediately terminating the Noise output.

To avoid that, set VxSRCN to an endless looping dummy BRR block.

Note: The VxPITCH registers have no effect on noise frequency, and Gaussian interpolation isn't applied on noise.

#### KON/KOFF Notes

```text
        These registers seem to be polled only at 16000 Hz, when every other
        sample is due to be output. Thus, if you write two values in close
        succession, usually but not always only the second value will have an
        effect:
          ; assume KOFF = 0, but no voices playing
          mov $f2, #$4c  ; KON = 1 then KON = 2
          mov $f3, #$01  ; -> *usually* only voice 2 is keyed on. If both are,
          mov $f3, #$02  ; voice 1 will be *2* samples ahead rather than one.
        and
          ; assume various voices playing
          mov $f2, #$5c  ; KOFF = $ff then KOFF = 0
          mov $f3, #$ff
          mov $f3, #$00  ; -> *usually* all voices remain playing
        FLG bit 7, however, is polled every sample and polled for each voice.

        These registers and FLG bit 7 interact as follows:
          1. If FLG bit 7 or the KOFF bit for the channel is set, transition
             to the Release state. If FLG bit 7 is set, also set the envelope
             to 0.
          2. If the 'internal' value of KON has the channel's bit set, perform
             the KON actions described above.
          3. Set the 'internal' value of KON to 0.

        This has a number of consequences:
          * KON effectively takes effect 'on write', even though a non-zero
            value can be read back much later. KOFF and FLG.7, on the other
            hand, exert their influence constantly until a new value is
            written.
          * Writing KON while KOFF or FLG.7 will not result in any samples
            being output by the channel. The channel is keyed on, but it is
            turned off again 2 samples later. Since there is a 5 sample delay
            after KON before the channel actually beings processing, the net
            effect is no output.
          * However, if KOFF is cleared within 63 SPC700 cycles of the
            KON write above, the channel WILL be keyed on as normal. If KOFF
            is cleared betwen 64 and 127 SPC700 cycles later, the channel
            MIGHT be keyed on with decreasing probability depending on how
            many cycles before the KON/KOFF poll the KON write occurred.
          * Setting both KOFF and KON for a channel will turn the channel
            off much faster than just KOFF alone, since the KON will set the
            envelope to 0. This can cause a click/pop, though.
```

xAh - NA - Unused (8 bytes of general-purpose RAM) (R/W) xBh - NA - Unused (8 bytes of general-purpose RAM) (R/W) 1Dh - NA - Unused (1 byte of general-purpose RAM) (R/W) xEh - NA - Unused (8 bytes of general-purpose RAM) (R/W) These registers seem to have no function at all. Data written to them seems to have no effect on sound output, the written values seem to be left intact (ie. they aren't overwritten by voice or echo status information).
