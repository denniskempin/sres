# SNES Cart Satellaview Items

Predefined Items (and their 24bit memory pointers) Items sold in C-Skyscraper:

00 88C229h Transfer Device (allows to teleport to any building) (unlimited)  01 88C2B8h Telephone Card (5) (allows to enter phone booth) ;\  02 88C347h Telephone Card (4) (allows to enter phone booth) ; decreases  03 88C3D6h Telephone Card (3) (allows to enter phone booth) ; after usage  04 88C465h Telephone Card (2) (allows to enter phone booth) ;  05 88C4F4h Telephone Card (1) (allows to enter phone booth) ;/  06 88C583h Fishing Pole (allows to get Money from Dead Dentist, Person 2Eh)  07 88C612h Express Train Ticket                  ;\these are treated special  08 88C6A1h Museum Train Ticket                   ;/by code at 88936Ch  09 88C630h Bus Ticket (at Fountain)    ;\these all have same description  0A 88C7BFh Taxi Ticket                 ;  0B 88C84Eh Ferrari Blowjob Ticket      ;/ Items sold by Dr.Hiroshi (spring-boot guy near News Center)  0C 88C8DDh Doping Item (walk/run faster when pushing B Button)  0D 88C96Ch Unknown (disappears after usage) Items sold in Beach Shop:

0E 88C9FBh Whale Food (can be used at Oceans Shore)  0F 88CA8Ah Dolphin Food (can be used at Oceans Shore)  10 88CB19h Fish Food (can be used at Oceans Shore) Items sold in Sewerage:

11 88CBA8h Boy/Girl Gender Changer (can be used only once)  12 88CC37h Transform Boy/Girl into Purple Helmet guy (Person 08h)(temporarily)  13 88CCC6h Transform Boy/Girl into Brunette chick    (Person 1Dh)(temporarily)  14 88CD55h Smaller Neighbor's Home Door Key (allows to enter that building) Items obtained when picking-up Frogs:

15 88CDE4h Change Identity (edit user name) (from Frog 32h) (works only once)  16 88CE73h Change GUI Border Scheme         (from Frog 33h) (works only once)  17 88CF02h Change GUI Color Scheme          (from Frog 34h) (works only once)  18 88CF91h Change GUI Cursor Shape          (from Frog 35h) (works only once)

#### Item Format

As shown above, 25 items are defined in ROM at 99C229h-88D020h with 8Fh bytes per item. Custom Items (defined in Directory packet's "File" entries) can be stored at 10506Ah. The item format is:

```text
  00h 15h Item Name (max 20 chars, plus ending 00h) (First 2 bytes 00h = Free)
  15h 1   Length of following (Description, Pointer, Whatever) (always 79h?)
  16h 25h Item Description (max 36 chars, plus ending 00h)
  3Bh 47h Item Activation Message (max 70 chars, plus ending 00h)
   If Activation Message = empty (single 00h byte), then Item Function follows:
   3Ch 3   Pointer to Interpreter Tokens (eg. 99974Dh for Transfer Device)
   3Fh 43h Unknown/Unused/Padding (should be zero)
   (there is no SRAM allocated for custom item functions,
   so this part may be used only for predefined ROM items)
  82h 12  Item Price (12-Digit ASCII String, eg. "000000001200" for 1200G)
  8Eh 1   Item Drop/Keep Flag (00h=Drop after Activation, 01h=Keep Item)
```

In case of Custom Items, above ITEM[00h..8Eh] is copied from FILE[02h..90h] (ie. a fragment of "File" Entries in the Directory Packet).

Entry [15h] seems to be always 79h, giving a total length of 8Fh per item.

The Item Message is used for items that cannot be activated (eg. "You can't use telephone card outside of the phone booth.",00h). If the message is empty (00h), then the next 24bit are a pointer to the item handler (eg. the Teleport function for the Transfer Device).

Note: Items can be listed, activated, and dropped via Y-Button. The teleport device can be also activated via X-button.

#### Shops

There are four pre-defined shops: Dr.Hiroshi's appears when Person 01h exists, WITHOUT folder assigned, or WITH an item-folder. The Beach Shop, C-Skyscraper and Sewerage Shops appear if they HAVE an folder assigned, the folder must be flagged as Item/Shop. In all cases, the folder may contain additional items which are added to the Shop's predefined item list. Custom Shops can be created by assigning Item-Folders to other People/Buildings (in that case, the Folder MUST contain at least one item, otherwise the BIOS shows garbage). Shops may contain max 0Ah items (due to 7E865Eh array size).
