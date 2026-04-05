# SNES Cart X-Band I/O - Rockwell Modem Ports

Below are the I/O Ports of the Rockwell chip. In the SNES, Rockwell registers 00h-1Fh are mapped to EVEN memory addresses at FBC180h-FBC1BEh. The chip used in the SNES supports data/voice modem functions (but not fax modem functions).

#### FBC180h/FBC182h - 00h/01h - Receive Data Buffer

```text
  0-7  RBUFFER Received Data Buffer. Contains received byte of data
  8    RXP     Received Parity bit (or ninth data bit)
  9-15 N/A     Unused
```

#### FBC184h/FBC186h - 02h/03h - Control

```text
  0-8  N/A     Unused
  9    GTE     TX 1800Hz Guard Tone Enable (CCITT configuration only)
  10   SDIS    TX Scrambler Disable
  11   ARC     Automatic on-line Rate Change sequence Enable
  12   N/A     Unused
  13   SPLIT   Extended Overspeed TX/RX Split. Limit TX to basic overspeed rate
  14   HDLC    High Level HDLC Protocol Enable (in parallel data mode)
  15   NRZIE   Unknown (listed in datasheet without further description)
```

#### FBC188h/FBC18Ah - 04h/05h - Control

```text
  0     CRFZ   Carrier Recovery Freeze. Disable update of receiver's carrier
               recovery phase lock loop
  1     AGCFZ  AGC Freeze. Inhibit updating of receiver AGC
  2     IFIX   Eye Fix. Force EYEX and EYEY serial data to be rotated
               equalizer output
  3     EQFZ   Equalizer Freeze. Inhibit update of receiver's adaptive
               equalizer taps
  4-5   N/A    Unused
  6     SWRES  Software Reset. Reinitialize modem to its power turn-on state
  7     EQRES  Equalizer Reset. Reset receiver adaptive equalizer taps to zero
  8     N/A    Unused
  9     TXVOC  Transmit Voice. Enable sending of voice samples
  10    RCEQ   Receiver Compromise Equalizer Enable. Control insertion of
               receive passband digital compromise equalizer into receive path
  11    CEQ(E) Compromise Equalizer Enable. Enable transmit passband digital
               compromise equalizer
  12    TXSQ   Transmitter Squelch. Disable transmission of energy
  13-15 N/A    Unused
```

#### FBC18Ch/FBC18Eh - 06h/07h - Control

```text
  0,1 WDSZ   Data Word Size, in asynchronous mode (5, 6, 7, or 8 bits)
  2   STB    Stop Bit Number (number of stop bits in async mode)
  3   PEN    Parity Enable (generate/check parity in async parallel data mode)
  4,5 PARSL  Parity Select (stuff/space/even/odd in async parallel data mode)
  6   EXOS   Extended Overspeed. Selects extended overspeed mode in async mode
  7   BRKS   Break Sequence. Send of continuous space in parallel async mode
  8   ABORT  HDLC Abort. Controls sending of continuous mark in HDLC mode
  9   RA     Relay A Activate. Activate RADRV output
  10  RB     Relay B Activate. Activate RBDVR output
  11  L3ACT  Loop 3 (Local Analog Loopback) Activate. Select connection of
             transmitter's analog output Internally to receiver's analog input
  12  N/A    Unused
  13  L2ACT  Loop 2 (Local Digital Loopback) Activate. Select connection of
             receiver's digital output Internally to transmitter's digital
             input (locally activated digital loopback)
  14  RDL    Remote Digital Loopback Request. Initiate a request for remote
             modem to go into digital loop-back
  15  RDLE   Remote Digital Loopback Response Enable. Enable modem to respond
             to remote modem's digital loopback request
```

#### FBC190h/FBC192h - 08h/09h - Control

