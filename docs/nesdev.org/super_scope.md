---
title: "Super Scope"
source_url: "https://snes.nesdev.org/wiki/Super_Scope"
pageid: 191
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The Super Scope is a Standard Peripheral that needs to be plugged into the Second Controller Port. This is because when the second [[Controller connector|controller port]]'s [[MMIO registers#RDIO|I/O line]] is asserted by the Super Scope, the [[PPU registers#OPVCT|PPU updates registers OPHCT and and OPVCT]] with the current screen coordinate that is being rendered.

## Interface

The Super Scope reports 16 bits of data, similar to the Standard Controller.
It sends its data on the First Data line, and the Second one is disconnected.

When using automatic reads, the data can be read like this:

```
  JOY2H     JOY2L
  $421B     $421A
15 bit  8 7  bit  0
---- ---- ---- ----
FCTP 00nN 1111 1111
|||| |||| |||| ||||
|||| |||| |||| ++++- Signature
|||| |||| ++++------ Always High
|||| |||+----------- Null Flag
|||| ||+------------ Noise Flag
|||| ++------------- Always Low
|||+---------------- Pause Button
||+----------------- Turbo Switch
|+------------------ Cursor Button
+------------------- Fire Button
```

When manually read through JOYSER1, the Fire Button will be sent first, followed by the Cursor, and so on. Additional bits will read as high, similar to the Standard Controller.

The Null Flag is set when the receiver is active but the Super Scope is not pointed at the screen.
The Noise Flag is set when the Super Scope is active but is sending garbage to the sensor.

The Super Scope itself has a lens, which allows it to only look at a small section of the screen at a time.
Because red light tends to last longer on CRTs, it is filtered out as well.
The filtered light is sent to a light detector, which sends its data to a small motherboard that handles button presses as well.
It generates a signal for the infrared LEDs on the front of the Super Scope to send to the receiver.

One of six different codes is sent.

```
         Pause: 10001
   Single Fire: 11001
    Turbo Fire: 11101
        Cursor: 10011
Turbo + Cursor: 11111
Single + Turbo: 11011
```

If multiple commands need to be sent at once, these values are ORed together. Because of this, the Fire and Cursor buttons overwrite the Pause button.
Each digit is comprised of 8 pulses. For a bit to be read as high, at least 5 of those pulses need to be high.

Each command takes 5.5ms to send, which is about 1/3 of a frame.

After this command is sent and successfully received, the detector is enabled, and the receiver will prepare to receive light data. For the next 85ms (or about 5 frames), the signal from the light detector will be directly sent to the LEDs. Each time the CRT scanning beam crosses the sensor, the LEDs are activated. When the LED has been activated 6 times, the receiver will push the I/O line high and then low to latch the screen coordinates. These values will approximate where the Super Scope was held during the 5 frames.

There is some delay to this signal, which is why games utilizing the Super Scope had calibration screens.

10ms after this, the process is reset and the next data can be fetched. If the Cursor button and/or the Fire button is pressed while the turbo switch is on, the process restarts immediately. Otherwise, it idles until another button is pressed.

An image needs to contain at least 40% green and/or at least 60% blue to be detectable by the Super Scope.

## Source

<https://youtu.be/2Dw7NFm1ZfY?feature=shared&t=977>
