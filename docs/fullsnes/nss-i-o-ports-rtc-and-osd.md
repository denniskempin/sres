# NSS I/O Ports - RTC and OSD

#### Real-Time Clock (RTC) and On-Screen Display (OSD) Registers

#### Port 03h.R - Real-Time Clock (RTC) Input (IC31/74HC367)

```text
  7-1 Unknown/unused    (seems to be always 7Eh, ie. all seven bits set)
  0   RTC Data In       (0=Low=Zero, 1=High=One)
```

#### Port 02h/82h/72h/EAh.W - RTC and OSD (IC45/74HC377)

```text
  7   OSD Clock ?       (usually same as Bit6)  ;\Chip Select when Bit6=Bit7 ?
  6   OSD Clock ?       (usually same as Bit7)  ;/
  5   OSD Data Out      (0=Low=Zero, 1=High=One)
  4   OSD Special       (?)  ... or just /CS ? (or software index DC3F/DD3F?)
  3   RTC /CLK          (0=Low=Clock,  1=High=Idle)              ;S-3520
  2   RTC Data Out      (0=Low=Zero,   1=High=One)
  1   RTC Direction     (0=Low=Write,  1=High=Read)
  0   RTC /CS           (0=Low/Select, 1=High/No)
```

RTC is accessed via "Port 82h", OSD via "Port 02h/72h/EAh". For OSD access, the BIOS toggles a LOT of data (and address) lines; not quite clear which of those lines are OSD CLK and OSD Chip Select.

#### RTC Real-Time Clock (S-3520)

The NSS-BIOS supports year 1900..2099 (century 00h=19xx, FFh=20xx is stored in RAM at 8F8Dh/978Dh/9F8Dh; in the two version "03" BIOSes). The current time is shown when pressing Restart in the Bookkeeping screen.

> **See:** [RTC S-3520 (Real-Time Clock)](rtc-s-3520-real-time-clock.md)

#### OSD On-Screen Display (M50458-001SP)

> **See:** [NSS On-Screen Controller (OSD)](nss-on-screen-controller-osd.md)