```text
  0   RTS    Request to Send. Request transmitter to send data
  1   RTRN   Retrain. Send retrain-request or auto-rate-change to remote modem
  2   N/A    Unused
  3   TRFZ   Timing Recovery Freeze. Inhibit update of receiver's timing
             recovery algorithm
  4   DDIS   Descrambler Disable. Disable receiver's descrambler circuit
  5   N/A    Unused
  6   TPDM   Transmitter Parallel Data Mode. Select parallel/serial TX mode
  7   ASYNC  Asynchronous/Synchronous. Select sync/async data mode
  8   SLEEP  Sleep Mode. Enter SLEEP mode (wakeup upon pulse on RESET pin)
  9   N/A    Unused
  10  DATA   Data Mode. Select idle or data mode
  11  LL     Leased Line. Select leased line data mode or handshake mode
  12  ORG    Originate. Select originate or answer mode (see TONEC)
  13  DTMF   DTMF Dial Select. Select DTMF or Pulse dialing in dial mode
  14  CC     Controlled Carrier. Select controlled or constant carrier mode
  15  NV25   Disable V.25 Answer Sequence (Data Modes), Disable Echo Suppressor
             Tone (Fax Modes). Disable transmitting of 2100Hz CCITT answer tone
             when a handshake sequence is initiated in a data mode or disables
             sending of echo suppressor tone in a fax mode
```

#### FBC194h/FBC196h - 0Ah/0Bh - Status

```text
  0   CRCS   CRC Sending. Sending status of 2-byte CRC in HDLC mode
  1-7 N/A    Unused
  8   BEL1O3 Bell 103 Mark Frequency Detected. Status of 1270Hz Bell 103 mark
  9   DTDET  DTMF Digit Detected. Valid DTFM digit has been detected
  10  PNSUC  PN Success. Receiver has detected PN portion of training sequence
  11  ATBELL Bell Answer Tone Detected. Detection status of 2225Hz answer tone
  12  ATV25  V25 Answer Tone Detected. Detection status of 2100Hz answer tone
  13  TONEC  Tone Filter C Energy Detected. Status of 1650Hz or 980Hz (selected
             by ORG bit) FSK tone energy detection by Tone C bandpass filter in
             Tone Detector configuration
  14  TONEB  Tone Filter B Energy Detected. Status of 390Hz FSK tone energy
             detection by Tone B bandpass filter in Tone Detector configuration
  15  TONEA  Tone Filter A Energy Detected. Status of energy above threshold
             detection by Call Progress Monitor filter in Dial Configuration or
             1300 Hz FSK tone energy detection by Tone A bandpass filter in
             Tone Detector configuration
```

#### FBC198h/FBC19Ah - 0Ch/0Dh - Status

```text
  0-3 DTDIG  Detected DTMF Digit. Hexadecimal code of detected DTMF digit
  4-6 N/A    Unused
  7   EDET   Early DTMF Detect. High group frequency of DTMF tone pair detected
  8-9 N/A    Unused
  10  SADET  Scrambled Alternating Ones Sequence Detected
  11  U1DET  Unscrambled Ones Sequence Detected
  12  SCR1   Scrambled Ones Sequence Detected
  13  S1DET  S1 Sequence Detected
  14  PNDET  Unknown (listed in datasheet without further description)
  15  N/A    Unused
```

#### FBC19Ch/FBC19Eh - 0Eh/0Fh - Status

```text
  0-2 SPEED  Speed Indication. Data rate at completion of a connection
  3   OE     Overrun Error. Overrun status of Receiver Data Buffer (RBUFFER)
  4   FE     Framing Error. Framing error or detection of an ABORT sequence
  5   PE     Parity Error. Parity error status or bad CRC
  6   BRKD   Break Detected. Receipt status of continuous space
  7   RTDET  Retrain Detected. Detection status of a retrain request sequence
  8   FLAGS  Flag Sequence. Transmission status of Flag sequence in HDLC mode,
             or transmission of a constant mark in parallel asynchronous mode
  9   SYNCD  Unknown (listed in datasheet without further description)
  10  TM     Test Mode. Active status of selected test mode
  11  RI     Ring Indicator. Detection status of a valid ringing signal
  12  DSR    Data Set Ready. Data transfer state
  13  CTS    Clear to Send. Training sequence has been completed (see TPDM)
  14  FED    Fast Energy Detected. Energy above turn-on threshold is detected
  15  RLSD   Received Line Signal Detector (carrier and receipt of valid data)
```

