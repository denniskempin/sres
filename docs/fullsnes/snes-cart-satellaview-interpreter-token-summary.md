# SNES Cart Satellaview Interpreter Token Summary

#### Interpreter Tokens

```text
  00h  ControlSubThread(pEntrypoint)  ;special actions upon xx0000h..xx0005h
  01h  SetXYsignViewDirectionToSignsOfIncomingValues(vX,vY) ;not if both zero
  02h  SleepWithFixedObjShape(wSleep,pObjShape)
  03h  SleepWithXYstepAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9)
  04h  SleepWithXYsignAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9)
  05h  ClearForcedBlankAndFadeIn(wSleep,wSpeedRange?)
  06h  MasterBrightnessFadeOut(wSleep,wSpeedRange?) ;OptionalForcedBlank?
  07h  SetMosaicAndSleep(wSleep,wBgFlags,wMosaicSize)
  08h  N/A (hangs)
  09h  SleepAndBlendFromCurrentToNewPalette(wSleep,vPalIndex,pNewPalette)
  0Ah  HdmaEffectsOnBg3(wSleep,wEffectType,vScrollOffset,vExtraOffset)
  0Bh  SleepWithAngleAs9wayObjShape(wSleep,pObjShape1,..,pObjShape9) ;[18A8+X]
  0Ch  DisableObjsOfAllThreads()
  0Dh  ReEnableObjsOfAllThreads()
  0Eh  SleepWithXYsignAs9wayPlayerGenderObjShape(wSleep,pObjShape1,..,Shape9)
  0Fh  N/A (hangs)
  10h  SleepAndSetXYpos(wSleep,vX,vY)
  11h  SleepAndMoveTowardsTargetXYpos(wSleep,vX,vY)
  12h  SleepAndMoveByIncomingXYstep(wSleep,vX,vY)
  13h  SleepAndMoveAndAdjustXYstep(wSleep,vRotationAngleToOldXYstepOrSo?)
  14h  SleepAndMoveWithinBoundary(wSleep,vX1,vX2,vY1,vY2,wFactor?)
  15h  SleepAndMoveChangeBothXYstepsIfCollideOtherThread(wSleep,wBounceSpeed?)
  16h  SleepAndMoveAndIncrementXYstep(wSleep,vXincr,vYincr,qXlimit,qYlimit)
  17h  SleepAndMoveByIncomingYstepAndWavingXstep(wSleep,wY)
  18h  SleepAndMoveAndAccelerateTowardsTarget(wSleep,vX,vY,vSpeed)
  19h  SleepAndMoveAndSomethingComplicated?(wSleep,vX,vY)  ;out: X,Y=modified
  1Ah  AdjustXYstep(wNewSpeedOrSo?) ;in: [18A8+X]=angle
  1Bh  MoveByOldXYstepWithoutSleep()
  1Ch  SleepAndMoveChangeXYstepIfCollideOtherThread(wSleep,vMask,vX?,vY?)
  1Dh  N/A (hangs)
  1Dh  N/A (hangs)
  1Fh  N/A (hangs)
  20h  Goto(pTarget)
  21h  Gosub(pTarget)   ;max nesting=8 (or less when also using Loops)
  22h  Return()         ;return from Gosub
  23h  QuitThread()     ;terminate thread completely
  24h  LoopStart(wRepeatCount)  ;see token 62h (LoopNext)
  25h  Sleep(wSleep)
  26h  MathsLet(vA,vB)       ;A=B
  27h  MathsAdd(vA,vB)       ;A=A+B      ;1998 if unsigned carry
  28h  MathsSub(vA,vB)       ;A=A-B      ;1998 if signed overflow
  29h  MathsAnd(vA,vB)       ;A=A AND B  ;1998 if nonzero
  2Ah  MathsOr(vA,vB)        ;A=A OR B   ;1998 if nonzero
  2Bh  MathsXor(vA,vB)       ;A=A XOR B  ;1998 if nonzero
  2Ch  MathsNot(vA)          ;A=NOT A    ;1998 if nonzero
  2Dh  MathsMulSigned(vA,vB) ;A=A*B/100h ;1998 never (tries to be overflow)
  2Eh  MathsDivSigned(vA,vB) ;A=A/B*100h ;1998 if division by 0
  2Fh  SignedCompareWithConditionalGoto(vA,wOperator,vB,pTarget)
  30h  GotoIf_1998_IsNonzero(pTarget)
  31h  GotoIf_1998_IsZero(pTarget)
  32h  GotoArray(vArrayIndex,pPointerToArrayWithTargets)
  33h  ReadJoypad(bJoypadNumber,wX,wY)
  34h  CreateAnotherInterpreterThreadWithLimit(vThreadCount,bLimit,pEntry)
  35h  CheckIfXYposCollidesWithFlaggedThreads(vFlagMask) ;out: 1998=ID
  36h  GetUnsignedRandomValue(vA,wB) ;A=Random MOD B, special on B>7FFFh
  37h  SetObjWidthDepthFlagmask(vWidth,vDepth,vMask) ;for collide checks
  38h  CreateAnotherInterpreterThreadWithIncomingXYpos(vX,vY,pEntrypoint)
  39h  N/A (hangs)
  3Ah  SoundApuMessage00h_nnh(vParameter8bit)
  3Bh  SoundApuMessage01h_nnnh(vLower6bit,bMiddle2bit,bUpper2bit)
  3Ch  SoundApuMessage02h_nnnnh(vLower6bit,bMiddle2bit,bUpper2bit)
  3Dh  SoundApuUpload(bMode,pPtrToPtrToData)
  3Eh  SetPpuBgModeKillAllOtherThreadsAndResetVariousStuff(bBgMode)
  3Fh  SetTemporaryTableForBanksF1hAndUp(vTableNumber,pTableBase)
  40h  KillAllFlaggedThreads(vMask)  ;ignores flags, and kills ALL when Mask=0
  41h  SetBUGGEDTimerHotspot(wHotspot) ;BUG: accidently ORed with AE09h
  42h  Ppu_Bg1_Bg2_SetScrollPosition(vX,vY)
  43h  Ppu_Bg1_Bg2_ApplyScrollOffsetAndSleep(wSleep,vX,vY)
  44h  NopWithDummyParameters(wUnused,wUnused)
  45h  NopWithoutParameters()
  46h  AllocateAndInitObjTilesOrUseExistingTiles(wLen,pSrc)
  47h  AllocateAndInitObjPaletteOrUseExistingPalette(pSrc)
  48h  DmaObjTilesToVram(wObjVramAddr,wOBjVramEnd,pSrc)
  49h  SetObjPalette(wObjPalIndex,wObjPalEnd,pSrc)
  4Ah  SramAddSubOrSetMoney(bAction,vLower16bit,vMiddle16bit,vUpper16bit)
  4Bh  SramUpdateChksumAndBackupCopy()
  4Ch  N/A (hangs)
  4Dh  N/A (hangs)
  4Eh  N/A (hangs)
  4Fh  N/A (hangs)
  50h  TestAndGotoIfNonzero(vA,vB,pTarget)  ;Goto if (A AND B)<>0
  51h  TestAndGotoIfZero(vA,vB,pTarget)     ;Goto if (A AND B)==0
  52h  InitNineGeneralPurposePrivateVariables(wA,wB,wC,wD,wE,wF,wG,wH,wI)
  53h  MultipleCreateThreadBySelectedTableEntries(vFlags,vLimit,pPtrToTable)
  54h  PrepareMultipleGosub()  ;required prior to token 6Ah
  55h  StrangeXYposMultiplyThenDivide(wA,wB) ;Pos=Pos*((B-A)/2)/((B-A)/2)
  56h  BuggedForceXYposIntoScreenArea() ;messes up xpos and/or hangs endless
  57h  Maths32bitAdd16bitMul100h(vA(Msw),vB) ;A(Msw:Lsw)=A(Msw:Lsw)+B*100h
  58h  Maths32bitSub16bitMul100h(vA(Msw),vB) ;A(Msw:Lsw)=A(Msw:Lsw)-B*100h
  59h  SoundApuUploadWithTimeout(wTimeout,pPtrToPtrToData)
  5Ah  N/A (hangs)
  5Bh  N/A (hangs)
  5Ch  N/A (hangs)
  5Dh  N/A (hangs)
  5Eh  N/A (hangs)
  5Fh  N/A (hangs)
  60h  CallMachineCodeFunction(pTarget)
  61h  SetTemporaryOffsetFor0AxxxxhVariables(vOffset)
  62h  LoopNext()  ;see token 24h (LoopStart)
  63h  SetForcedBlankAndSleepOnce()
  64h  ClearForcedBlankAndSleepOnce()
  65h  AllocateAndInitObjPaletteAndObjTilesOrUseExistingOnes(pSrc) ;fragile
  66h  WriteBgTiles(wBgNumber,pPtrTo16bitLenAnd24bitSrcPtr)
  67h  WritePalette(pPtrTo16bitLenAnd24bitSrcPtr)  ;to backdrop/color0 and up
  68h  WriteBgMap(wBgNumber,pPtrTo16bitLenAnd24bitSrcPtr)
  69h  KillAllOtherThreads()
  6Ah  MultipleGosubToSelectedTableEntries(vFlags,pPtrToTable) ;see token 54h
  6Bh  AllocateAndInitBgPaletteTilesAndMap2(vX1,vY1,pPtrToThreePtrs,vBgMapSize)
  6Ch  DeallocateAllObjTilesAndObjPalettes()
  6Dh  BuggedSetBgParameters(bBgNumber,pPtr,wXsiz,wYsiz,wUnused,wUnused)
  6Eh  BuggedSetUnusedParameters(bSomeNumber,pPtr,wX,wY)
  6Fh  BuggedChangeBgScrolling(wX,wY)
  70h  PauseAllOtherThreads()
  71h  UnPauseAllOtherThreads()
  72h  GosubIfAccessedByPlayer(pGosubTargetOrPeopleFolderID)
  73h  Dma16kbyteObjTilesToTempBufferAt7F4000h()   ;Backup OBJ Tiles
  74h  Dma16kbyteObjTilesFromTempBufferAt7F4000h() ;Restore OBJ Tiles
  75h  SetFixedPlayerGenderObjShape(pSrc,wLen1,wLen2)
  76h  InstallPeopleIfSatelliteIsOnline() ;create all people-threads
  77h  KillAllOtherThreadsAndGotoCrash()  ;Goto to FFh-filled ROM at 829B5Eh
  78h  ZerofillBgBufferInWram(vBgNumber)
  79h  ChangePtrToObjPriority(vVariableToBePointedTo)  ;default is <Ypos>
  7Ah  ChangeObjVsBgPriority(vPriorityBits)     ;should be (0..3 * 1000h)
  7Bh  SetXYposRelativeToParentThread(vX,vY)
  7Ch  TransferObjTilesAndObjPaletteToVram(pPtrToPtrsToPaletteAndTileInfo)
  7Dh  AllocateAndInitBgPaletteTilesAndMap1(vX1,vY1,pPtrToThreePtrs,vBgMapSize)
  7Eh  DrawMessageBoxAllAtOnce(vWindowNumber,vDelay,vX,vY,pPtrToString)
  7Fh  DrawMessageBoxCharByCharBUGGED(..)  ;works only via CALL, not token 7Fh
  80h..FFh  Reserved/Crashes (jumps to garbage function addresses)
```

