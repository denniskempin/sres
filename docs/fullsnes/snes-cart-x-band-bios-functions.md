# SNES Cart X-Band BIOS Functions

#### X-Band BIOS Functions (CALL E00040h)

Invoked via CALL E00040h, with X=function_number (0001h..054xh on SNES/US), with parameters pushed on stack, and with return value in A register (16bit) or X:A register pair (32bit), and with zeroflag matched to the A return value.

The function table isn't initialized by the compiler/linker, instead, the BIOS boot code is starting the separate components (such like "controls.c"), which are then installing their function set via calls to "SetDispatchedFunction".

The Sega function numbers are based on the string list in file "SegaServer\Server\Server_OSNumbers.h" (which is part of the SERVER sources, but it does hopefully contain up to date info on the retail BIOS functions).

```text
  Sega SNES SNES Function
  Gen. US   JP
```

#### Sourceless - Misc

```text
  000h           RestoreSegaOS
  001h           AskForReplay
  002h           ThankYouScreen            ;thankyou shown at next coldboot?
  003h           InstallDispatchedManager
  004h           CallManagerControl
  005h           SoftInitOS
  006h           GetDispatchedFunction
  007h 007h 007h SetDispatchedFunction     ;change/install BIOS function vector
  008h           SetDispatchedGroup
  009h           GetManagerGlobals
  00Ah           SetManagerGlobals
  00Bh           AllocateGlobalSpace
  00Ch           FreeGlobalSpace
  00Dh           DisposePatch
  00Eh           CompactOSCodeHeap
  00Fh           GetPatchVersion
  010h           SetPatchVersion
```

#### Sourceless - Memory

```text
  011h           InitHeap
  012h           NewMemory
  013h           NewMemoryHigh
  014h           NewMemoryClear
  015h           DisposeMemory
  016h 01Ah 01Ah GetMemorySize          ;get size of an item
  017h           MaxFreeMemory
  018h           TotalFreeMemory
  019h           SwitchPermHeap
  01Ah           SwtichTempHeap  ;uh, Swtich?
  01Bh           CreateTempHeap
  01Ch           CreateHeapFromPtr
  01Dh           CreateTempSubHeap
  01Eh           AllocPermHeapZone
  01Fh           DisposePermHeapZone
  020h           CompactHeap
  021h           MoveHeap
  022h           PrepareHeapForMove
  023h           ComputeHeapPtrDelta
  024h           ResizeHeap
  025h           BlockMove
  026h           WhichMemory
  027h           GetHeapSize
  028h           VerifySegaHeap
  029h           PurgePermHeaps
  02Ah           ByteCopy
  02Bh           UnpackBytes
  02Ch           FillMemory
  02Dh           GetCurrentHeap
  02Eh           FindLastAllocatedBlock
  02Fh           SetOSUnstable
  030h           SetDBUnstable
  031h           SetAddressUnstable
  032h           InstallReliableAddress
  033h           CheckOSReliable
```

#### GameLib\controls.c - Keyboard/Joypad Controls

```text
  034h ?         InitControllers
  035h 033h      ReadHardwareController      ;get joypad data
  036h 034h      ControllerVBL               ;do joypad and keyboard scanning
  037h ?         ReadAllControllers
  038h 036h      FlushHardwareKeyboardBuffer ;flush char_queue
  039h 037h      GetNextHardwareKeyboardChar ;read char_queue
  03Ah 038h      GetHardwareKeyboardFlags
  03Bh 039h      SetHardwareKeyboardFlags
  03Ch 03Ah      GetNextESKeyboardRawcode ;read scancode_queue  ;ES=Eric Smith
  03Dh ?         GetNextESKeyboardStatus
  03Eh 03Ch      GetNextESKeyboardChar    ;read scancode_queue, xlat to char
  03Fh ?         SendCmdToESKeyboard
  -    03Eh 03Fh keyb_io_read_scancodes
  -    03Fh      keyb_blah_do_nothing
  -    040h      keyb_io_read_verify_id_code
  -    041h 043h keyb_forward_scancode_queue_to_char_queue
```

#### Sourceless - Misc

```text
  040h           GetGlobal
  041h           SetGlobal
```

#### Database\PatchDB.c - Game/Patch (SNES: installed at D6:4F93 ?)

```text
  042h 042h 044h AddGamePatch
  043h           LoadGamePatch
  044h           DisposeGamePatch
  045h           GetGamePatchVersion
  046h           GetGamePatchFlags
  047h 04Ah 04Eh FindGamePatch
  048h 054h 058h CreateGameDispatcher
  049h           InitGamePatch
  04Ah           StartGame
  04Bh           GameOver
  04Ch           ResumeGame
  04Dh           GameDoDialog
  04Eh           UpdateGameResultsAfterError
  04Fh           HandleGameError
  050h           PlayCurrentGame
  051h 053h 057h InstallGameFunction
  052h 055h 059h DisposeOldestGamePatch
  053h           MarkGamePatchUsed
```

#### Sourceless - Messages

```text
  054h           InitMessages
  055h           ProcessServerData
  056h           ProcessPeerData
  057h           SendMessage
  058h           GetSendMessageHandler
  059h           GetPeerMessageHandler
  05Ah           GetSerialOpCode
  05Bh           GetServerMessageHandler
  05Ch           InstallPeerHandler
  05Dh           InstallReceiveServerHandler
  05Eh           InstallSendMessageHandler
  05Fh           ReceivePeerMessageDispatch
  060h           ReceiveServerMessageDispatch
  061h           GobbleMessage
  062h           SetClearLoginMisc
  063h           GetLoginMisc
```

#### Graphics\Sprites.c