#### FBC1A0h/FBC1A2h - 10h/11h - Transmit Data Buffer

```text
  0-7   TBUFFER Transmitter Data Buffer. Byte to be sent in parallel mode
  8     TXP     Transmit Parity Bit (or 9th Data Bit)
  9-15  N/A     Unused
```

#### FBC1A4h/FBC1A6h - 12h/13h - Control

```text
  0-7   CONF   Modem Configuration Select. Modem operating mode (see below)
  8-9   TXCLK  Transmit Clock Select (internal, disable, slave, or external)
  10-11 VOL    Volume Control. Speaker volume (off, low, medium, high)
  12-15 TLVL   Transmit Level Attenuation Select. Select transmitter analog
               output level attenuation in 1 dB steps. The host can fine tune
               transmit level to a value lying within a 1 dB step in DSP RAM
```

#### FBC1A8h/FBC1AAh - 14h/15h - Unused

```text
  0-15  N/A    Unused
```

FBC1ACh/FBC1AEh - 16h/17h - Y-RAM Data (16bit) FBC1B0h/FBC1B2h - 18h/19h - X-RAM Data (16bit)

```text
  0-15  DATA  RAM data word (R/W)
```

#### FBC1B4h/FBC1B6h - 1Ah/1Bh - Y-RAM Addresss/Control

#### FBC1B8h/FBC1BAh - 1Ch/1Dh - X-RAM Addresss/Control

```text
  0-8   ADDR  RAM Address
  9     WT    RAM Write (controls read/write direction for RAM Data registers)
  10    CRD   RAM Continuous Read. Enables read of RAM every sample from
              location addressed by ADDR Independent of ACC and WT bits
  11    IOX   X-RAM only: I/O Register Select. Specifies that X RAM ADDRESS
              bit0-7 (Port 1Ch) is an internal I/O register address
  11    N/A   Y-RAM only: Unused
  12-14 N/A   Unused
  15    ACC   RAM Access Enable. Controls DSP access of RAM associated with
              address ADDR bits. WT determines if a read or write is performed
```

#### FBC1BCh/FBC1BEh - 1Eh/1Fh - Interrupt Handling

```text
  0   RDBF    Receiver Data Buffer Full (RBUFFER Full)
  1   N/A     Unused
  2   RDBIE   Receiver Data Buffer Full Interrupt Enable
  3   TDBE    Transmitter Data Buffer Empty (TBUFFER Empty)
  4   N/A     Unused
  5   TDBIE   Transmitter Data Buffer Empty Interrupt
  6   RDBIA   Receiver Data Buffer Full Interrupt Active (IRQ Flag)
  7   TDBIA   Transmitter Data Buffer Empty Interrupt Active (IRQ Flag)
  8   NEWC    New Configuration. Initiates new configuration (cleared by modem
  9   N/A     Unused                   upon completion of configuration change)
  10  NCIE    New Configuration Interrupt Enable
  11  NEWS    New Status. Detection of a change in selected status bits
  12  NSIE    New Status Interrupt Enable
  13  N/A     Unused
  14  NCIA    New Configuration Interrupt Active (IRQ Flag)
  15  NSIA    New Status Interrupt Active (IRQ Flag)
```

#### CONF Values

Below are CONF values taken from RC96DT/RC144DT datasheet (the RC96V24DP/RC2324DPL datasheet doesn't describe CONF values). Anyways, the values hopefully same for both chip versions (except that, the higher baudrates obviously won't work on older chips).