#### Legend for Token Parameters

```text
  v   16bit Global or Private Variable or Immediate (encoded as 3 token bytes)
  p   24bit Pointer (3 token bytes) (banks F0h..FFh translated, in most cases)
  b   8bit  Immediate (encoded directly as 1 token byte)
  w   16bit Immediate (encoded directly as 2 token bytes)
  q   16bit Immediate (accidently encoded as 3 token bytes, last byte unused)
```

#### 3-byte Variable Encoding (v)

```text
  +/-00nnnnh  -->  +/-nnnnh          R     ;immediate
  +/-01nnnnh  -->  +/-[nnnnh+X]      R/W   ;private variable (X=thread_id*2)
  +/-02nnnnh  -->  +/-[nnnnh]        R/W   ;global variable
  +  03nnnnh  -->  +[nnnnh+[19A4h]]  W     ;special (write-only permission)
  +  09nnnnh  -->  +[nnnnh+[19A4h]]  R/W   ;special (read/write permission)
  +  0Annnnh  -->  +[nnnnh+[19A4h]]  R     ;special (read-only permission)
  Examples: 000001h or FF0001h (aka -00FFFFh) are both meaning "+0001h".
  021111h means "+[1111h]", FDEEEF (aka -021111h) means "-[1111h]".
```