```text
  064h           CreateSprite
  065h           CreateSpriteInFront
  066h           CreateSpriteHigh
  067h           DisposeSprite
  068h           MoveSprite
  069h           DrawSprite
  06Ah           IncrementSpriteFrame
  06Bh           SetSpriteFrame
  06Ch           GetSpriteFrame
  06Dh           FlipSprite
  06Eh           CreateSpriteData
  06Fh           CreateTextSprite
  070h           CreateTextSpriteFromBitmap
  071h           ExplodeSprite
  072h           SetSpriteGrayFlag
  073h           SetSpriteTilePosition
  074h           SetSpriteImage
  075h           SetSpritePalette
  076h           WriteSpriteToVDP
  077h           FigureTileSize
  078h           AllocateSprite
  079h           FreeSprite
  07Ah           GetSpriteLastTile
  07Bh           GetSpriteFirstTile
  07Ch           NewSpark
  07Dh           DisposeSpark
  07Eh           GetSparkSprite
  07Fh           StartSpark
  080h           StopSpark
  081h           DrawXBandLogo
  082h           DisposeXBandLogoRef
  083h           DisposeXBandLogoSparks
  084h           SyncOTron
```

#### Graphics\Decompress.c

```text
  085h           InitDecompression
  086h           CreateDecompressor
  087h           DisposeDecompressor
  088h           SetDstPattern
  089h           SetImageTiling
  08Ah           SetImageOrigin
  08Bh           GetImageClut
  08Ch           DisposeImagePatterns
  08Dh           DecompressFrame
  08Eh           SetDecompressorOptionsSelector
  08Fh           SetDecompressorPixelMappingSelector
  090h           SetDecompressorPaletteSelector
  091h           GetDictionaryCache
  092h           ReleaseDictionaryCache
  093h           SetDecompressorImage
  094h           ExpandPatternDictionary
  095h           GetDecompressorCache
  096h           ReleaseDecompressorCache
  097h           JoshDecompress
```

Sourceless - Time...

```text
  098h           AddTimeRequest
  099h           RemoveTimeRequest
  09Ah           TimeIdle
  09Bh           IncCurrentTime
  09Ch           DelayMS
  09Dh           DelayTicks
  09Eh           SetOSIdle
  09Fh           SegaOSIdle
  0A0h           GetJesusTime
  0A1h           SetJesusTime
  0A2h           GetJesusDate
  0A3h           SetJesusDate
```

#### Graphics\animation.c - Animations

```text
  0A4h           InitAnimateProcs
  0A5h           SpawnAnimation
  0A6h           SpawnDBAnimation
  0A7h           CreateAnimation
  0A8h           DisposeAnimation
  0A9h           DrawAnimationFrame
  0AAh           StartAnimation
  0ABh           StopAnimation
  0ACh           SuspendAnimations
  0ADh           SetAnimationPriority
  0AEh           SetAnimationGrayFlag
  0AFh           GetAnimationSuspendLevel
```

#### Graphics\paths.c - Paths (and maybe also LinePath.c?)

```text
  0B0h           InitPathManager
  0B1h           CreatePath
  0B2h           DisposePath
  0B3h           SetPathPoints
  0B4h           SetPathFrames
  0B5h           SetPathVelocity
  0B6h           GetPathPoint
  0B7h           DistBetweenPoints
```

#### Graphics\Pattern.c

```text
  0B8h           InitPatternManager
  0B9h           NewPatternBlock
  0BAh           NewPatternBlockHigh
  0BBh           FreePatternBlock
  0BCh           DeallocateTopPatternBlock
  0BDh           NewFirstPatternBlock
  0BEh           SetRange
  0BFh           ClearRange
  0C0h           RangeIsFree
  0C1h           FindFreeRange
  0C2h           GetLeftOnesTable
  0C3h           GetRightOnesTable
```

#### Graphics\Cursor.c

```text
  0C4h           CreateSegaCursor
  0C5h           DisposeSegaCursor
  0C6h           MoveSegaCursor
  0C7h           HideSegaCursor
  0C8h           ShowSegaCursor
  0C9h           GetSegaCursorPos
  0CAh           SetSegaCursorImage
  0CBh           LoadCursorFromVRAM
  0CCh           DrawSegaCursor
  0CDh           LoadCursorPattern
```

#### Graphics\SegaText.c (1)

```text
  0CEh           InitSegaFonts
  0CFh           SetCurFont
  0D0h           GetCurFont
  0D1h           GetCurFontHeight
  0D2h           GetCurFontLineHeight
  0D3h           SetFontColors
  0D4h           GetFontColors
  0D5h           SetupTextGDevice
  0D6h           GetTextPatternAddress
  0D7h           GetTextGDeviceOrigin
  0D8h           DrawSegaString
  0D9h           RenderSegaString
  0DAh           MeasureSegaText
  0DBh           CenterSegaText
  0DCh           DrawClippedSegaText
  0DDh           DrawCenteredClippedSegaText
  0DEh           DrawPaddedClippedSegaText
  0DFh           GetCharWidth
  0E0h           SegaNumToString
  0E1h           SegaNumToDate
  0E2h           SegaAppendText
  0E3h           CompareDates
  0E4h           CompareStrings
  0E5h           SetupTextSpriteGDevice
  0E6h           EraseTextGDevice
  0E7h           GetStringLength
```

#### Graphics\SegaText.c (2) and Database\StringDB.c

```text
  0E8h           DrawDBXYString              ;Database\StringDB.c
  0E9h           GetDBXYString               ;Database\StringDB.c
  0EAh           GetSegaString               ;Database\StringDB.c
  0EBh           GetWriteableString          ;Database\StringDB.c
  0ECh           SetWriteableString          ;Database\StringDB.c
  0EDh           DeleteWriteableString       ;Database\StringDB.c
  0EEh           GetUniqueWriteableStringID  ;Database\StringDB.c
  -              AddDBXYString               ;Database\StringDB.c (simulator)
```

