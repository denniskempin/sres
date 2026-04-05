# SNES Controllers Miracle Piano Instruments

#### Available Patches (aka Instruments)

The following patches are available through both Library Select Mode and MIDI control:

```text
  000 Grand Piano     032 Marimba         064 Synth Bells    096 Tube Bells'
  001 Detuned Piano   033 Glockenspiel'   065 Vox 1          097 Frogs/Ducks
  002 FM Piano        034 Kalimba'        066 Vox 2          098 Banjo'
  003 Dyno            035 Tube Bells      067 Vox 3          099 Shakuhachi'
  004 Harpsichord     036 Steel Drums     068 Mod Synth      100 Piano'
  005 Clavinet        037 Log Drums'      069 Pluck Synth    101 Vibraphone'
  006 Organ           038 Strings 1       070 Hard Synth     102 FM Piano'
  007 Pipe Organ      039 Pizzicato       071 Syntar         103 Clock Belis'
  008 Steel Guitar    040 Strings 2       072 Effects 1 *    104 Harpsichord'
  009 12-StringGuitar 041 Violin 1'       073 Effects 2 *    105 Clavinet'
  010 Guitar          042 Trumpet'        074 Percussion 1 * 106 Organ'
  011 Banjo           043 Trumpets        075 Percussion 2 * 107 Pipe Organ'
  012 Mandolin        044 Horn'           076 Percussion 3 * 108 Metal Guitar'
  013 Koto'           045 Horns           077 Sine Organ'    109 Stick'
  014 Jazz Guitar'    046 Trombone'       078 Organ #        110 Guitar'
  015 Clean Guitar'   047 Trombones       079 Pipe Organ #   111 Xylophone'
  016 Chorus Guitar   048 CupMuteTrumpet' 080 Harpsichord #  112 Marimba'
  017 Fuzz Guitar     049 Sfz Brass 1     081 Synth Pad 1    113 Syn Trombone'
  018 Stop Guitar     050 Sfz Brass 2     082 Synth Pad 2    114 Syn Trumpet'
  019 Harp'           051 Saw Synth       083 Synth Pad 3    115 Sfz Brass 1'
  020 Detuned Harp    052 Tuba'           084 Synth Pad 4    116 Sfz Brass 2'
  021 Upright Bass'   053 Harmonica       085 Synth Pad 5    117 Saw Synth'
  022 Slap Bass'      054 Flute'          086 Synth Pad 6    118 Church Bells'
  023 Electric Bass'  055 Pan Flute'      087 Synth Pad 7    119 Marcato'
  024 Moog            056 Calliope        088 Synth Pad 8    120 Marcato
  025 Techno Bass     057 Shakuhachi      089 Synth Pad 9    121 Violin 2'
  026 Digital Waves   058 Clarinet'       090 Synth Pad 10   122 Strings 3
  027 Fretless Bass'  059 Oboe'           091 Synth Pad 11   123 Synth Bells'
  028 Stick Bass      060 Bassoon'        092 Synth Pad 12   124 Techno Bass'
  029 Vibraphone      061 Sax'            093 Synth Pad 13   125 Mod Synth'
  030 MotorVibraphone 062 Church Bells    094 Synth Pad 14   126 Pluck Synth'
  031 Xylophone       063 Big Bells       095 Synth Pad 15   127 Hard Synth'
```

Notes:

' These programs are single voice, which lets The Miracle play up to 16

```text
   notes simultaneously. All other programs are dual voice, which lets it
   play up to 8 notes simultaneously.
```

* 072..076 See below for a list of Effects/Percussion sounds.

# 078..080 To be true to the nature of the sampled instrument, these patches

```text
   do not respond to velocity.
```

#### Effects and Percussion Patches

When selecting instruments 072..076 (Effects 1-2 and Percussion 1-3), a number of different sounds are mapped to each six keyboard keys/notes:

```text
  Note      Effects 1   Effects 2    Percussion 1   Percussion 2   Percussion 3
  30-35     Jet         Yes (ding)   -              -              Ratchet
  36-4l     Gunshot     No (buzz)    Kick Drum      Rim Shot       Snap 1
  42-47     RoboDeath   Applause     Snare          Exotic         Snap 2
  48-53     Whoosh      Dogbark      Toms           Congas         Dripdrum 1
  54-59     Punch       Door creak   Cymbal         Timbale        Dripdrum 2
  60-65     Slap        Door slam    Closed Hat     Cowbell        Wet clink
  66-71     Duck        Boom         Open Hat       Bongos         Talk Drum
  72-77     Ow! 1       Car skid     Ride           Whistle        Agogo
  78-83     Ow! 2       Goose        Shaker         Clave          Explosion
```

Note: The piano keys are numbered 36..84 (so notes 30..35 can be used only through MIDI messages, not via keyboard).