#### 3-byte Pointer Encoding (p)

```text
  00nnnnh..EFnnnnh     --> 00nnnnh..EFnnnnh      (unchanged)
  F0nnnnh              --> TokenProgramPtr+nnnn  (relative)
  F1nnnnh (or F2nnnnh) --> [[AFh+0]+nnnn*3]      (indexed by immediate)
  F3nnnnh              --> [[AFh+0]+[nnnn+X]*3]  (indexed by thread-variable)
  F4nnnnh              --> [[AFh+0]+[nnnn]*3]    (indexed by global-variable)
  F5nnnnh (or F6nnnnh) --> [[AFh+3]+nnnn*3]      (indexed by immediate)
  F7nnnnh              --> [[AFh+3]+[nnnn+X]*3]  (indexed by thread-variable)
  F8nnnnh              --> [[AFh+3]+[nnnn]*3]    (indexed by global-variable)
  F9nnnnh (or FAnnnnh) --> [[AFh+6]+nnnn*3]      (indexed by immediate)
  FBnnnnh              --> [[AFh+6]+[nnnn+X]*3]  (indexed by thread-variable)
  FCnnnnh              --> [[AFh+6]+[nnnn]*3]    (indexed by global-variable)
  FDnnnnh..FFnnnnh     --> crashes               (undefined/reserved)
```