#### Graphics\SegaText.c (3)

```text
  0EFh           CopyCString
  0F0h           SetTextPatternStart
  0F1h           EqualCStrings
  0F2h           GetTextStateReference
  0F3h           SaveTextState
  0F4h           RestoreTextState
  0F5h           DisposeTextStateReference
  0F6h           VDPCopyBlitDirect
  0F7h           VDPCopyBlitDirectBGColor
  0F8h           VDPCopyBlitTiled
  0F9h           VDPCopyBlitTiledBGColor
  0FAh           OrBlit2to4
  0FBh           OrBlit1to4
```

#### Sourceless - Modem? (parts related to GameLib\CommManager.c?)

```text
  0FCh           PInit
  0FDh           POpen
  0FEh           PListen
  0FFh           POpenAsync
  100h           PListenAsync
  101h           PClose
  102h           PNetIdle
  103h           PCheckError
  104h           PWritePacketSync
  105h           PWritePacketASync
  106h           PGetError
  107h           PUOpenPort
  108h           PUClosePort
  109h           PUProcessIdle
  10Ah           PUProcessSTIdle
  10Bh           PUReadSerialByte
  10Ch           PUWriteSerialByte
  10Dh           PUTransmitBufferFree
  10Eh           PUReceiveBufferAvail
  10Fh           PUTestForConnection
  110h           PUReadTimeCallback
  111h           PUWriteTimeCallback
  112h           PUSetupServerTalk
  113h           PUTearDownServerTalk
  114h           PUSetError
  115h           PUIsNumberBusy
  116h           PUOriginateAsync
  117h           PUAnstondet
  118h           PUWaitForRLSD
  119h           PUInitCallProgress
  11Ah           PUCallProgress
  11Bh           PUDialNumber
  11Ch           PUWaitDialTone
  11Dh           PUAnswerAsync
  11Eh           PUCheckAnswer
  11Fh           PUCheckRing
  120h           PUResetModem
  121h           PUSetTimerTicks
  122h           PUSetTimerSecs
  123h           PUTimerExpired
  124h           PUHangUp
  125h           PUPickUp
  126h           PUWriteXRAM
  127h           PUWriteYRAM
  128h 13Dh      PUReadXRAM
  129h 13Eh      PUReadYRAM
  12Ah           PUIdleMode
  12Bh           PUDataMode
  12Ch           PUDialMode
  12Dh           PUToneMode
  12Eh           PUCheckLine
  12Fh           PUCheckCarrier
  130h           PUDetectLineNoise
  131h           PUListenToLine
  132h           PUDisableCallWaiting
  133h           PUAsyncReadDispatch
  134h           PUDoSelectorLogin
  135h           PUMatchString
  136h           PGetDebugChatScript
```

Sourceless - Transport?

```text
  137h           TInit
  138h           TOpen
  139h           TListen
  13Ah           TOpenAsync
  13Bh           TListenAsync
  13Ch           TClose
  13Dh           TCloseAsync
  13Eh           TUnthread
  13Fh           TNetIdle
  140h           TUCheckTimers
  141h           TReadDataSync
  142h           TReadDataASync
  143h           TWriteDataSync
  144h           TWriteDataASync
  145h           TAsyncWriteFifoData
  146h           TReadData
  147h           TWriteData
  148h           TReadAByte
  149h           TWriteAByte
  14Ah           TQueueAByte
  14Bh           TReadBytesReady
  14Ch           TDataReady
  14Dh           TDataReadySess
  14Eh           TIndication
  14Fh           TForwardReset
  150h           TNetError
  151h           TCheckError
  152h           TUInitSessRec
  153h           TUSendCtl
  154h           TUDoSendCtl
  155h           TUDoSendOpenCtl
  156h           TUUpdateSessionInfo
  157h           TUSendOpen
  158h           TUSendOpenAck
  159h           TUSendCloseAdv
  15Ah           TUSendFwdReset
  15Bh           TUSendFwdResetAck
  15Ch           TUSendFwdResetPacket
  15Dh           TUSendRetransAdv
  15Eh           TUOpenDialogPacket
  15Fh           TUFwdResetPacket
  160h           TUCloseConnPacket
  161h           TURetransAdvPacket
  162h           TUAllowConnection
  163h           TUDenyConnection
  164h           TUSetError
  165h           TUGetError
  166h           TGetUserRef
  167h           TSetUserRef
  168h           TGetTransportHold
  169h           TGetTransportHoldSession
  16Ah           TSetTransportHold
  16Bh           TSetTransportHoldSession
```

#### Database\DB.c - Database

```text
  16Ch           InitPermDatabase
  16Dh           CompactPermDatabase
  16Eh 185h      DBGetItem
  16Fh           DBAddItem
  170h 188h 19Bh DBDeleteItem
  171h 189h 19Ch DBGetUniqueID
  172h           DBGetUniqueIDInRange
  173h           DBGetItemSize
  174h           DBCountItems
  175h 18Dh      DBGetFirstItemID
  176h 18Eh      DBGetNextItemID
  177h           DBNewItemType
  178h           DBGetTypeFlags
  179h           DBSetTypeFlags
  17Ah           DBDeleteItemType
  17Bh           DBPurge
  17Ch           DBTypeChanged
  17Dh           ComputeTypeCheckSum
  17Eh           DBVerifyDatabase
  17Fh           DBROMSwitch
  180h           DBAddItemPtrSize
  181h 199h 1ACh DBAddItemHighPtrSize
  182h 19Ah 1ADh DBPreflight    ;check if enough free mem for new item
  183h           GetItemSize
  184h           DBGetTypeNode
  185h           DBGetPrevTypeNode
  186h           DBTNGetItem
  187h           DBTNGetPrevItem
  188h           DBTNDisposeList
  189h           DeleteItem
  18Ah           AddItemToDB
  18Bh           AllowDBItemPurge
```

