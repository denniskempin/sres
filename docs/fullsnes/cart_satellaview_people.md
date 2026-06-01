# SNES Cart Satellaview People

#### People

People are showing up only if they are flagged in the 64bit People Present Flags in the Town Status packet. If more than 5 people are flagged, then only the first 5 are shown (regardless of that limit, the 4 frogs and the ship can additionally be there).

```text
  00h Red Ball (on beach) (disappears after access)
  01h Spring-boot (aka Dr.Hiroshi's Shop) (near news center) (sells items)
  02h General Pee (showing up here and there pissing against buildings)
  03h Brown Barbarian on Cocaine (near temple)
  04h Blue Depressive Barbarian (near temple)
  05h Ghost Waver (near phone booth)
  06h Boy in Neighborhood
  07h Older Elvis (near churches)
  08h Purple Helmet (on beach)
  09h Surfer (near beach shop)
  0Ah Grayhaired (northwest lawn)
  0Bh Alien Man (near phone booth)
  0Ch Uncle Bicycle (near parabol antenna)
  0Dh Circus Man (near temple/lake)
  0Eh Speedy Blind Man (near parabol antenna/spintop)
  0Fh Blonde Boy (near factory)
  10h Girl with Pink Dress (near Bigger Neighbor's Home)
  11h Brunetty Guy (near Bigger Neighbor's Home)
  12h Brunette (near junkfood)
  13h Darkhaired (near junkfood)
  14h Blue Longhair (near junkfood)
  15h Brunette Longhair (near junkfood)
  16h Brunette Longhair (near red-heart church)
  17h Green Longhair (near red-heart church)
  18h Bicycle Girl (near C-Skyscraper)
  19h Brunette Office Woman (near C-Skyscraper)
  1Ah Blue Longhair (near parabol antenna)
  1Bh Turquoise Longhair (near home)
  1Ch Blue Longhair (near maths/spintop)
  1Dh Brunette Longhair (near news center/beach stairs)
  1Eh Black Longhair (near police)
  1Fh Red Longhair (southeast beach)
  20h Blackhaired Girl (near police)
  21h Greenhaired Girl (on bench between temple and lake)
  22h Graybluehaired older Woman (east of C-skyscraper)
  23h Darkhaired Housewife (near home)
  24h Traditional Woman (west of Robot-Skyscraper)
  25h Greenhaired Girl (near Turtle-Arena)
  26h Pinkhaired Girl (near Cow-Skull Church)
  27h Brown Dog (northeast lawn)
  28h White Dog (near home)
  29h Gray Duck (near temple/lake)
  2Ah Portable TV-Headed Guy (near Robot-Skyscraper)
  2Bh Satellite Wide-screen TV-Headed Guy (near Robot-Skyscraper)
  2Ch Custom Person 2Ch  ;\may be enabled only if defined in Expansion Area
  2Dh Custom Person 2Dh  ;/of Directory Packet (otherwise crashes)
```

Below 2Eh-37h are specials which cannot have a Folder assigned to them:

```text
  2Eh Dead Dentist (on bench near lake) (gives Money when owning Fishing Pole)
  2Fh Gimmick: Allows to use Bus/Taxi/Ferrari Tickets at Fountain
  30h Gimmick: Allows to use Express/Museum-Train-Tickets at Railways Station
  31h Gimmick: Special Event when accessing the Hydrant
  32h Frog 32h (west of Robot-Skyscraper)       ;Change Identity Item
  33h Frog 33h (west of Robot-Skyscraper, too)  ;Change GUI Border Scheme
               (or on street near turtle arena?)
  34h Frog 34h (northwest lawn)                 ;Change GUI Color Scheme
  35h Frog 35h (near Cow-Skull Church)          ;Change GUI Cursor Shape
  36h Gimmick: Allows to use Whale/Dolphin/Fish Food at Oceans Shore
  37h Ship (cannot be accessed?) (near factory)
  38h Mr.Money (near police) (donates a 500G coin)  ;\only one can be present
  39h Mr.Money (near police) (donates a 1000G coin) ; at once, after the coin,
  3Ah Mr.Money (near police) (donates a 5000G coin) ;/all do act as Folder 38h
  3Bh-3Fh Unused?
```

Like Buildings, People can have a Folder associated to them (using above values as Folder ID, at least, that works for People 00h..2Bh).

If there is no corresponding folder transmitted, then People aren't doing anything useful (aside from producing whatever japanese sentences). One exception: If there's no People-File-Folder with ID=01h, then the Spring-boot guy acts as "Dr Hiroshi's Shop" where one can buy question-marked items for 3000G. The Frogs can be picked up (adding an Item to the inventory). Frog positions seem to be random (west of Robot-Skyscraper, street near Turtle Arena, northwest lawn, or near Cow-Skull Church).

#### Avatars

These are assigned for folders (either for people in buildings, or people on the streets).

```text
  00h Geisha (faecher)
  01h Snorty (nose bubble)
  02h Gold hat
  03h Naked guy
  04h Soldier (lanze)
  05h Whore (lipstick/eye blinking)
  06h Wise man1 (huge white eyebrows)
  07h Wise man2 (huge stirn, huge ears)
  08h DJ Proppy (headphones, sunglasses, muscles)
  09h Casino manager (fat guy with red slip-knot)
  0Ah Student girl (manga, karierte bluse)
  0Bh School girl (satchel ranzen/smiley)
  0Ch Kinky gay (green hair, sunglasses, gold-ohrring)
  0Dh Yankee (glistening teeth/dauerwelle)
  0Eh Robot (blech-mann)
  0Fh Blonde chick (blonde, lipstick)
  10h BS-X logo (not a person, just the "BS-X" letters)
  11h-FFh Unknown/Unused/Crashes
  30h None (seems to be an un-intended effect, probably unstable, don't use)
```
