---
title: "Tricky-to-emulate games"
source_url: "https://snes.nesdev.org/wiki/Tricky-to-emulate_games"
pageid: 8
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This is a list of various emulation bugs encountered during the development of Mesen-S.
It is meant as a quick reference to help others find and debug issues with their own emulation core.
It can also be used to find games that might be affected by a code change, for regression testing purposes.

| Cause | Affected games |
| --- | --- |
| Missing/incorrect open bus implementation (CPU or PPU) | - **Captain America and the Avengers**: Broken graphics during gameplay - **Combatribes, The**: Doesn't boot - **Home Alone**: Crashes after main menu - **Rock n' Roll Racing**: Broken graphics in character select - **Super 3D Noah's Ark**: Broken mode 7 graphics |
| Incorrect or missing BRK/COP implementation | - **Actraiser**: Crashes after menu - **Bishoujo Senshi Sailor Moon - Another Story**: Crashes after menu - **Cybernator**: Intro restarts when starting game - **Dekitate High School**: Doesn't boot - **Illusion of Gaia**: Doesn't boot - **Kamaitachi no Yoru**: Doesn't boot - **Soul Blazer**: Doesn't boot |
| Incorrect ORA [d] implementation | - **Super Mario World**: Corrupted overworld map (bad 3bpp decompression) |
| VRAM writes are not ignored during rendering | - **Hook**: Text glitches during intro |
| Ignored VRAM writes during rendering are not incrementing the VRAM address | - **Kick Off** |
| VRAM read implementation is incorrect | - **Breath of Fire** |
| Offset-per-tile wraparound logic is incorrect | - **Super Famista 5**: Issues at the top of the screen during animation inside the main menu |
| Offset-per-tile implementation has bugs | - **Axelay**: Broken level background in some areas of stage 2 - **Chrono Trigger**: Broken layout in in-game menu |
| Mode 7 doesn't implement window logic | - **Atlas, The - Renaissance Voyager**: Broken mode 7 graphics during intro - **MechWarrior**: Broken mode 7 graphics during gameplay |
| Mode 7 scroll offsets are not latched at the beginning of the scanline | - **NHL '94**: Glitch scanline in the middle of screen during intro animation |
| Mode 7 direct color implementation is wrong | - **Aerobiz**: Wrong colors during intro sequence |
| Color window is not applied to all pixels | - **Krusty's Super Fun House**: Broken graphics at the edges of the screen |
| Color math for subscreen in high resolution modes is incorrect | - **Jurassic Park**: Broken graphics during gameplay |
| Implementation of OAM writes during rendering is inaccurate | - **Uniracers**: In 2-player mode, second player's unicycle doesn't display correctly |
| OAM fetching/rendering timing inaccurate | - **Mega lo Mania - Jikuu Daisenryaku**: A single black line appears in the middle of the screen during the intro |
| DMA controller power on state is invalid | - **Heian Fuuunden**: Corrupt title screen when pressing start to skip opening animation |
| CPU doesn't run an extra cycle before starting DMA after write to $420B | - **Mighty Morphin Power Rangers - The Fighting Edition** |
| DMA controller allows reading B-bus registers using A-bus address | - **Krusty's Super Fun House**: Wrong colors during gameplay |
| DMA/HDMA timings are inaccurate | - **Circuit USA**: Broken graphics in menu - **Jumbo Ozaki no Hole in One**: Broken graphics in menu |
| DMA is not suspended when HDMA runs | - **Dekitate High School** |
| HDMA doesn't ignore "fixed transfer" flag | - **Batman Forever**: Broken mode 7 graphics during intro - **Lost Vikings, The**: Black screen when gameplay starts |
| HDMA doesn't ignore "decrement" flag | - **Adventures of Kid Kleets, The**: Broken graphics at power on - **MechWarrior**: Broken mode 7 graphics during gameplay |
| HDMA ignores "direction" flag | - **Pocky & Rocky**: Broken license/logo screens on power on |
| HDMA "do tranfer" flag isn't set/reset properly | - **Aladdin**: Broken graphics in the background on first level - **Super Ghouls'n Ghosts**: Black screen once gameplay starts |
| V-IRQ doesn't trigger when V-IRQs are enabled for the current scanline | - **RoboCop versus The Terminator** |
| NMI isn't triggered when NMI is enabled partway through vertical blank | - **Alien vs Predator**: Screen flashes during gameplay - **Pocky & Rocky**: Screen flashes during gameplay |
| When NMI is triggered as per above, CPU jumps to interrupt vector immediately (should run 1 more instruction) | - **Jaki Crush**: Doesn't boot (may depend on other timing) |
| SPC timings are inaccurate (requires SPC to run 1 cycle at a time, rather than 1 instruction at a time) | - **ActRaiser 2**: Freezes - **Hiouden - Mamono-tachi to no Chikai**: Broken sound - **Illusion of Gaia**: Doesn't boot - **Tales of Phantasia**: Missing sound effects, freezes |
| CPU read effects do not occur early enough in the CPU's cycle | - **Rendering Ranger R2**: Freezes due to infinite loop with SPC |
| DSP KOF register is not initialized to $00 | - **Chester Cheetah - Too Cool to Fool**: Missing sound effects - **King of Dragons**: Missing sound effects |
| SRAM mappings are incorrect | - **Fire Emblem - Thracia 776** - **Ys III - Wanderers from Ys** |
| RAM power on state is "incorrect" | - **Bishoujo Senshi Sailor Moon - Another Story**: Random static in music during intro (when SPC RAM is randomized) - **Death Brade**: Duel ends instantly when RAM is initialized with 0s - **Power Drive**: Broken graphics when RAM is initialized with 0s - **Super Keiba 2**: Blackscreen after a few menus when SRAM is initialized with 0s |
| Super FX - RPIX implementation is incorrect | - **Yoshi's Island**: Broken effects in tunnels[[1]](https://github.com/SourMesen/Mesen-S/issues/25) |
| OAMADD priority sprite rotation is missing | - **Super Mario World**: The lives-indicator Mario is hidden at the top left of the map screen |
| HDMA can't affect open bus | - **Speedy Gonzales: Los Gatos Bandidos**: Game locks up in level 6-1[[2]](https://mgba.io/2017/07/31/holy-grail-bugs-2/#speedy-gonzales-los-gatos-bandidos) |

## See Also

- [[Uncommon graphics mode games]]
- [[Emulator tests]]