#### Graphics\SegaScrn.c - Video/Screen

```text
  18Ch           LinearizeScreenArea
  18Dh           GetSegaScreenBaseAddr
  18Eh           InitSegaGDevices
  18Fh           SetCurrentDevice
  190h           GetCurrentDevice
  191h           RequestClut
  192h           ReleaseClut
  193h           IncrementClutReferences
  194h           SetupClutDB
  195h           GetSegaScreenOrigin
  196h           GetSegaGDevice
  197h           EraseGDevice
  198h           SetupVDP
  199h           BlankClut
  19Ah           FadeInClut
  19Bh           FadeInScreen
  19Ch           GenerateGrayMap
  19Dh           WaitVBlank
  19Eh           SetBackgroundColor
  19Fh           GetBackgroundColor
  1A0h           RequestUniqueClut
  1A1h           RequestSpecificClut
  1A2h           SetupClut
  1A3h           GetClut
  1A4h           GetColorLuminance
  1A5h           FillNameTable
```

Sourceless - VRAM...

```text
  1A6h           DMAToVRAM
  1A7h           CopyToVRAM
  1A8h           CopyToCRAM
  1A9h           CopyToVSRAM
  1AAh           CopyToVMap
  1ABh           FillVRAM
  1ACh           FillCRAM
  1ADh           FillVSRAM
```

#### Database\Opponent.c - Opponent

```text
  1AEh           GetOpponentPhoneNumber
  1AFh           SetOpponentPhoneNumber
  1B0h           GetCurOpponentIdentification
  1B1h           SetCurOpponentIdentification
  1B2h           GetCurOpponentTaunt
  1B3h           GetCurOpponentInfo
  1B4h           ClearOldOpponent
  1B5h           GetOpponentVerificationTag
  1B6h           SetOpponentVerificationTag
```

#### Database\UsrConfg.c - User/Password

```text
  1B7h           GetCurrentLocalUser
  1B8h           FillInUserIdentification
  1B9h           GetLocalUserTaunt
  1BAh           SetLocalUserTaunt
  1BBh           GetLocalUserInfo
  1BCh           SetLocalUserInfo
  1BDh           IsUserValidated
  1BEh           SetCurUserID
  1BFh           GetCurUserID
  1C0h           VerifyPlayerPassword
  1C1h           IsEmptyPassword
  1C2h           ComparePassword
  1C3h           GetPlayerPassword
```

#### UserInterface\DitlMgr.c - DITL (also related to Database\DITLItemSetup.c?)

```text
  1C4h           NewDITL
  1C5h           GiveDITLTime
  1C6h           DisposeDITL
  1C7h           GetDITLItem
  1C8h           InitDITLMgr
  1C9h           ClearDITLDone
  1CAh           ProcessDITLScreen
  1CBh           SetupDITLItemList
  1CCh           SetupDITLObjectData
  1CDh           DisposeDITLItemList
  1CEh           SetupControlTable
  1CFh           DisposeControlTable
  1D0h           GetDITLObjectData
```

#### UserInterface\Events.c

```text
  1D1h           InitUserEvents
  1D2h           FlushUserEvents
  1D3h           WaitForUserButtonPress
  1D4h           CheckUserButtonPress
  1D5h           GetNextControllerEvent
  1D6h           GetNextCommand
  1D7h           QueueGet
  1D8h           QueueInsert
```

#### Sourceless - Sound

```text
  1D9h           SetBGMDisable
  1DAh           GetBGMDisable
  1DBh           InitSoundMgr
  1DCh           ShutDownSoundMgr
  1DDh           StartDBBGM
  1DEh           StopBGM
  1DFh           PlayDBFX
  1E0h           FX1NoteOff
  1E1h           FX2NoteOff
  1E2h           ShutUpFXVoice1
  1E3h           ShutUpFXVoice2
```

#### Sourceless - Misc

```text
  1E4h           GetDataSync
  1E5h           GetDataBytesReady
  1E6h           GetDataError
```

#### Database\Challnge.c - Challenge

```text
  1E7h           GetChallengePhoneNumber
  1E8h           SetChallengePhoneNumber
  1E9h           GetChallengeIdentification
  1EAh           SetChallengeIdentification
```

#### Database\GameID.c - Game ID

```text
  1EBh 210h 224h GetGameID     ;out:A=SnesCartStandardChksum, X=SnesHeaderCCITT
  -    211h 225h   ... related to GameID ?
```

#### Sourceless - Misc

```text
  1ECh           IsRemoteModemTryingToConnect
  1EDh           SetRemoteModemTryingToConnectState
  1EEh           InitScreen
  1EFh           PreflightScreen
  1F0h           SetupScreen
  1F1h           SendCommandToScreen
  1F2h           KillScreen
  1F3h           GetNewScreenIdentifier
  1F4h           GetCurScreenIdentifier
  1F5h           GetScreenStateTable
  1F6h           ResetCurrentScreen
  1F7h           GetScreenLayoutRectangleCount
  1F8h           GetScreenLayoutRect
  1F9h           GetScreenLayoutCharRect
  1FAh           GetScreenLayoutPointCount
  1FBh           GetScreenLayoutPoint
  1FCh           GetScreenLayoutStringCount
  1FDh           GetScreenLayoutString
  1FEh           DrawScreenLayoutString
  1FFh           BoxScreenLayoutString
  200h           GetScreensEnteredCount
```