#### 2-byte Operators for Signed Compare (Token 2Fh)

```text
  0000h Goto_if_less              ;A<B
  0001h Goto_if_less_or_equal     ;A<=B
  0002h Goto_if_equal             ;A=B
  0003h Goto_if_not_equal         ;A<>B
  0004h Goto_if_greater           ;A>B
  0005h Goto_if_greater_or_equal  ;A>=B
```

#### ControlSubThread(pEntrypoint) values

```text
  xx0000h Pause
  xx0001h UnpauseSubThreadAndReenableObj
  xx0002h PauseAfterNextFrame
  xx0003h PauseAndDisableObj
  xx0004h ResetAndRestartSubThread
  xx0005h KillSubThread
  NNNNNNh Entrypoint (with automatic reset; only if other than old entrypoint)
```

Maximum Stack Nesting is 4 Levels (Stack is used by Gosub and Loop tokens).

Token Extensions (some predefined functions with Token-style parameters) These are invoked via CALL Token (60h), call address, followed by params.

```text
  809225h CallKillAllMachineCodeThreads()
  80B47Dh CallGetTextLayerVramBase()
  80B91Eh CallClearBg3TextLayer()
  818EF9h CallSetApuRelatedPtr()
  818F06h CallDrawMessageBoxCharByChar(vWindowNumber,vDelay,vX,vY,pPtrToString)
  818FF0h CallDrawBlackCircleInLowerRightOfWindow()
  81903Dh CallDisplayButton_A_ObjInLowerRightOfWindow()
  81A508h CallSetGuiBorderScheme(pAddr1,pAddr2)
  81A551h CallSetTextWindowBoundaries(wWindowNumber,bXpos,bYpos,bXsiz,bYsiz)
  81A56Eh CallHideTextWindow(wWindowNumber)
  81A57Bh CallSelectWindowBorder(wWindowNumber,wBorder) ;0..3, or FFh=NoBorder
  81A59Ah CallSelectTextColor(wWindowNumber,bColor,bTileBank,bPalette)
  81A5C3h CallClearTextWindowDrawBorder(wWindowNumber)
  81A5D2h CallZoomInTextWindow(wWindowNumber,wZoomType)  ;\1,2,3=Zoom HV,V,H
  81A603h CallZoomOutTextWindow(wWindowNumber,wZoomType) ;/0=None/BuggyWinDiv2
  81A634h CallSetGuiColorScheme(pAddr)
  81A65Dh CallChangePaletteOfTextRow(vX,vY,vWidth,vPalette)
  81A693h CallPeekMemory16bit(vDest,pSource)
  81A6B4h CallPokeMemory16bit(vSource,pDest)
  81C7D0h CallInitializeAndDeallocateAllObjTilesAndObjPalettes()
  81C871h CallDeallocateAllObjs()
  81CDF9h CallBackupObjPalette()
  81CE09h CallRestoreObjPalette()
  829699h CallUploadPaletteVram(pSource,wVramAddr,bPaletteIndex)
  88932Fh CallTestIfFolderExists()  ;in: 0780, out: 1998,077C,077E
  88D076h CallTestIfDoor()
  99D9A4h CallSelectPlayerAsSecondaryThread  ;[19A4]=PlayerThreadId*2
```

Note: Some of these Call addresses are also listed in a 24bit-pointer table at address 808000h (though the (BIOS-)code uses direct 8xxxxxh values instead of indirect [808xxxh] values).

#### Token Functions (some predefined token-functions)

These can be invoked with GOSUB token (or GOTO or used as thread entrypoint):

```text
  99D69A EnterTown (use via goto, or use as entrypoint)
  828230 DeallocMostBgPalettesAndBgTiles ;except tile 000h and color 00h-1Fh
  88C1C6 SetCursorShape0
  88C1D0 SetCursorShape1
  88C1E0 SetCursorShape2
  88C1EA SetCursorShape3
  88C1F4 SetCursorShape4
  88C1FE SetCursorShape5
  99D8AB PauseSubThreadIfXYstepIsZero
  99D8CD MoveWithinX1andX2boundaries
  99D903 MoveWithinY1andY2boundaries
```

Note: Some of the above functions are also listed in the table at 808000h.

#### Compressed Data

Some of the Functions can (optionally) use compressed Tile/Map/Palette data.

> **See:** [SNES Decompression Formats](snes-decompression-formats.md)

Note: The actual compressed data is usually preceeded-by or bundled-with compression flags, length entry, and/or (in-)direct src/dest pointers (that "header" varies from function to function).