```text
  CONF  Bits/sec Mode Name
  01h   2400     V.27 ter
  02h   4800     V.27 ter
  11h   4800     V.29
  12h   7200     V.29
  14h   9600     V.29
  52h   1200     V.22
  51h   600      V.22
  60h   0-300    Bell 103
  62h   1200     Bell 212A
  70h   -        V.32 bis/V.23 clear down    ;\
  71h   4800     V.32                        ;
  72h   12000    V.32 bis TCM                ; RC96DT/RC144DT only
  74h   9600     V.32 TCM                    ; (not RC96V24DP/RC2324DPL)
  75h   9600     V.32                        ;
  76h   14400    V.32 bis TCM                ;
  78h   7200     V.32 bis TCM                ;/
  80h   -        Transmit Single Tone
  81h   -        Dialing                  ;used by SNES X-Band (dial mode)
  82h   1200     V.22 bis
  83h   -        Transmit Dual Tone
  84h   2400     V.22 bis                 ;used by SNES X-Band (normal mode)
  86h   -        DTMF Receiver
  A0h   0-300    V.21
  A1h   75/1200  V.23 (TX/RX)
  A4h   1200/75  V.23 (TX/RX)
  A8h   300      V.21 channel 2
  B1h   14400    V.17 TCM                    ;\
  B2h   12000    V.17 TCM                    ; RC96DT/RC144DT only
  B4h   9600     V.17 TCM                    ; (not RC96V24DP/RC2324DPL)
  B8h   7200     V.17 TCM                    ;/
```

#### XBand X/Y RAM Rockwell

Below are X/Y RAM addresses that can be accessed via Ports 16h-1Dh.

Addresses 000h-0FFh are "Data RAM", 100h-1FFh are "Coefficient RAM".

X-RAM is "Real RAM", Y-RAM is "Imaginary RAM" (whatever that means).

```text
  XRAM     YRAM     Parameter
  032      -        Turn-on Threshold
  03C      -        Lower Part of Phase Error (this, in X RAM ?)
  -        03C      Upper Part of Phase Error (this, in Y RAM ?)
  -        03D      Rotation Angle for Carrier Recovery
  03F      -        Max AGC Gain Word
  049      049      Rotated Error, Real/Imaginary
  059      059      Rotated Equalizer Output, Real/Imaginary
  05E      05E      Real/Imaginary Part of Error
  06C      -        Tone 1 Angle Increment Per Sample (TXDPHI1)
  06D      -        Tone 2 Angle Increment Per Sample (TXDPHI2)
  06E      -        Tone 1 Amplitude (TXAMP1)
  06F      -        Tone 2 Amplitude (TXAMP2)
  070      -        Transmit Level Output Attenuation
  071      -        Pulse Dial Interdigit Time
  072      -        Pulse Dial Relay Make Time
  073      -        Max Samples Per Ring Frequency Period (RDMAXP)
  074      -        Min Samples Per Ring Frequency Period (RDMINP)
  07C      -        Tone Dial Interdigit Time
  07D      -        Pulse Dial Relay Break Time
  07E      -        DTMF Duration
  110-11E  100-11E  Adaptive Equalizer Coefficients, Real/Imag.
  110      100      First coefficient, Real/Imag. (1) (Data/Fax)
  110      110      Last Coefficient, Real/Imag. (17) (Data)
  11E      11E      Last Coefficient, Real/Imag. (31) (Fax)
  -        121      RLSD Turn-off Time
  12D      -        Phase Error
  12E      -        Average Power
  12F      -        Tone Power (TONEA)
  130      -        Tone Power (TONEB,ATBELL,BEL103)
  131      -        Tone Power (TONEC,ATV25)
  136      -        Tone Detect Threshold for TONEA               (THDA)
  137      -        Tone Detect Threshold for TONEB,ATBELL,BEL103 (THDB)
  138      -        Tone Detect Threshold for TONEC,ATV25         (THDC)
  13E      -        Lower Part of AGC Gain Word
  13F      -        Upper Part of AGC Gain Word
  152      -        Eye Quality Monitor (EQM)
  -        162-166  Biquad 5 Coefficients a0,a1,a2,b1,b2
  -        167-16B  Biquad 6 Coefficients a0,a1,a2,b1,b2
  -        16C-170  Biquad 1 Coefficients a0,a1,a2,b1,b2
  -        171-175  Biquad 2 Coefficients a0,a1,a2,b1,b2
  -        176-17A  Biquad 3 Coefficients a0,a1,a2,b1,b2
  179      -        Turn-off Threshold
  -        17B-17F  Biquad 4 Coefficients a0,a1,a2,b1,b2
```