#### Graphics\Backdrops.c

```text
  201h           SetBackdropID
  202h           SetBackdropBitmap
  203h           ClearBackdrop
  204h           HideBackdrop
  205h           SetAuxBackgroundGraphic
  206h           ShowBackdrop
  207h           GetBlinkySprite
```

#### Database\BoxSer.c (1)

```text
  208h           GetBoxSerialNumber
  209h           SetBoxSerialNumber
  20Ah           GetHiddenBoxSerialNumbers
  20Bh           GetBoxHometown
  20Ch           SetBoxHometown
  20Dh           SetBoxState
  20Eh           ResetBoxState
  20Fh           GetBoxState
  210h           SetLastBoxState
  211h           ResetLastBoxState
  212h           GetLastBoxState
  213h           GetGameWinsLosses
  214h           SetCompetitionResults
  215h           GetCompetitionResults
  216h           SetGameErrorResults
  217h           GetGameErrorResults
  218h           UpdateGameResults
  219h           ClearGameResults
  21Ah           ClearNetErrors
  21Bh           GetLocalGameValue
  21Ch           SetLocalGameValue
  21Dh           GetOppGameValue
  21Eh           SetOppGameValue
  21Fh           IsBoxMaster
  220h           SetBoxMaster
  221h 24Bh 25Fh SetCurGameID            ;SNES/US: [3631,3633]
  222h 24Ch      GetCurGameID
  223h           CheckBoxIDGlobals
  224h           InitBoxIDGlobals
  225h           ChangedBoxIDGlobals
  226h           DBAddConstant
  227h           DBGetConstant
  228h           DBSetConstants
  229h           SetDialNetworkAgainFlag
  22Ah           CheckDialNetworkAgainFlag
  22Bh           SetBoxXBandCard
  22Ch           GetBoxXBandCard
  22Dh           GetBoxLastCard
  22Eh           SetBoxMagicToken
  22Fh           SetBoxProblemToken
  230h           GetBoxProblemToken
  231h           UseBoxProblemToken
  232h           SetBoxValidationToken
  233h           GetBoxValidationToken
  234h           SetIMovedOption
  235h           SetQwertyKeyboardOption
  236h           SetCallWaitingOption
  237h           SetAcceptChallengesOption
  238h           GetAcceptChallengesOption
  239h           GetIMovedOption
  23Ah           GetQwertyKeyboardOption
  23Bh           GetCallWaitingOption
  23Ch           GetNetErrors
```

Database\BoxSer.c (2), and also Database\PhoneNumbers.c ?

```text
  23Dh           GetBoxPhoneNumber
  23Eh           SetBoxPhoneNumber
  23Fh           GetLocalAccessPhoneNumber
  240h           SetLocalAccessPhoneNumber
  241h           Get800PhoneNumber
```

#### Database\BoxSer.c (3)

```text
  242h           GetLocalUserName
  243h           SetLocalUserName
  244h           GetLocalUserROMIconID
  245h           SetLocalUserROMIconID
  246h           GetLocalUserCustomROMClutID
  247h           SetLocalUserCustomROMClutID
  248h           GetLocalUserPassword
  249h           SetLocalUserPassword
  24Ah           ValidateUserPersonification
  24Bh           InvalidateUserPersonification
```

#### Database\PlayerDB.c

```text
  24Ch           GetAddressBookTypeForCurrentUser
  24Dh           GetAddressBookIDFromIndex
  24Eh           CountAddressBookEntries
  24Fh           RemoveAddressBookEntry
  250h           GetIndexAddressBookEntry
  251h           AddAddressBookEntry
  252h           GetUserAddressBookIndex
  253h           DeleteAddressBookEntry
  254h           SendNewAddressesToServer
  255h           MarkAddressBookUnchanged
  256h           AddressBookHasChanged
  257h           CorrelateAddressBookEntry
  -              PreflightNewAddressEntry
```

#### UserInterface\NewAddressMgr.c

```text
  258h           AddPlayerToAddressBook
  259h           UpdateAddressBookStuff
  25Ah           AddOnDeckAddressBookEntry
  25Bh           MinimizeUserHandle
```

#### Database\GraphicsDB.c

```text
  25Ch           GetDBGraphics
  25Dh           DrawDBGraphic
  25Eh           DrawDBGraphicAt
  25Fh           DrawGraphic
  260h           DisposeGraphicReference
  261h           GetGraphicReferenceClut
  262h           DrawPlayerIcon
  263h           NukePlayerRAMIcon
  264h           GetPlayerRAMIconBitMap
  265h           GetPlayerIconBitMap
  266h           GetIconBitMap
  267h           PlayerRAMIconExists
  268h           DisposeIconReference
  269h           GetDBButtonFrame
  26Ah           DrawGraphicGray
  26Bh           HueShift
```

#### Graphics\TextUtls.c - Text Edit

```text
  26Ch           FindLineBreak
  26Dh           SegaBoxText
  26Eh           DrawSegaStringLength
  26Fh           MeasureSegaTextLength
  270h           InitTextEdit
  271h           SetTextEditLineHeight
  272h           TextEditAppend
  273h           TextEditDelete
  274h           DisposeTextEdit
  275h           TextEditActivate
  276h           TextEditDeactivate
  277h           TextEditPreflightAppend
  278h           TextEditGetLineLength
  279h           DrawTextBox
  27Ah           SetJizzleBehavior
  27Bh           GetJizzleBehavior
  27Ch           StartTextBoxAnimation
  27Dh           StopTextBoxAnimation
  27Eh           DisposeTextBoxReference
  27Fh           DrawSegaTextPlusSpaces
  280h           UpdateTECaret
  281h           EraseTextEditLine
  282h           GetCompressedJizzlers
```

