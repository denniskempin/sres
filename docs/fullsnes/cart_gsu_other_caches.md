# SNES Cart GSU-n Other Caches

#### ROM-Read-Data Cache (1-byte read-ahead)

The cache is used for GETB/GETBS/GETBL/GETBH/GETC opcodes (which do read from [ROMBR:R14]). Loading the cache is invoked by any opcodes that do change R14 (such like ADD,MOVE,etc.), allowing following GETxx opcodes to be executed without Waitstates.

In some situations WAITs can occur: When the cache-load hasn't yet completed (ie. GETxx executed shortly after changing R14), when an opcode is fetched from ROM (rather than from RAM or Code-Cache), when ROMBR is changed (caution: in this special case following GETxx will receive [OldROMBR:R14] rather than [NewROMBR:R14]).

Caution: Do not execute the CACHE opcode shortly (7 cycles in 21MHz mode, or 4 cycles in 10MHz mode) after changing R14 (when doing that, the read from R14 will fail somehow, and following GETxx will return garbage).

Unknown if SNES writes to R14 (via Port 301Ch) do also prefetch [R14] data?

#### RAM-Write-Data Cache (1-byte/1-word write queue)

This cache is used for STB/STW/SM/SMS/SBK opcodes. After any such store opcodes, the written byte/word is memorized in the cache, and further opcodes can be fetched (from ROM or from Code-Cache) immediately without Waitstates, simultaneously with the cached value being forwarded to RAM.

In some situations WAITs can occur: When cache already contained data (ie. when executing two store opcodes shortly after each other), when an opcode is fetched from RAM (rather than from ROM or Code-Cache), when the RAMBR register is changed (this works as expected, it finishes the write to [OldRAMBR:nnnn]).

Results on doing Data-RAM-Reads while the Data-RAM-write is still busy are unknown (possibly, this will WAIT, too) (or it may return garbage)?

WAITs should also occur when the pixel-cache gets emptied?

RAM-Address-Cache (1 word) (Bulk Processing for read-modify-write) This very simple cache memorizes the most recently used RAM address (from LM/LMS opcodes, and probably also from LDB/LDW/STB/STW/SM/SMS opcodes; though some games insert STW to push data on stack, as if they were intended not to change the memorized address?), the SBK opcode can be used to write a word to the memorized address (ie. one can avoid repeating immediate operands in SM/SMS opcodes).