#### Database\News.c (and NewsUtils.c) - News

```text
  283h           FindNextNewsString
  284h           AddPageToNewsBox
  285h           GetPageFromNewsBox
  286h           GetNewsForm
  287h           GetNumNewsPages
  288h           EmptyNewsBox
  289h           DrawNewsPage
  28Ah           ValidateNews
  28Bh           InvalidateNews
  28Ch           SetupNewsForServerConnect
  28Dh           ServerConnectNewsDone
  28Eh           DoNewsControlIdle
  28Fh           KillCurNewsPage
  290h           GetNewsGraphicsID
  291h           ShowLeftRightPageControls
  292h           DrawNewsReturnIcon
  293h           SetNewsCountdownTimeConst
  294h           DrawXBandNews
  295h           DisposeXBandNews
```

#### Database\GameDB.c - Network Game Database (NGP)

```text
  296h           GetNGPListGamePatchInfo
  297h           GetNGPListGamePatchVersion
  298h           GetNGPVersion
  299h           UpdateNGPList
  29Ah           UpdateNameList
  29Bh           GetGameName
```

#### Database\Personification.c

```text
  29Ch           ChangeUserPersonificationPart
  29Dh           InstallOpponentPersonification
  29Eh           GetPersonificationPart
  29Fh           PutPersonificationOnWire
  2A0h           GetPersonificationFromWire
  2A1h           DisposePersonificationSetup
  2A2h           ReceivePersonficationBundle
  2A3h           ParsePersonificationBundle
  2A4h           CreatePersonificationBundle
```

#### Database\Mail.c - MailCntl

```text
  2A5h           CountInBoxEntries
  2A6h           CountOutBoxEntries
  2A7h           AddMailToOutBox
  2A8h           AddMailToInBox
  2A9h           RemoveMailFromInBox
  2AAh           GetIndexInBoxMail
  2ABh           GetIndexOutBoxMail
  2ACh           GetInBoxGraphicID
  2ADh           MarkMailItemRead
  2AEh           DeleteAllOutBoxMail
  2AFh           GetInBoxTypeForCurrentUser
  2B0h           GetOutBoxTypeForCurrentUser
  2B1h           GetOutBoxIDFromIndex
  2B2h           GetInBoxIDFromIndex
  2B3h           GetBoxIDFromIndex
```

Database\SendQ.c - Send Queue or so?

```text
  2B4h           AddItemToSendQ
  2B5h           AddItemSizeToSendQ
  2B6h           DeleteSendQ
  2B7h           KillSendQItem
  2B8h           GetFirstSendQElementID
  2B9h           GetNextSendQElementID
  2BAh           CountSendQElements
  2BBh           GetSendQElement
  2BCh           RemoveItemFromSendQ
```

#### UserInterface\DialogMgr.c

```text
  2BDh           SetDialogColors
  2BEh           DoDialog
  2BFh           DialogParameterText
  2C0h           DoDialogItem
  2C1h           DoDialogParam
  2C2h           DoPlayAgainDialog
  2C3h           CopyString
  2C4h           DoAnyResponse
  2C5h           DoDataDrivenDismissal
  2C6h           DoPassword
  2C7h           DrawDialogFrame
  2C8h           FillTextRectangle
  2C9h           HorizontalLine
  2CAh           KillProgressTimer
  2CBh           ReplaceParameters
  2CCh           SetupProgressTimer
  2CDh           VerticalLine
  2CEh           CreateShiners
  2CFh           DisposeShiners
```

#### Sourceless - Fred Chip Hardware

```text
  2D0h           SetVector
  2D1h           SetVectorTblAddr
  2D2h           SetSafeRamSrc
  2D3h           SetSafeRomSrc
  2D4h 323h 33Dh SetLEDs
  2D5h           SetLEDScreenAnimation
```

#### Sourceless - Joggler

```text
  2D6h           InitJoggler
  2D7h           DisplayJoggler
  2D8h           StopJoggler
```

#### Database\DeferredDialogMgr.c

```text
  2D9h           QDefDialog
  2DAh           ShowDefDialogs
  2DBh           CountDefDialogs
  2DCh           DisableDefDialogs
  2DDh           EnableDefDialogs
```

#### Sourceless - Misc

```text
  2DEh           CheckNetRegister
  2DFh           NetRegister
  2E0h           NetRegisterDone
  2E1h           SetNetTimeoutValue
  2E2h           GetNetTimeoutValue
  2E3h           GetNetWaitSoFar
  2E4h           NetRegisterTimeOutTimeProc
  2E5h           IsBoxNetRegistered
  2E6h           GetNetRegisterCase
```

#### Database\Capture.c - Session Capture (not actually implemented?)

```text
  -              BeginSession
  -              DeleteSession
  -              EndSession
  -              BeginStreamCapture
  -              AddDataToStream
```

#### Database\Playback.c - Session Playback (not actually implemented?)

```text
  -              BeginSessionPlayback
  -              SessionExists
  -              PlaybackNextStream
  -              PlaybackCurrentStream
  -              PlaybackPreviousStream
  -              DoesNextSessionStreamExist
  -              DoesPreviousSessionStreamExist
```

#### GameLib\Synch.c - Synch (not actually implemented?)

```text
  -              SynchModems
  -              SynchVbls
```

Sourceless - Game Talk Session?

```text
  2E7h           GTSInit
  2E8h           GTSShutdown
  2E9h           GTSFlushInput
  2EAh           GTSessionPrefillFifo
  2EBh           GTSessionEstablishSynch
  2ECh           GTSessionExchangeCommands
  2EDh           GTSessionValidateControl
  2EEh           GTSErrorRecover
  2EFh           GTSCloseSessionSynch
  2F0h           GTSDoCommand
  2F1h           GTSDoResend
  2F2h           GTSResendFromFrame
  2F3h           GTSSetPacketFormat
  2F4h           GTSSetRamRomOffset
  2F5h           GTSessionSetLatency
  2F6h           GTSessionSendController8
  2F7h           GTSessionReadController8
  2F8h           GTSessionSendController12
  2F9h           GTSessionReadController12
  2FAh           GTSessionSendController16
  2FBh           GTSessionReadController16
  2FCh           GTSessionSendController18
  2FDh           GTSessionReadController18
  2FEh           GTSessionSendController24
  2FFh           GTSessionReadController24
  300h           GTSessionSendController27
  301h           GTSessionReadController27
```

Sourceless - Game Talk Modem?

```text
  302h           GTModemInit
  303h           GTModemGetModemError
  304h           GTModemClearFifo
  305h           GTModemClockInByte
  306h           GTModemClockOutByte
  307h           GTModemAbleToSend
  308h           GTModemSendBytes
  309h           GTModemCheckLine
  30Ah           GTModemReadModem
  30Bh           GTSendReceiveBytes
  30Ch           GTCloseSessionSafe
  30Dh           GTCreateLooseSession
  30Eh           GTLooseSessionIdle
  30Fh           GTCloseLooseSession
  310h           GTSyncotron
  311h           GTMasterCalculateLatency
  312h           GTSlaveCalculateLatency
  313h           GTSyncoReadModemVBL
  314h           GTSyncronizeVBLs
  315h           GTSyncronizeMasterLeave
  316h           GTSyncronizeSlaveLeave
  317h           GTSyncoTronVBLHandler
  318h           GTUnused1
  319h           GTUnused2
  31Ah           GTUnused3
  31Bh           GTUnused4
  31Ch           GTUnused5
  31Dh           GTUnused6
```

#### UserInterface\Keyboard.c - Keyboard

```text
  31Eh           SetupKeyboardEntryLayout
  31Fh           DisposeKeyboardEntryLayout
  320h           DoKeyboardEntry
  321h           InitKeyboardEntry
  322h           SendCommandToKeyboard
  323h           FinishKeyboardEntry
  324h           RefreshKeyboard
  325h           StuffCurrentKeyboardField
  326h           SelectKeyboardField
  327h           SendCommandToChatKeyboard
  328h           GetKeyLayoutFieldCount
  329h           GetKeyLayoutFieldSize
  32Ah           SetKeyboardEntryMeasureProc
  32Bh           SetFocusField
  32Ch           DrawKeyboard
  32Dh           ComputeCursorLineNumber
  32Eh           CacheKeyboardGraphics
  32Fh           ReleaseKeyboardGraphicsCache
```

#### Sourceless - Smart Card

```text
  330h           GetCardType
  331h           CardInstalled
  332h 381h      ReadCardBytes       ;read smart card byte(s)
  333h           WriteCardBit
  334h           GotoCardAddress
  335h           IncrementCardAddress
  336h 385h      ReadCardBit         ;read smart card bit
  337h           ResetCard
  338h           PresentSecretCode
  339h           GetRemainingCredits
  33Ah           FindFirstOne
  33Bh           CountCardBits
  33Ch           DebitCardForConnect
  33Dh           DebitSmartCard
  33Eh           CheckValidDebitCard
  33Fh           IsGPM896
  340h           IsGPM103
  341h           IsGPM256
  342h           Debit896Card
  343h           Debit103Card
  344h           Get896Credits
  345h           Get103Credits
  346h           CheckWipeCard
  347h           UserWantsToDebitCard
```

#### Sourceless - Sort

```text
  348h           QSort
```

#### UserInterface\Secrets.c

```text
  349h           TrySecretCommand
  34Ah           TestThisSequence
  34Bh           ExecCommands
  34Ch           GetSecretList
  34Dh           GetSecretSequence
  34Eh           ResetSecretCommand
  34Fh           TestSequence
  350h           PlayMaze
  351h           EndPlayMaze
```

#### Sourceless - Maths

```text
  352h           LongDivide
  353h           LongMultiply
  354h           Sqrt
  355h           RandomShort
  356h           Sine
  357h           Cosine
```

#### Database\RankingMgr.c - Ranking

```text
  358h           GetFirstRanking
  359h           GetNextRanking
  35Ah           GetPrevRanking
  35Bh           GetHiddenStat
  35Ch           NextRankingExists
  35Dh           PrevRankingExists
  35Eh           CountRankings
  35Fh           GetFirstRankingID
  360h           GetNextRankingID
  361h           GetUniqueRankingID
  362h           GetRankingSize
  363h           DeleteRanking
  364h           AddRanking
  365h           GetRanking
```

#### Graphics\Progress.c - Progress Bar Manager

```text
  366h           InitProgressProcs
  367h           SpawnProgressProc
  368h           DisposeProgressProc
  369h           SetProgressPosition
  36Ah           ProgressIdle
```

#### UserInterface\RadioButtons.c

```text
  36Bh           SetupRadioButton
  36Ch           DrawRadioButton
  36Dh           ActivateRadioButton
  36Eh           DeactivateRadioButton
  36Fh           RadioButtonSelectNext
  370h           RadioButtonSelectPrevious
  371h           RadioButtonGetSelection
  372h           RadioButtonSetSelection
  373h           RadioButtonIdle
  374h           DisposeRadioButtonRef
  375h           DrawRadioSelection
```

#### Sourceless - Misc

```text
  376h           NetIdleFunc
  377h           CheckError
  378h 3D9h 3F8h ccitt_updcrc
```

#### UserInterface\PeerConnect.c

```text
  379h           DoPeerConnection
  37Ah           ConnectToPeer
  37Bh           DisplayPeerInfo
  37Ch           DoSlavePeerConnect
  37Dh           DoMasterPeerConnect
  37Eh           PeerConnectionDropped
  37Fh           DoPeerRestoreOS
  380h           DoExchangePeerData
  381h           DoPeerDialog
  382h           Chat
  383h           PeerStartVBL
  384h           PeerStopVBL
  385h           PeerVBLHandler
```

#### Sourceless - Fifo

```text
  386h           FifoInit
  387h           FifoActive
  388h           FifoWrite
  389h           FifoRead
  38Ah           FifoPeek
  38Bh           FifoPeekEnd
  38Ch           FifoAvailable
  38Dh           FifoRemaining
  38Eh           FifoSkip
  38Fh           FifoCopy
  390h           FifoChkSum
  391h           GetFifoIn
  392h           FifoLastCharIn
  393h           FifoUnwrite
  394h           FifoSize
  395h           FifoFlush
  396h           FifoUnread
  397h           FifoResetConsumption
  398h           FifoAdjustConsumption
```

#### Database\Results.c - Result FIFO (not implemented?)

```text
  -              AddToResultFIFO
  -              ReplaceTopEntryOfResultFIFO
  -              GetTopEntryOfResultFIFO
  -              GetIndexEntryInResultFIFO
  -              CountEntriesInResultFIFO
```

#### Database\FourWayMailView.c

```text
  -              FourWayMail stuff
```

#### Sourceless - Misc

```text
  399h           AddVBLRequest
  39Ah           RemoveVBLRequest
  39Bh           VBLIdle
  39Ch           PatchRangeStart                              <--- ??
  39Dh           PatchRangeEnd = kPatchRangeStart + 50        <--- ???
  -    54xh      SNES table end
```

#### X-Band GAME Functions (CALL E000CCh)

The GAME functions are just aliases for the normal BIOS functions. The idea seems to have been that the BIOS function numbering might change in later BIOS revisions, which would cause compatibility issues for older game patches. As a workaround, there's a separate GAME function table which contains copies of some important BIOS function vectors (and which is probably intendend to maintain fixed function numbers even in later BIOS revisions).

The GAME functions are invoked via CALL E000CCh, with X=function_number (0000h..004Dh on SNES/US).

The Game Function numbers for Sega are enumerated (among others) in "Database\GamePatch.h". The Game Function table is initialized by "CreateGameDispatcher" (which is using a lot of "InstallGameFunction" calls to transfer the separate function vectors from BIOS table to GAME table).

```text
  Sega SNES SNES Function
  Gen. US   JP
```

#### general game stuff

```text
  00h  00h?      kOSHandleGameError
  01h  01h?      kOSGameOver
```

#### basic os stuff

```text
  02h            kOSNewMemory
  03h            kOSDisposeMemory
  04h            kOSDelayTicks
```

#### hardware stuff

```text
  05h            kOSSetSafeRomSrc
  06h            kOSSetSafeRamSrc
  07h            kOSSetVectorTableAddr
  08h            kOSSetVector
  09h  12h       kOSSetLEDs
```

#### PModem

```text
  0Ah            kOSReadSerialByte
  0Bh            kOSWriteSerialByte
  0Ch            kOSReceiveBufferAvail
  0Dh            kOSTransmitBufferFree
  0Eh            kOSCheckLine
  0Fh            kOSDetectLineNoise
  10h            kOSCheckCarrier
  11h            kOSListenToLine
  12h            kOSSetTimerTicks
  13h            kOSTimerExpired
  14h            kOSToneMode
  15h  20h       kOSReadXRAM
  16h  21h       kOSReadYRAM
  17h            kOSWriteXRAM
  18h            kOSWriteYRAM
```

#### gametalk

```text
  19h            kOSGTSSetPacketFormat
  1Ah            kOSGTSSetRamRomOffset
  1Bh            kOSGTSessionSetLatency
  1Ch            kOSGTSessionPrefillFifo
  1Dh            kOSGTSessionEstablishSynch
  1Eh            kOSGTSErrorRecover
  1Fh            kOSGTSCloseSessionSynch
  10h            kOSGTSFlushInput
  11h            kOSGTSessionValidateControl
  12h            kOSGTSessionExchangeCommands
  13h            kOSGTSDoCommand
  14h            kOSGTSDoResend
  15h            kOSGTSResendFromFrame
  16h            kOSGTModemInit
  17h            kOSGTModemGetModemError
  18h            kOSGTModemClearFifo
  19h            kOSGTModemClockInByte
  1Ah            kOSGTModemClockOutByte
  1Bh            kOSGTModemAbleToSend
  1Ch            kOSGTModemSendBytes
  1Dh            kOSGTModemCheckLine
```

#### controller should probably be in "hardware stuff"

```text
  1Eh            kOSInitControllers
  1Fh            kOSReadControllers
```

#### stinkotron

```text
  20h            kOSGTSyncotron
  21h            kOSGTMasterCalculateLatency
  22h            kOSGTSlaveCalculateLatency
  23h            kOSGTSyncoReadModemVBL
  24h            kOSGTSyncronizeVBLs
  25h            kOSGTSyncronizeMasterLeave
  26h            kOSGTSyncronizeSlaveLeave
  27h            kOSGTSyncoTronVBLHandler
```

#### keep this one

```text
  28h  4Eh       kOSLastFunction
```
