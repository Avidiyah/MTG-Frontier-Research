# S8 counterexample search: P-ATQ-3 prefix rule and P-ATQ-4 spell-created delayed triggers

Units: 72533 (printed 71563); pre-change dump: yes, 72652 units

## P3.1 Prefix rule firing inventory: 3572 units (pool 224)
by source: Counter({'printed': 3572})
by role: Counter({'ability': 3325, 'mode': 241, 'granted': 6})
by kind (post): Counter({'triggered_ability': 1830, 'spell_or_static_text': 1036, 'activated_ability': 422, 'keyword_ability': 210, 'replacement_effect': 57, 'characteristic_defining_ability': 10, 'prevention_effect': 4, 'additional_cost': 2, 'cast_restriction': 1})
by decade: {'1990s': 24, '2000s': 251, '2010s': 606, '2020s': 2691}
by set_type: Counter({'expansion': 2007, 'commander': 568, 'funny': 465, 'draft_innovation': 220, 'alchemy': 84, 'token': 72, 'memorabilia': 46, 'core': 43, 'masters': 20, 'box': 19, 'eternal': 19, 'duel_deck': 6, 'vanguard': 2, 'masterpiece': 1})

## P3.2 Categories

### chapter (Saga face): 624 units, 17 distinct prefixes (pool 34)
kinds: Counter({'triggered_ability': 624})
   183  'III'
   172  'I'
   142  'II'
    47  'I, II'
    24  'II, III'
    22  'IV'
    15  'I, II, III'
     5  'III, IV'
     4  'I, II, III, IV'
     3  'II, III, IV'
     1  'I, II, III, IV, V, VI'  e.g. [ltr 2023 | Enchantment — Saga] Long List of the Ents #0:1 (triggered_ability/ability): I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga.
     1  'I, III'
     1  'II, IV'
     1  'II, III, IV, V, VI'  e.g. [who 2023 | Enchantment — Saga] City of Death #0:2 (triggered_ability/ability): II, III, IV, V, VI — Create a token that's a copy of target non-Saga token you control.
     1  'II, III, IV, V'  e.g. [who 2023 | Enchantment — Saga] The Flux #0:2 (triggered_ability/ability): II, III, IV, V — Exile the top card of your library. You may play that card this turn.
     1  'VI'  e.g. [who 2023 | Enchantment — Saga] The Flux #0:3 (triggered_ability/ability): VI — Add six {R}.
     1  'V'  e.g. [fin 2025 | Enchantment Creature — Saga Kn] Summon: Knights of Round #0:2 (triggered_ability/ability): V — Ultimate End — Other creatures you control get +2/+2 until end of turn. Put an indestructible counter on each of them.

### ability word (CR 207.2c): 1406 units, 61 distinct prefixes (pool 91)
kinds: Counter({'triggered_ability': 786, 'spell_or_static_text': 408, 'activated_ability': 163, 'replacement_effect': 39, 'characteristic_defining_ability': 10})
   198  'Landfall'
   103  'Threshold'
    74  'Delirium'
    65  'Domain'
    47  'Raid'
    45  'Heroic'
    40  'Channel'
    36  'Constellation'
    35  'Metalcraft'
    31  'Magecraft'
    29  'Imprint'
    29  'Morbid'
    28  'Converge'
    27  'Ferocious'
    27  'Alliance'
    25  'Enrage'
    24  'Hellbent'
    22  'Battalion'
    20  'Strive'
    20  'Coven'
    19  'Inspired'
    19  'Descend N'
    18  'Vivid'
    18  'Spell mastery'
    18  'Revolt'
    18  'Adamant'
    18  'Survival'
    17  'Eerie'
    15  'Void'
    14  'Formidable'
    14  'Rally'
    14  'Flurry'
    14  'Renew'
    14  'Repartee'
    13  'Bloodrush'
    13  'Lieutenant'
    13  'Undergrowth'
    13  'Valiant'
    13  'Infusion'
    12  'Kinship'

### named mode (role = mode): 239 units, 214 distinct prefixes (pool 19)
kinds: Counter({'spell_or_static_text': 200, 'triggered_ability': 29, 'keyword_ability': 5, 'activated_ability': 3, 'prevention_effect': 1, 'replacement_effect': 1})
     5  'Khans'
     5  'Dragons'
     5  'N'
     2  'Antimagic Cone'  e.g. [afr 2021 | Creature — Beholder] Baleful Beholder #0:1 (spell_or_static_text/mode): Antimagic Cone — Each opponent sacrifices an enchantment of their choice.
     2  'Fear Ray'  e.g. [afr 2021 | Creature — Beholder] Baleful Beholder #0:2 (spell_or_static_text/mode): Fear Ray — Creatures you control gain menace until end of turn.
     2  'Cure Wounds'  e.g. [afr 2021 | Creature — Human Cleric] Dawnbringer Cleric #0:1 (spell_or_static_text/mode): Cure Wounds — You gain 2 life.
     2  'Dispel Magic'  e.g. [afr 2021 | Creature — Human Cleric] Dawnbringer Cleric #0:2 (spell_or_static_text/mode): Dispel Magic — Destroy target enchantment.
     2  'Gentle Repose'  e.g. [afr 2021 | Creature — Human Cleric] Dawnbringer Cleric #0:3 (spell_or_static_text/mode): Gentle Repose — Exile target card from a graveyard.
     2  'Fight the Current'  e.g. [afr 2021 | Instant] You Come to a River #0:1 (spell_or_static_text/mode): Fight the Current — Return target nonland permanent to its owner's hand.
     2  'Find a Crossing'  e.g. [afr 2021 | Instant] You Come to a River #0:2 (spell_or_static_text/mode): Find a Crossing — Target creature gets +1/+0 until end of turn and can't be blocked this turn.
     2  'Go to Sleep'  e.g. [unk 2025 | Creature — Sphinx] MagicConsecrated Sphinx #0:5 (spell_or_static_text/mode): Go to Sleep — Exile this creature, then return it to the battlefield under its owner's control.
     2  'Abzan'  e.g. [tdm 2025 | Enchantment] Barrensteppe Siege #0:1 (triggered_ability/mode): Abzan — At the beginning of your end step, put a +1/+1 counter on each creature you control.
     2  'Mardu'  e.g. [tdm 2025 | Enchantment] Barrensteppe Siege #0:2 (triggered_ability/mode): Mardu — At the beginning of your end step, if a creature died under your control this turn, each opponent sacrifices a creature of their choice.
     2  'Jeskai'  e.g. [tdm 2025 | Enchantment] Frostcliff Siege #0:1 (triggered_ability/mode): Jeskai — Whenever one or more creatures you control deal combat damage to a player, draw a card.
     2  'Temur'  e.g. [tdm 2025 | Enchantment] Frostcliff Siege #0:2 (spell_or_static_text/mode): Temur — Creatures you control get +1/+0 and have trample and haste.
     2  'Sultai'  e.g. [tdm 2025 | Enchantment] Glacierwood Siege #0:2 (spell_or_static_text/mode): Sultai — You may play lands from your graveyard.
     1  'Whack'  e.g. [ust 2017 | Artifact] Buzzing Whack-a-Doodle #0:1 (activated_ability/mode): Whack — {T}: Target player loses 2 life.
     1  'Doodle'  e.g. [ust 2017 | Artifact] Buzzing Whack-a-Doodle #0:2 (activated_ability/mode): Doodle — {T}: You gain 3 life.
     1  'Buzz'  e.g. [ust 2017 | Artifact] Buzzing Whack-a-Doodle #0:3 (activated_ability/mode): Buzz — {2}, {T}: Draw a card.
     1  'Flavorful'  e.g. [ust 2017 | Enchantment] Ineffable Blessing #0:1 (triggered_ability/mode): Flavorful — Whenever a creature you control with flavor text enters, draw a card.
     1  'Bland'  e.g. [ust 2017 | Enchantment] Ineffable Blessing #0:2 (triggered_ability/mode): Bland — Whenever a creature you control without flavor text enters, draw a card.
     1  'Mirran'  e.g. [mh1 2019 | Enchantment] Mirrodin Besieged #0:1 (triggered_ability/mode): Mirran — Whenever you cast an artifact spell, create a 1/1 colorless Myr artifact creature token.
     1  'Phyrexian'  e.g. [mh1 2019 | Enchantment] Mirrodin Besieged #0:2 (triggered_ability/mode): Phyrexian — At the beginning of your end step, draw a card, then discard a card. Then if there are fifteen or more artifact cards in your graveyard, t
     1  'Truth'  e.g. [cmb1 2019 | Sorcery] Truth or Dare #0:1 (spell_or_static_text/mode): Truth — That player plays with their hand revealed for the rest of the game.
     1  'Dare'  e.g. [cmb1 2019 | Sorcery] Truth or Dare #0:2 (spell_or_static_text/mode): Dare — Mill all but the bottom ten cards of that player's library.
     1  'Breathe Flame'  e.g. [afc 2021 | Instant] Klauth's Will #0:1 (spell_or_static_text/mode): Breathe Flame — Klauth's Will deals X damage to each creature without flying.
     1  'Smash Relics'  e.g. [afc 2021 | Instant] Klauth's Will #0:2 (spell_or_static_text/mode): Smash Relics — Destroy up to X target artifacts and/or enchantments.
     1  'Two-Weapon Fighting'  e.g. [afr 2021 | Instant] Choose Your Weapon #0:1 (spell_or_static_text/mode): Two-Weapon Fighting — Double target creature's power and toughness until end of turn.
     1  'Archery'  e.g. [afr 2021 | Instant] Choose Your Weapon #0:2 (spell_or_static_text/mode): Archery — This spell deals 5 damage to target creature with flying.
     1  'Bardic Inspiration'  e.g. [afr 2021 | Creature — Elf Bard] Inspiring Bard #0:1 (spell_or_static_text/mode): Bardic Inspiration — Target creature gets +2/+2 until end of turn.
     1  'Song of Rest'  e.g. [afr 2021 | Creature — Elf Bard] Inspiring Bard #0:2 (spell_or_static_text/mode): Song of Rest — You gain 3 life.
     1  'Smash the Chest'  e.g. [afr 2021 | Creature — Dwarf Barbarian] Plundering Barbarian #0:1 (spell_or_static_text/mode): Smash the Chest — Destroy target artifact.
     1  'Pry It Open'  e.g. [afr 2021 | Creature — Dwarf Barbarian] Plundering Barbarian #0:2 (spell_or_static_text/mode): Pry It Open — Create a Treasure token.
     1  'Intimidate Them'  e.g. [afr 2021 | Instant] You Come to the Gnoll Camp #0:1 (spell_or_static_text/mode): Intimidate Them — Up to two target creatures can't block this turn.
     1  'Fend Them Off'  e.g. [afr 2021 | Instant] You Come to the Gnoll Camp #0:2 (spell_or_static_text/mode): Fend Them Off — Target creature gets +3/+1 until end of turn.
     1  'Smash It'  e.g. [afr 2021 | Sorcery] You Find a Cursed Idol #0:1 (spell_or_static_text/mode): Smash It — Destroy target artifact.
     1  'Lift the Curse'  e.g. [afr 2021 | Sorcery] You Find a Cursed Idol #0:2 (spell_or_static_text/mode): Lift the Curse — Destroy target enchantment.
     1  'Steal Its Eyes'  e.g. [afr 2021 | Sorcery] You Find a Cursed Idol #0:3 (spell_or_static_text/mode): Steal Its Eyes — Create a Treasure token and venture into the dungeon.
     1  'Break Their Chains'  e.g. [afr 2021 | Instant] You Find Some Prisoners #0:1 (spell_or_static_text/mode): Break Their Chains — Destroy target artifact.
     1  'Interrogate Them'  e.g. [afr 2021 | Instant] You Find Some Prisoners #0:2 (spell_or_static_text/mode): Interrogate Them — Exile the top three cards of target opponent's library. Choose one of them. Until the end of your next turn, you may play that card

### roman numerals on non-Saga face: 2 units, 2 distinct prefixes (pool 0)
kinds: Counter({'spell_or_static_text': 2})
     1  'C'  e.g. [unf 2022 | Sorcery] Phone a Friend #0:3 (spell_or_static_text/mode): C — Take an extra turn after this one.
     1  'D'  e.g. [unf 2022 | Sorcery] Phone a Friend #0:4 (spell_or_static_text/mode): D — Draw seven cards.

### other (flavor word CR 207.2d, label, or false positive): 1301 units, 598 distinct prefixes (pool 80)
kinds: Counter({'spell_or_static_text': 426, 'triggered_ability': 391, 'activated_ability': 256, 'keyword_ability': 205, 'replacement_effect': 17, 'prevention_effect': 3, 'additional_cost': 2, 'cast_restriction': 1})
    98  '{M}{M}'  e.g. [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:0 (keyword_ability/ability): {TK}{TK} — Afflict 2
    79  'N'  e.g. [ugl 1998 | Instant] Goblin Tutor #0:1 (keyword_ability/ability): 2 — A card named Goblin Tutor
    52  '{M}{M}{M}'  e.g. [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Flying
    50  'Visit'  e.g. [unf 2022 | Artifact — Attraction] Balloon Stand #0:0 (spell_or_static_text/ability): Visit — Choose one.
    46  'Exhaust'  e.g. [dft 2025 | Creature — Goblin Artificer] Afterburner Expert #0:0 (activated_ability/ability): Exhaust — {2}{G}{G}: Put two +1/+1 counters on this creature.
    45  '+ {M}'  e.g. [otj 2024 | Instant] Caught in the Crossfire #0:1 (spell_or_static_text/ability): + {1} — Caught in the Crossfire deals 2 damage to each outlaw creature.
    40  'Max speed'  e.g. [dft 2025 | Artifact] Aether Syphon #0:2 (triggered_ability/ability): Max speed — Whenever you draw a card, each opponent mills two cards.
    37  'Power-up'  e.g. [msc 2026 | Legendary Creature — Human War] Black Panther, Most Dangerous #0:1 (activated_ability/ability): Power-up — {5}{W}{W}: Put two +1/+1 counters on Black Panther. Other creatures you control get +2/+2 until end of turn.
    32  '{M}{M}{M}{M}'  e.g. [sunf 2022 | Stickers] Contortionist Otter Storm #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Deathtouch, lifelink
    26  'Corrupted'  e.g. [onc 2023 | Creature — Phyrexian Druid] Contaminant Grafter #0:3 (triggered_ability/ability): Corrupted — At the beginning of your end step, if an opponent has three or more poison counters, draw a card, then you may put a land card from your h
    25  'Prize'  e.g. [unf 2022 | Artifact — Attraction] Dart Throw #0:1 (spell_or_static_text/ability): Prize — Create two 2/2 pink Teddy Bear creature tokens. Sacrifice this Attraction, then open an Attraction.
    20  '{M}{M}{M}{M}{M}'  e.g. [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/6
    19  'Boast'  e.g. [khm 2021 | Legendary Creature — Human Ber] Arni Brokenbrow #0:1 (activated_ability/ability): Boast — {1}: You may change Arni's base power to 1 plus the greatest power among other creatures you control until end of turn.
    15  "Hero's Reward"  e.g. [tfth 2013 | Creature — Head] Hydra Head #0:0 (triggered_ability/ability): Hero's Reward — When Hydra Head leaves the battlefield, each player gains 2 life.
    15  'To solve'  e.g. [mkc 2024 | Enchantment — Case] Case of the Shifting Visage #0:1 (spell_or_static_text/ability): To solve — There are fifteen or more cards in your graveyard.
    15  'Solved'  e.g. [mkc 2024 | Enchantment — Case] Case of the Shifting Visage #0:2 (triggered_ability/ability): Solved — Whenever you cast a nonlegendary creature spell, copy that spell.
    12  'Forecast'  e.g. [psal 2005 | Enchantment — Aura] Plumes of Peace #0:2 (activated_ability/ability): Forecast — {W}{U}, Reveal this card from your hand: Tap target creature.
    12  'Companion'  e.g. [iko 2020 | Legendary Creature — Demon Kra] Gyruda, Doom of Depths #0:0 (spell_or_static_text/ability): Companion — Your starting deck contains only cards with even mana values.
    12  'Prototype {M}{M}'  e.g. [ybro 2022 | Artifact Creature — Construct] Warzone Duplicator #0:0 (keyword_ability/ability): Prototype {3}{U} — 3/3
    12  'Rulebreaker'  e.g. [unk 2023 | Legendary Creature — Human Kni] Arvad of the Weatherlight #0:0 (spell_or_static_text/ability): Rulebreaker — If Arvad of the Weatherlight is your Commander, you may include legendary permanents of any color in your deck regardless of color ident
    11  'Gotcha'  e.g. [unh 2004 | Creature — Bird] Cardpecker #0:1 (spell_or_static_text/ability): Gotcha — If an opponent touches the table with their hand, you may say "Gotcha!" When you do, return this card from your graveyard to your hand.
     9  'Prototype {M}{M}{M}'  e.g. [bro 2022 | Artifact Creature — Wizard] Arcane Proxy #0:0 (keyword_ability/ability): Prototype {1}{U}{U} — 2/1
     6  '{M}'  e.g. [unk 2025 | Land — Barnyard] Blustering Barnyard #0:3 (spell_or_static_text/ability): {P} — Create a 1/1 white Bird token with flying.
     5  'Mono Eminence'  e.g. [unk 2023 | Legendary Creature — Goblin] Auntie Flint #0:0 (spell_or_static_text/ability): Mono Eminence — If Auntie Flint is on the battlefield or in the command zone and your deck's color identity is mono-black, creatures you control get +
     5  'Will of the Planeswalkers'  e.g. [moc 2023 | Sorcery] Path of the Animist #0:1 (spell_or_static_text/ability): Will of the Planeswalkers — Starting with you, each player votes for planeswalk or chaos. If planeswalk gets more votes, planeswalk. If chaos gets mor
     5  '+ {M}{M}'  e.g. [otj 2024 | Instant] Great Train Heist #0:1 (spell_or_static_text/ability): + {2}{R} — Untap all creatures you control. If it's your combat phase, there is an additional combat phase after this phase.
     4  'Team Cloudspire'  e.g. [unk 2025 | Sorcery] Draw Team Lines #0:0 (spell_or_static_text/ability): Team Cloudspire — Destroy all creatures with power 4 or greater.
     4  'Team Speed Demons'  e.g. [unk 2025 | Sorcery] Draw Team Lines #0:1 (spell_or_static_text/ability): Team Speed Demons — Return all creature cards with power 3 or less from your graveyard to the battlefield.
     3  'Stage N'  e.g. [cmb1 2019 | Creature — Lobster] Loopy Lobster #0:2 (keyword_ability/ability): Stage 2 — Evolve
     3  'Split'  e.g. [afr 2022 | Creature — Ooze] A-Ochre Jelly #0:3 (triggered_ability/ability): Split — When Ochre Jelly dies, if it had two or more +1/+1 counters on it, create a token that's a copy of it at the beginning of the next end step. T
     3  'Jump'  e.g. [fic 2025 | Legendary Creature — Human War] Cid, Freeflier Pilot #0:1 (spell_or_static_text/ability): Jump — During your turn, Cid has flying.
     2  'Legacy'  e.g. [cmb1 2019 | Land] Gold Mine #0:1 (activated_ability/ability): Legacy — {T}, Mark one of Gold Mine's unmarked nodes: Add one mana of any color.
     2  'Bewitching Whispers'  e.g. [afr 2022 | Legendary Creature — Human Elf] A-Shessra, Death's Whisper #0:0 (triggered_ability/ability): Bewitching Whispers — When Shessra, Death's Whisper enters, target creature blocks this turn if able.
     2  'Whispers of the Grave'  e.g. [afr 2022 | Legendary Creature — Human Elf] A-Shessra, Death's Whisper #0:1 (triggered_ability/ability): Whispers of the Grave — At the beginning of your end step, if a creature died this turn, you may pay 2 life. If you do, draw a card.
     2  'Rejuvenation'  e.g. [hbg 2022 | Legendary Creature — Zombie Kn] Vladimir and Godfrey #0:0 (activated_ability/ability): Rejuvenation — {2}{W}: Return Vladimir and Godfrey from your graveyard to the battlefield tapped. It perpetually gets +1/+1. Activate only if you cont
     2  'Tail Spikes'  e.g. [hbg 2022 | Creature — Manticore] A-Manticore #0:2 (triggered_ability/ability): Tail Spikes — When Manticore enters, destroy target creature an opponent controls that was dealt damage this turn.
     2  'Natural Recovery'  e.g. [clb 2022 | Creature — Gnome Druid] Circle of the Land Druid #0:1 (triggered_ability/ability): Natural Recovery — When this creature dies, return target land card from your graveyard to your hand.
     2  'Body Thief'  e.g. [clb 2022 | Creature — Horror] Intellect Devourer #0:1 (spell_or_static_text/ability): Body Thief — You may play lands and cast spells from among cards exiled with this creature. If you cast a spell this way, you may spend mana as though
     2  'Homunculus Servant'  e.g. [clb 2022 | Creature — Bird Artificer] Kenku Artificer #0:0 (triggered_ability/ability): Homunculus Servant — When this creature enters, put three +1/+1 counters on up to one target noncreature artifact. That artifact becomes a 0/0 Homuncu
     2  'Crash Landing'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:0 (spell_or_static_text/ability): Crash Landing — Search your library for a basic land card, reveal it, put it into your hand, then shuffle.
     2  '{M}{M}{M}{M}{M}{M}'  e.g. [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK}{TK} — 10/10
     2  'N or N'  e.g. [unf 2022 | Legendary Planeswalker — Comet] Comet, Stellar Pup #0:1 (spell_or_static_text/ability): 1 or 2 — [+2], then create two 1/1 green Squirrel creature tokens. They gain haste until end of turn.
     2  'Trance'  e.g. [fic 2025 | Legendary Creature — Human Wiz] Terra, Herald of Hope #0:0 (triggered_ability/ability): Trance — At the beginning of combat on your turn, mill two cards. Terra gains flying until end of turn.
     2  '∞'  e.g. [spm 2025 | Legendary Artifact — Infinity ] The Soul Stone #0:3 (triggered_ability/ability): ∞ — At the beginning of your upkeep, return target creature card from your graveyard to the battlefield.
     2  'Federation'  e.g. [trk 2026 | Legendary Creature — Human Sci] Hoshi Sato, Exolinguist #0:0 (triggered_ability/ability): Federation — When Hoshi Sato enters, look at the top X cards of your library, where X is the number of creature types among non-Borg creatures you con
     1  'DCI ruling'  e.g. [ugl 1998 | Sorcery] Once More with Feeling #0:1 (spell_or_static_text/ability): DCI ruling — A deck can have only one card named Once More with Feeling.
     1  'Choose one'  e.g. [tfth 2013 | Sorcery] Noxious Hydra Breath #0:0 (spell_or_static_text/ability): Choose one — Noxious Hydra Breath deals 5 damage to each player; or destroy each tapped non-Head creature.
     1  'Family gathering'  e.g. [hho 2009 | Sorcery] Season's Beatings #0:0 (spell_or_static_text/ability): Family gathering — Each creature target player controls deals damage equal to its power to another random creature that player controls.
     1  'Teamwork'  e.g. [ph18 2019 | Legendary Creature — Dragon An] Sol, Advocate Eternal #0:3 (triggered_ability/ability): Teamwork — Whenever you attack or block with both Sol, Advocate Eternal and its partner, support 4 and investigate four times.
     1  'Landship'  e.g. [cmb1 2019 | Creature — Elf Warrior] Plane-Merge Elf #0:0 (triggered_ability/ability): Landship — At the beginning of your upkeep, you may look at the top card of your library. If it's a land, you may reveal it. If you do, create a 1/1 g
     1  'Kinfall'  e.g. [cmb1 2019 | Creature — Elf Warrior] Plane-Merge Elf #0:1 (triggered_ability/ability): Kinfall — Whenever a creature enters the battlefield under your control, if it shares a creature type with Plane-Merge Elf, creatures you control get 
     1  'Requirement'  e.g. [cmb1 2019 | Vanguard] Ral's Vanguard #0:0 (spell_or_static_text/ability): Requirement — Your starting deck contains only instant, sorcery, and land cards.
     1  'Underdog'
     1  'Ransom'  e.g. [cmb1 2019 | Creature — Squid Pirate] Squidnapper #0:1 (keyword_ability/ability): Ransom — {6} and 2 life
     1  'Berserk'  e.g. [afc 2021 | Artifact Creature — Golem] Clay Golem #0:1 (triggered_ability/ability): Berserk — When this creature becomes monstrous, destroy target permanent.
     1  'Negative Energy Cone'
     1  'Focus Beam'  e.g. [afc 2021 | Enchantment] Netherese Puzzle-Ward #0:0 (triggered_ability/ability): Focus Beam — At the beginning of your upkeep, roll a d4. Scry X, where X is the result.
     1  'Perfect Illumination'  e.g. [afc 2021 | Enchantment] Netherese Puzzle-Ward #0:1 (triggered_ability/ability): Perfect Illumination — Whenever you roll a die's highest natural result, draw a card.
     1  'Mystic Arcanum'  e.g. [afc 2021 | Legendary Creature — Tiefling ] Prosper, Tome-Bound #0:1 (triggered_ability/ability): Mystic Arcanum — At the beginning of your end step, exile the top card of your library. Until the end of your next turn, you may play that card.
     1  'Pact Boon'  e.g. [afc 2021 | Legendary Creature — Tiefling ] Prosper, Tome-Bound #0:2 (triggered_ability/ability): Pact Boon — Whenever you play a card from exile, create a Treasure token.
     1  'Astral Projection'  e.g. [afc 2021 | Artifact — Equipment] Robe of Stars #0:1 (activated_ability/ability): Astral Projection — {1}{W}: Equipped creature phases out.
     1  'Create Undead'  e.g. [afc 2021 | Legendary Creature — Human Wiz] Sefris of the Hidden Ways #0:1 (triggered_ability/ability): Create Undead — Whenever you complete a dungeon, return target creature card from your graveyard to the battlefield.
     1  'Psionic Spells'  e.g. [afr 2021 | Creature — Human Elf Shaman So] Aberrant Mind Sorcerer #0:0 (triggered_ability/ability): Psionic Spells — When this creature enters, choose target instant or sorcery card in your graveyard, then roll a d20.
     1  'Whirlwind'  e.g. [afr 2021 | Creature — Elemental] Air-Cult Elemental #0:1 (triggered_ability/ability): Whirlwind — When this creature enters, return up to one other target creature to its owner's hand.
     1  'Search the Room'  e.g. [afr 2021 | Creature — Elf Wizard] Arcane Investigator #0:0 (activated_ability/ability): Search the Room — {5}{U}: Roll a d20.
     1  'Binding Contract'  e.g. [afr 2021 | Legendary Creature — Devil God] Asmodeus the Archfiend #0:0 (replacement_effect/ability): Binding Contract — If you would draw a card, exile the top card of your library face down instead.
     1  'Acid Breath'  e.g. [afr 2021 | Creature — Dragon] Black Dragon #0:1 (triggered_ability/ability): Acid Breath — When this creature enters, target creature an opponent controls gets -3/-3 until end of turn.
     1  'Teleport'  e.g. [afr 2021 | Creature — Dog] Blink Dog #0:1 (activated_ability/ability): Teleport — {3}{W}: This creature phases out.
     1  'Lightning Breath'  e.g. [afr 2021 | Creature — Dragon] Blue Dragon #0:1 (triggered_ability/ability): Lightning Breath — When this creature enters, until your next turn, target creature an opponent controls gets -3/-0, up to one other target creature g
     1  'Wild Magic Surge'  e.g. [afr 2021 | Creature — Human Shaman] Chaos Channeler #0:0 (triggered_ability/ability): Wild Magic Surge — Whenever this creature attacks, roll a d20.
     1  'Bear Form'  e.g. [afr 2021 | Creature — Human Elf Druid] Circle of the Moon Druid #0:0 (spell_or_static_text/ability): Bear Form — During your turn, this creature is a Bear with base power and toughness 4/2.
     1  'Mage Hand'  e.g. [afr 2021 | Creature — Gnome Wizard] Clever Conjurer #0:0 (activated_ability/ability): Mage Hand — {T}: Untap target permanent not named Clever Conjurer. Activate only as a sorcery.
     1  'Beacon of Hope'  e.g. [afr 2021 | Creature — Orc Knight] Devoted Paladin #0:0 (triggered_ability/ability): Beacon of Hope — When this creature enters, creatures you control get +1/+1 and gain vigilance until end of turn.
     1  'Displacement'  e.g. [afr 2021 | Creature — Cat Beast] Displacer Beast #0:1 (activated_ability/ability): Displacement — {3}{U}: Return this creature to its owner's hand.
     1  'Drag Below'  e.g. [afr 2021 | Creature — Dragon Turtle] Dragon Turtle #0:1 (triggered_ability/ability): Drag Below — When this creature enters, tap it and up to one target creature an opponent controls. They don't untap during their controllers' next unt
     1  'Siege Monster'  e.g. [afr 2021 | Creature — Elemental] Earth-Cult Elemental #0:0 (triggered_ability/ability): Siege Monster — When this creature enters, roll a d20.
     1  "Dark One's Own Luck"
     1  'Climb Over'  e.g. [afr 2021 | Artifact] Fifty Feet of Rope #0:0 (activated_ability/ability): Climb Over — {T}: Target Wall can't block this turn.
     1  'Tie Up'  e.g. [afr 2021 | Artifact] Fifty Feet of Rope #0:1 (activated_ability/ability): Tie Up — {3}, {T}: Target creature doesn't untap during its controller's next untap step.
     1  'Rappel Down'  e.g. [afr 2021 | Artifact] Fifty Feet of Rope #0:2 (activated_ability/ability): Rappel Down — {4}, {T}: Venture into the dungeon. Activate only as a sorcery.
     1  'Engulf'  e.g. [afr 2021 | Creature — Ooze] Gelatinous Cube #0:0 (triggered_ability/ability): Engulf — When this creature enters, exile target non-Ooze creature an opponent controls until this creature leaves the battlefield.
     1  'Dissolve'  e.g. [afr 2021 | Creature — Ooze] Gelatinous Cube #0:1 (activated_ability/ability): Dissolve — {X}{B}: Put target creature card with mana value X exiled with this creature into its owner's graveyard.
     1  'Poison Breath'  e.g. [afr 2021 | Creature — Dragon] Green Dragon #0:1 (triggered_ability/ability): Poison Breath — When this creature enters, until end of turn, whenever a creature an opponent controls is dealt damage, destroy it.
     1  'Tragic Backstory'  e.g. [afr 2021 | Creature — Goblin Warlock] Grim Wanderer #0:1 (cast_restriction/ability): Tragic Backstory — Cast this spell only if a creature died this turn.
     1  'Cunning Action'  e.g. [afr 2021 | Creature — Orc Rogue] Guild Thief #0:1 (activated_ability/ability): Cunning Action — {3}{U}: This creature can't be blocked this turn.
     1  'Stunning Strike'  e.g. [afr 2021 | Creature — Human Elf Monk] Half-Elf Monk #0:1 (activated_ability/ability): Stunning Strike — {1}{W}, {T}: Tap target creature.
     1  'Circle of Death'  e.g. [afr 2021 | Creature — Human Warlock] Herald of Hadar #0:0 (activated_ability/ability): Circle of Death — {5}{B}: Roll a d20.
     1  'Sneak Attack'  e.g. [afr 2021 | Creature — Halfling Rogue] Lightfoot Rogue #0:0 (triggered_ability/ability): Sneak Attack — Whenever this creature attacks, roll a d20.
     1  'Dominate Monster'  e.g. [afr 2021 | Creature — Horror] Mind Flayer #0:0 (triggered_ability/ability): Dominate Monster — When this creature enters, gain control of target creature for as long as you control this creature.
     1  'Flurry of Blows'  e.g. [afr 2021 | Creature — Elf Monk] Monk of the Open Hand #0:0 (triggered_ability/ability): Flurry of Blows — Whenever you cast your second spell each turn, put a +1/+1 counter on this creature.
     1  'Divine Intervention'  e.g. [afr 2021 | Creature — Human Elf Cleric] Moon-Blessed Cleric #0:0 (triggered_ability/ability): Divine Intervention — When this creature enters, you may search your library for an enchantment card, reveal it, then shuffle and put that card on top
     1  'Magical Tinkering'  e.g. [afr 2021 | Legendary Creature — Gnome Art] Oswald Fiddlebender #0:0 (activated_ability/ability): Magical Tinkering — {W}, {T}, Sacrifice an artifact: Search your library for an artifact card with mana value equal to 1 plus the sacrificed artifact'
     1  'Keen Senses'  e.g. [afr 2021 | Creature — Bird Bear] Owlbear #0:1 (triggered_ability/ability): Keen Senses — When this creature enters, draw a card.
     1  'Grant an Advantage'  e.g. [afr 2021 | Creature — Faerie] Pixie Guide #0:1 (replacement_effect/ability): Grant an Advantage — If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.
     1  'Fire Breath'
     1  'Cone of Cold'  e.g. [afr 2021 | Creature — Tiefling Shaman] Scion of Stygia #0:1 (triggered_ability/ability): Cone of Cold — When this creature enters, choose target creature an opponent controls, then roll a d20.
     1  'Animate Walking Statue'  e.g. [afr 2021 | Legendary Artifact] The Blackstaff of Waterdeep #0:1 (activated_ability/ability): Animate Walking Statue — {1}{U}, {T}: Another target nontoken artifact you control becomes a 4/4 artifact creature for as long as The Blackstaff of Wa
     1  'N | Trapped!'  e.g. [afr 2021 | Artifact] Treasure Chest #0:1 (spell_or_static_text/ability): 1 | Trapped! — You lose 3 life.
     1  'Invoke Duplicity'  e.g. [afr 2021 | Artifact — Equipment] Trickster's Talisman #0:0 (spell_or_static_text/ability): Invoke Duplicity — Equipped creature gets +1/+1 and has "Whenever this creature deals combat damage to a player, you may sacrifice Trickster's Talisma
     1  'Combat Inspiration'  e.g. [afr 2021 | Creature — Tiefling Bard] Valor Singer #0:0 (triggered_ability/ability): Combat Inspiration — At the beginning of combat on your turn, target creature you control gets +1/+0 until end of turn.
     1  'Cold Breath'  e.g. [afr 2021 | Creature — Dragon] White Dragon #0:1 (triggered_ability/ability): Cold Breath — When this creature enters, tap target creature an opponent controls. That creature doesn't untap during its controller's next untap step
     1  'Life Drain'  e.g. [afr 2021 | Creature — Zombie Soldier] Wight #0:1 (triggered_ability/ability): Life Drain — Whenever a creature dealt damage by this creature this turn dies, create a tapped 2/2 black Zombie creature token and exile that card.
     1  'Yawning Portal'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:0 (spell_or_static_text/ability): Yawning Portal — You gain 1 life.
     1  'Dungeon Level'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:1 (spell_or_static_text/ability): Dungeon Level — Scry 1.
     1  'Goblin Bazaar'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:2 (spell_or_static_text/ability): Goblin Bazaar — Create a Treasure token.
     1  'Twisted Caverns'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:3 (spell_or_static_text/ability): Twisted Caverns — Target creature can't attack until your next turn.
     1  'Lost Level'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:4 (spell_or_static_text/ability): Lost Level — Scry 2.
     1  'Runestone Caverns'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:5 (spell_or_static_text/ability): Runestone Caverns — Exile the top two cards of your library. You may play them.
     1  "Muiral's Graveyard"  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:6 (spell_or_static_text/ability): Muiral's Graveyard — Create two 1/1 black Skeleton creature tokens.
     1  'Deep Mines'  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:7 (spell_or_static_text/ability): Deep Mines — Scry 3.
     1  "Mad Wizard's Lair"  e.g. [oafr 2021 | Dungeon] Dungeon of the Mad Mage #0:8 (spell_or_static_text/ability): Mad Wizard's Lair — Draw three cards and reveal them. You may cast one of them without paying its mana cost.
     1  'Cave Entrance'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:0 (spell_or_static_text/ability): Cave Entrance — Scry 1.
     1  'Goblin Lair'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:1 (spell_or_static_text/ability): Goblin Lair — Create a 1/1 red Goblin creature token.
     1  'Mine Tunnels'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:2 (spell_or_static_text/ability): Mine Tunnels — Create a Treasure token.
     1  'Storeroom'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:3 (spell_or_static_text/ability): Storeroom — Put a +1/+1 counter on target creature.
     1  'Dark Pool'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:4 (spell_or_static_text/ability): Dark Pool — Each opponent loses 1 life and you gain 1 life.
     1  'Fungi Cavern'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:5 (spell_or_static_text/ability): Fungi Cavern — Target creature gets -4/-0 until your next turn.
     1  'Temple of Dumathoin'  e.g. [oafr 2021 | Dungeon] Lost Mine of Phandelver #0:6 (spell_or_static_text/ability): Temple of Dumathoin — Draw a card.
     1  'Trapped Entry'  e.g. [oafr 2021 | Dungeon] Tomb of Annihilation #0:0 (spell_or_static_text/ability): Trapped Entry — Each player loses 1 life.
     1  'Veils of Fear'  e.g. [oafr 2021 | Dungeon] Tomb of Annihilation #0:1 (spell_or_static_text/ability): Veils of Fear — Each player loses 2 life unless they discard a card.
     1  'Sandfall Cell'  e.g. [oafr 2021 | Dungeon] Tomb of Annihilation #0:2 (spell_or_static_text/ability): Sandfall Cell — Each player loses 2 life unless they sacrifice a creature, artifact, or land of their choice.
     1  'Oubliette'  e.g. [oafr 2021 | Dungeon] Tomb of Annihilation #0:3 (spell_or_static_text/ability): Oubliette — Discard a card and sacrifice a creature, an artifact, and a land.
     1  'Cradle of the Death God'  e.g. [oafr 2021 | Dungeon] Tomb of Annihilation #0:4 (spell_or_static_text/ability): Cradle of the Death God — Create The Atropal, a legendary 4/4 black God Horror creature token with deathtouch.
     1  'Probing Telepathy'  e.g. [clb 2022 | Creature — Fish Horror] Aboleth Spawn #0:2 (triggered_ability/ability): Probing Telepathy — Whenever a creature entering under an opponent's control causes a triggered ability of that creature to trigger, you may copy that
     1  'Project Image'  e.g. [clb 2022 | Creature — Dragon] Astral Dragon #0:1 (triggered_ability/ability): Project Image — When this creature enters, create two tokens that are copies of target noncreature permanent, except they're 3/3 Dragon creatures in a
     1  'Wind Walk'  e.g. [clb 2022 | Creature — Human Cleric] Bane's Invoker #0:0 (activated_ability/ability): Wind Walk — {8}: Up to two target creatures each get +2/+2 and gain flying until end of turn.
     1  'Lure the Unwary'  e.g. [clb 2022 | Creature — Spirit] Beckoning Will-o'-Wisp #0:1 (triggered_ability/ability): Lure the Unwary — At the beginning of combat on your turn, choose an opponent.
     1  'Scorching Ray'  e.g. [clb 2022 | Creature — Dragon Shaman] Bhaal's Invoker #0:0 (activated_ability/ability): Scorching Ray — {8}: This creature deals 4 damage to each opponent.
     1  'Crown of Madness'  e.g. [clb 2022 | Creature — Human Shaman Sorcer] Bloodboil Sorcerer #0:1 (activated_ability/ability): Crown of Madness — {1}{R}, Sacrifice an artifact or creature: Goad target creature.
     1  'Animate Chains'  e.g. [clb 2022 | Creature — Devil] Chain Devil #0:0 (triggered_ability/ability): Animate Chains — When this creature enters, each player sacrifices a nontoken creature of their choice.
     1  'Gathered Swarm'  e.g. [clb 2022 | Creature — Elf Ranger] Cloakwood Swarmkeeper #0:0 (triggered_ability/ability): Gathered Swarm — Whenever one or more tokens you control enter, put a +1/+1 counter on this creature.
     1  'Mold Earth'  e.g. [clb 2022 | Creature — Gnome Wizard] Deep Gnome Terramancer #0:1 (triggered_ability/ability): Mold Earth — Whenever one or more lands enter under an opponent's control without being played, you may search your library for a Plains card, put it 
     1  'Avoidance'  e.g. [clb 2022 | Creature — Cat Beast] Displacer Kitten #0:0 (triggered_ability/ability): Avoidance — Whenever you cast a noncreature spell, exile up to one target nonland permanent you control, then return that card to the battlefield unde
     1  'Enthralling Performance'  e.g. [clb 2022 | Creature — Giant Bard] Firbolg Flutist #0:0 (triggered_ability/ability): Enthralling Performance — When this creature enters, gain control of target creature you don't control until end of turn. Untap it. It gains haste and
     1  'Psychic Defense'  e.g. [clb 2022 | Creature — Gith Monk] Githzerai Monk #0:2 (triggered_ability/ability): Psychic Defense — When this creature enters, tap all creatures you don't control.
     1  'Aberrant Tinkering'  e.g. [clb 2022 | Creature — Horror Wizard] Grell Philosopher #0:0 (triggered_ability/ability): Aberrant Tinkering — When this creature enters and at the beginning of your upkeep, each Horror you control gains all activated abilities of target ar
     1  'Protection Fighting Style'  e.g. [clb 2022 | Creature — Tiefling Warrior] Icewind Stalwart #0:0 (triggered_ability/ability): Protection Fighting Style — When this creature enters, exile up to one target non-Warrior creature you control, then return it to the battlefield unde
     1  'Ceremorphosis'  e.g. [clb 2022 | Creature — Horror] Illithid Harvester // Plant Tadpoles #0:0 (triggered_ability/ability): Ceremorphosis — When this creature enters, turn any number of target tapped nontoken creatures face down. They're 2/2 Horror creatures.
     1  'Vicious Mockery'  e.g. [clb 2022 | Creature — Dwarf Bard] Insufferable Balladeer #0:0 (triggered_ability/ability): Vicious Mockery — When this creature enters, target creature an opponent controls can't block this turn. Goad it.
     1  'Devour Intellect'  e.g. [clb 2022 | Creature — Horror] Intellect Devourer #0:0 (triggered_ability/ability): Devour Intellect — When this creature enters, each opponent exiles a card from their hand until this creature leaves the battlefield.
     1  'Confounding Clouds'  e.g. [clb 2022 | Creature — Dragon] Juvenile Mist Dragon #0:1 (triggered_ability/ability): Confounding Clouds — When this creature enters, for each opponent, tap up to one target creature that player controls. Each of those creatures doesn't
     1  'Loud Ruckus'  e.g. [clb 2022 | Enchantment] Loot Dispute #0:2 (triggered_ability/ability): Loud Ruckus — Whenever you complete a dungeon, create a 5/5 red Dragon creature token with flying.
     1  'Mold Harvest'  e.g. [clb 2022 | Creature — Fungus Warrior] Mold Folk #0:1 (activated_ability/ability): Mold Harvest — {1}, Sacrifice another creature or an artifact: Put a +1/+1 counter on this creature.
     1  'Infesting Spores'  e.g. [clb 2022 | Creature — Fungus] Myconid Spore Tender #0:0 (triggered_ability/ability): Infesting Spores — When this creature enters, destroy up to one target artifact or enchantment.
     1  'Psychic Blades'  e.g. [clb 2022 | Creature — Cat Rogue] Myrkul's Invoker #0:0 (activated_ability/ability): Psychic Blades — {8}: Creatures you control get +2/+0 and gain menace until end of turn.
     1  "Bigby's Hand"  e.g. [clb 2022 | Creature — Dragon Wizard] Nimbleclaw Adept #0:0 (activated_ability/ability): Bigby's Hand — {T}: Untap two other target permanents. Activate only as a sorcery and only once each turn.
     1  'Weird Insight'  e.g. [clb 2022 | Creature — Horror] Nothic #0:0 (triggered_ability/ability): Weird Insight — When this creature dies, roll a d20.
     1  "Mama's Coming"  e.g. [clb 2022 | Creature — Bird Bear] Owlbear Cub #0:0 (triggered_ability/ability): Mama's Coming — Whenever this creature attacks a player who controls eight or more lands, look at the top eight cards of your library. You may put a c
     1  'Natural Shelter'  e.g. [clb 2022 | Creature — Elemental Spirit] Rescuer Chwinga #0:1 (triggered_ability/ability): Natural Shelter — When this creature enters, you may return another permanent you control to its owner's hand.
     1  'Keen Sight'  e.g. [clb 2022 | Creature — Bird] Scouting Hawk #0:1 (triggered_ability/ability): Keen Sight — When this creature enters, if an opponent controls more lands than you, search your library for a basic Plains card, put it onto the batt
     1  'Conjure Elemental'  e.g. [clb 2022 | Creature — Dragon Druid] Silvanus's Invoker #0:0 (activated_ability/ability): Conjure Elemental — {8}: Untap target land you control. It becomes an 8/8 Elemental creature with trample and haste until end of turn. It's still a la
     1  'Blood Drain'  e.g. [clb 2022 | Creature — Insect Bat] Stirge #0:2 (activated_ability/ability): Blood Drain — {1}{B}, Pay 1 life, Sacrifice this creature: Draw a card.
     1  'Mantle of Inspiration'  e.g. [clb 2022 | Creature — Dragon Bard] Stirring Bard #0:2 (activated_ability/ability): Mantle of Inspiration — {T}: Target creature gains menace and haste until end of turn.
     1  'Sleight of Hand'  e.g. [clb 2022 | Creature — Orc Rogue] Tymora's Invoker #0:0 (activated_ability/ability): Sleight of Hand — {8}: Draw two cards.
     1  'Horrific Symbiosis'  e.g. [clb 2022 | Creature — Crab Ooze Horror] Uchuulon #0:1 (triggered_ability/ability): Horrific Symbiosis — At the beginning of your end step, exile up to one target creature card from an opponent's graveyard. If you do, create a token t
     1  'Spiked Retribution'  e.g. [clb 2022 | Creature — Dwarf Barbarian] Vicious Battlerager #0:1 (triggered_ability/ability): Spiked Retribution — Whenever this creature becomes blocked by a creature, that creature's controller loses 5 life.
     1  'Toxic Spores'
     1  'Gust of Wind'  e.g. [clb 2022 | Creature — Faerie Elf Wizard] Winter Eladrin #0:0 (triggered_ability/ability): Gust of Wind — When this creature enters, return up to one other target creature to its owner's hand.
     1  'Hive Mind'  e.g. [clb 2022 | Legendary Creature — Horror] Zellix, Sanity Flayer #0:0 (triggered_ability/ability): Hive Mind — Whenever a player mills one or more creature cards, you create a 1/1 black Horror creature token.
     1  'Secret Entrance'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:2 (spell_or_static_text/ability): Secret Entrance — Search your library for a basic land card, reveal it, put it into your hand, then shuffle.
     1  'Forge'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:3 (spell_or_static_text/ability): Forge — Put two +1/+1 counters on target creature.
     1  'Lost Well'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:4 (spell_or_static_text/ability): Lost Well — Scry 2.
     1  'Trap!'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:5 (spell_or_static_text/ability): Trap! — Target player loses 5 life.
     1  'Arena'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:6 (spell_or_static_text/ability): Arena — Goad target creature.
     1  'Stash'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:7 (spell_or_static_text/ability): Stash — Create a Treasure token.
     1  'Archives'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:8 (spell_or_static_text/ability): Archives — Draw a card.
     1  'Catacombs'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:9 (spell_or_static_text/ability): Catacombs — Create a 4/1 black Skeleton creature token with menace.
     1  'Throne of the Dead Three'  e.g. [oclb 2022 | Dungeon — Undercity] Undercity // The Initiative #0:10 (spell_or_static_text/ability): Throne of the Dead Three — Reveal the top ten cards of your library. Put a creature card from among them onto the battlefield with three +1/+1 counter
     1  'Goblin Camp'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:1 (spell_or_static_text/ability): Goblin Camp — Create a Treasure token.
     1  'Emerald Grove'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:2 (spell_or_static_text/ability): Emerald Grove — Create a 2/2 white Knight creature token.
     1  "Auntie's Teahouse"  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:3 (spell_or_static_text/ability): Auntie's Teahouse — Scry 3.
     1  'Defiled Temple'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:4 (spell_or_static_text/ability): Defiled Temple — You may sacrifice a permanent. If you do, draw a card.
     1  'Mountain Pass'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:5 (spell_or_static_text/ability): Mountain Pass — You may put a land card from your hand onto the battlefield.
     1  'Ebonlake Grotto'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:6 (spell_or_static_text/ability): Ebonlake Grotto — Create two 1/1 blue Faerie Dragon creature tokens with flying.
     1  'Grymforge'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:7 (spell_or_static_text/ability): Grymforge — For each opponent, goad up to one target creature that player controls.
     1  'Githyanki Crèche'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:8 (spell_or_static_text/ability): Githyanki Crèche — Distribute three +1/+1 counters among up to three target creatures you control.
     1  'Last Light Inn'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:9 (spell_or_static_text/ability): Last Light Inn — Draw two cards.
     1  'Reithwin Tollhouse'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:10 (spell_or_static_text/ability): Reithwin Tollhouse — Roll 2d4 and create that many Treasure tokens.
     1  'Moonrise Towers'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:11 (spell_or_static_text/ability): Moonrise Towers — Instant and sorcery spells you cast this turn cost {3} less to cast.
     1  'Gauntlet of Shar'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:12 (spell_or_static_text/ability): Gauntlet of Shar — Each opponent loses 5 life.
     1  "Balthazar's Lab"  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:13 (spell_or_static_text/ability): Balthazar's Lab — Return up to two target creature cards from your graveyard to your hand.
     1  'Circus of the Last Days'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:14 (spell_or_static_text/ability): Circus of the Last Days — Create a token that's a copy of one of your commanders, except it's not legendary.
     1  'Undercity Ruins'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:15 (spell_or_static_text/ability): Undercity Ruins — Create three 4/1 black Skeleton creature tokens with menace.
     1  'Steel Watch Foundry'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:16 (spell_or_static_text/ability): Steel Watch Foundry — You get an emblem with "Creatures you control get +2/+2 and have trample."
     1  "Ansur's Sanctum"  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:18 (spell_or_static_text/ability): Ansur's Sanctum — Reveal the top four cards of your library and put them into your hand. Each opponent loses life equal to those cards' total mana val
     1  'Temple of Bhaal'  e.g. [tclb 2022 | Dungeon] Baldur's Gate Wilderness #0:19 (spell_or_static_text/ability): Temple of Bhaal — Creatures your opponents control get -5/-5 until end of turn.
     1  "Calim's Breath"  e.g. [hbg 2022 | Legendary Creature — Djinn Nob] Calim, Djinn Emperor #0:2 (activated_ability/ability): Calim's Breath — {1}{U}, Discard Calim: Tap up to one target nonland permanent. Draw a card. Then you may exile two other cards named Calim, Djinn Emp
     1  'Gift of Tiamat'  e.g. [hbg 2022 | Creature — Dragon Shaman] Dragonborn Immolator #0:1 (triggered_ability/ability): Gift of Tiamat — When this creature dies, if its power is greater than 0, note its power. You get a one-time boon with "When you cast a creature spell
     1  'Molting Exoskeleton'  e.g. [hbg 2022 | Creature — Horror] Hook Horror #0:0 (triggered_ability/ability): Molting Exoskeleton — When Hook Horror dies, it perpetually gets -1/-1. Then if that card's toughness is 1 or greater, return it to the battlefield un
     1  'Rage Beyond Death'
     1  'Wild Shape'  e.g. [hbg 2022 | Legendary Creature — Human Dru] Lukamina, Moon Druid #0:0 (spell_or_static_text/ability): Wild Shape — Specialize {3}. Activate only if you control six or more lands.
     1  'Psionic Adept'  e.g. [hbg 2022 | Creature — Gith Monk] Wizened Githzerai #0:0 (triggered_ability/ability): Psionic Adept — Whenever Wizened Githzerai becomes blocked by a creature, that creature perpetually gets -2/-0.
     1  'Mark of Chaos Ascendant'  e.g. [40k 2022 | Legendary Creature — Astartes ] Abaddon the Despoiler #0:1 (spell_or_static_text/ability): Mark of Chaos Ascendant — During your turn, spells you cast from your hand with mana value X or less have cascade, where X is the total amount of life
     1  'Heavy Power Hammer'  e.g. [40k 2022 | Creature — Tyranid Mutant] Aberrant #0:2 (triggered_ability/ability): Heavy Power Hammer — Whenever this creature deals combat damage to a player, destroy target artifact or enchantment that player controls.
     1  'Heavy Rock Cutter'  e.g. [40k 2022 | Creature — Tyranid Human] Acolyte Hybrid #0:0 (triggered_ability/ability): Heavy Rock Cutter — Whenever this creature attacks, destroy up to one target artifact. If an artifact is destroyed this way, its controller draws a ca
     1  'Lord of the Pyrrhian Legions'  e.g. [40k 2022 | Legendary Artifact Creature — ] Anrakyr the Traveller #0:0 (triggered_ability/ability): Lord of the Pyrrhian Legions — Whenever Anrakyr the Traveller attacks, you may cast an artifact spell from your hand or graveyard by paying life equal
     1  'Endurant'  e.g. [40k 2022 | Creature — Human] Arco-Flagellant #0:2 (activated_ability/ability): Endurant — Pay 3 life: This creature gains indestructible until end of turn.
     1  'Ruinous Ascension'  e.g. [40k 2022 | Creature — Astartes Warrior] Aspiring Champion #0:1 (triggered_ability/ability): Ruinous Ascension — When this creature deals combat damage to a player, sacrifice it. If you do, reveal cards from the top of your library until you r
     1  'Chainsword'  e.g. [40k 2022 | Creature — Astartes Warrior] Assault Intercessor #0:2 (triggered_ability/ability): Chainsword — Whenever a creature an opponent controls dies, that player loses 2 life.
     1  'Skilled Outrider'  e.g. [40k 2022 | Creature — Human Tyranid Scout] Atalan Jackal #0:2 (triggered_ability/ability): Skilled Outrider — Whenever this creature deals combat damage to a player, you may search your library for a basic land card, put it onto the battlefi
     1  'Prince of Chaos'  e.g. [40k 2022 | Legendary Creature — Demon Nob] Be'lakor, the Dark Master #0:1 (triggered_ability/ability): Prince of Chaos — When Be'lakor enters, you draw X cards and you lose X life, where X is the number of Demons you control.
     1  'Lord of Torment'  e.g. [40k 2022 | Legendary Creature — Demon Nob] Be'lakor, the Dark Master #0:2 (triggered_ability/ability): Lord of Torment — Whenever another Demon you control enters, it deals damage equal to its power to any target.
     1  'Ultima Founding'  e.g. [40k 2022 | Legendary Artifact Creature — ] Belisarius Cawl #0:0 (activated_ability/ability): Ultima Founding — {T}, Tap two untapped artifacts you control: Create a 2/2 white Astartes Warrior creature token with vigilance.
     1  'Master of Machines'  e.g. [40k 2022 | Legendary Artifact Creature — ] Belisarius Cawl #0:1 (activated_ability/ability): Master of Machines — {T}, Tap X untapped creatures you control: Look at the top X cards of your library. You may reveal an artifact card from among th
     1  'Genomic Enhancement'  e.g. [40k 2022 | Creature — Human Tyranid Wizar] Biophagus #0:0 (activated_ability/ability): Genomic Enhancement — {T}: Add one mana of any color. If this mana is spent to cast a creature spell, that creature enters with an additional +1/+1 co
     1  'Devastating Charge'  e.g. [40k 2022 | Creature — Demon Knight] Bloodcrusher of Khorne #0:1 (spell_or_static_text/ability): Devastating Charge — Other creatures you control have trample.
     1  'Brood Telepathy'  e.g. [40k 2022 | Creature — Tyranid] Broodlord #0:1 (triggered_ability/ability): Brood Telepathy — When this creature enters, distribute X +1/+1 counters among any number of other target creatures you control.
     1  'Polymorphine'  e.g. [40k 2022 | Creature — Human Shapeshifter ] Callidus Assassin #0:1 (replacement_effect/ability): Polymorphine — You may have this creature enter tapped as a copy of any creature on the battlefield, except it has "When this creature enters, destroy
     1  'Feeder Mandibles'  e.g. [40k 2022 | Artifact Creature — Insect] Canoptek Scarab Swarm #0:1 (triggered_ability/ability): Feeder Mandibles — When this creature enters, exile target player's graveyard. For each artifact or land card exiled this way, create a 1/1 colorless 
     1  'Fabricator Claw Array'  e.g. [40k 2022 | Artifact Creature — Spider] Canoptek Spyder #0:1 (triggered_ability/ability): Fabricator Claw Array — Whenever another nontoken artifact creature or Vehicle you control enters, draw a card.
     1  'Exile Cannon'  e.g. [40k 2022 | Artifact Creature — Insect] Canoptek Tomb Sentinel #0:1 (triggered_ability/ability): Exile Cannon — When this creature enters from a graveyard, exile up to one target nonland permanent.
     1  'Wraith Form'  e.g. [40k 2022 | Artifact Creature — Wraith] Canoptek Wraith #0:0 (spell_or_static_text/ability): Wraith Form — This creature can't be blocked.
     1  'Transdimensional Scout'  e.g. [40k 2022 | Artifact Creature — Wraith] Canoptek Wraith #0:1 (triggered_ability/ability): Transdimensional Scout — When this creature deals combat damage to a player, you may pay {3} and sacrifice it. If you do, choose a land you control. T
     1  'Healing Tears'  e.g. [40k 2022 | Legendary Creature — Human War] Celestine, the Living Saint #0:2 (triggered_ability/ability): Healing Tears — At the beginning of your end step, return target creature card with mana value X or less from your graveyard to the battlefield, where
     1  'Battle Cannon'  e.g. [40k 2022 | Artifact Creature — Demon Cons] Chaos Defiler #0:1 (triggered_ability/ability): Battle Cannon — When this creature enters or dies, for each opponent, choose a nonland permanent that player controls. Destroy one of them chosen at r
     1  'Lord of Chaos'  e.g. [40k 2022 | Creature — Astartes Warrior] Chaos Terminator Lord #0:0 (triggered_ability/ability): Lord of Chaos — At the beginning of combat on your turn, another target creature you control gains double strike until end of turn.
     1  'Atomic Transmutation'  e.g. [40k 2022 | Artifact Creature — Necron Wiz] Chronomancer #0:1 (activated_ability/ability): Atomic Transmutation — {1}, {T}, Sacrifice another artifact: Draw a card.
     1  'Proclamator Hailer'  e.g. [40k 2022 | Creature — Human Tyranid Artif] Clamavus #0:0 (spell_or_static_text/ability): Proclamator Hailer — Each creature you control gets +1/+1 for each +1/+1 counter on it.
     1  'Leading from the Front'  e.g. [40k 2022 | Legendary Creature — Human Sol] Commissar Severina Raine #0:0 (triggered_ability/ability): Leading from the Front — Whenever Commissar Severina Raine attacks, each opponent loses X life, where X is the number of other attacking creatures.
     1  'Summary Execution'  e.g. [40k 2022 | Legendary Creature — Human Sol] Commissar Severina Raine #0:1 (activated_ability/ability): Summary Execution — {2}, Sacrifice another creature: You gain 2 life and draw a card.
     1  'Command Section'  e.g. [40k 2022 | Creature — Human Soldier] Company Commander #0:0 (triggered_ability/ability): Command Section — When this creature enters, create a number of 1/1 white Soldier creature tokens equal to the number of opponents you have.
     1  'Bring it Down!'  e.g. [40k 2022 | Creature — Human Soldier] Company Commander #0:1 (triggered_ability/ability): Bring it Down! — Whenever this creature attacks, creatures you control gain deathtouch until end of turn.
     1  'Dynastic Command Node'  e.g. [40k 2022 | Artifact] Convergence of Dominion #0:0 (spell_or_static_text/ability): Dynastic Command Node — As long as you control your commander, activated abilities of cards in your graveyard cost {2} less to activate. This effect c
     1  'Translocation Protocols'  e.g. [40k 2022 | Artifact] Convergence of Dominion #0:1 (activated_ability/ability): Translocation Protocols — {3}, {T}: Mill three cards.
     1  'Protector'  e.g. [40k 2022 | Artifact Creature — Construct] Cryptothrall #0:0 (spell_or_static_text/ability): Protector — Other artifact creatures you control have hexproof.
     1  'Field Reprogramming'  e.g. [40k 2022 | Artifact Creature — Human Arti] Cybernetica Datasmith #0:1 (activated_ability/ability): Field Reprogramming — {U}, {T}: Target player draws a card. Another target player creates a 4/4 colorless Robot artifact creature token with "This tok
     1  'Gift of Chaos'  e.g. [40k 2022 | Creature — Astartes Warlock] Dark Apostle #0:0 (activated_ability/ability): Gift of Chaos — {3}, {T}: The next noncreature spell you cast this turn has cascade.
     1  'Flesh Hooks'  e.g. [40k 2022 | Legendary Creature — Tyranid] Deathleaper, Terror Weapon #0:2 (spell_or_static_text/ability): Flesh Hooks — Creatures you control that entered this turn have double strike.
     1  'Echo of the First Murder'  e.g. [40k 2022 | Legendary Artifact — Equipment] Drach'Nyen #0:0 (triggered_ability/ability): Echo of the First Murder — When Drach'Nyen enters, exile up to one target creature.
     1  'Daemon Sword'  e.g. [40k 2022 | Legendary Artifact — Equipment] Drach'Nyen #0:1 (spell_or_static_text/ability): Daemon Sword — Equipped creature has menace and gets +X/+0, where X is the exiled card's power.
     1  'Veil of Time'  e.g. [40k 2022 | Creature — Astartes Wizard] Epistolary Librarian #0:0 (triggered_ability/ability): Veil of Time — Whenever this creature attacks, you may cast a spell with mana value X or less from your hand without paying its mana cost, where X is 
     1  'Sorcerous Inspiration'  e.g. [40k 2022 | Creature — Demon] Exalted Flamer of Tzeentch #0:0 (triggered_ability/ability): Sorcerous Inspiration — At the beginning of your upkeep, return an instant or sorcery card at random from your graveyard to your hand.
     1  'Fire of Tzeentch'  e.g. [40k 2022 | Creature — Demon] Exalted Flamer of Tzeentch #0:1 (triggered_ability/ability): Fire of Tzeentch — Whenever you cast an instant or sorcery spell, this creature deals 1 damage to each opponent.
     1  'Bio-plasmic Barrage'  e.g. [40k 2022 | Creature — Tyranid] Exocrine #0:1 (triggered_ability/ability): Bio-plasmic Barrage — When this creature enters, it deals X damage to each player and each other creature.
     1  'Flesh Flayer'  e.g. [40k 2022 | Artifact Creature — Necron] Flayed One #0:1 (triggered_ability/ability): Flesh Flayer — When this creature enters, mill three cards.
     1  'Skyswarm'  e.g. [40k 2022 | Creature — Tyranid Gargoyle] Gargoyle Flock #0:1 (triggered_ability/ability): Skyswarm — At the beginning of your end step, if a creature entered the battlefield under your control this turn, create a 1/1 blue Tyranid Gargoyle c
     1  'Neurotraumal Rod'  e.g. [40k 2022 | Creature — Tyranid Human] Genestealer Locus #0:0 (triggered_ability/ability): Neurotraumal Rod — Whenever a creature attacks you, it gets -1/-0 until end of turn.
     1  "Genestealer's Kiss"  e.g. [40k 2022 | Creature — Tyranid] Genestealer Patriarch #0:0 (triggered_ability/ability): Genestealer's Kiss — Whenever this creature attacks, put an infection counter on target creature defending player controls.
     1  'Children of the Cult'  e.g. [40k 2022 | Creature — Tyranid] Genestealer Patriarch #0:1 (triggered_ability/ability): Children of the Cult — Whenever a creature with an infection counter on it dies, you create a token that's a copy of that creature, except it's a Tyra
     1  'Repair Barge'  e.g. [40k 2022 | Artifact — Vehicle] Ghost Ark #0:1 (triggered_ability/ability): Repair Barge — Whenever this Vehicle becomes crewed, each artifact creature card in your graveyard gains unearth {3} until end of turn.
     1  'Three Autostubs'  e.g. [40k 2022 | Legendary Creature — Tyranid H] Ghyrson Starn, Kelermorph #0:1 (triggered_ability/ability): Three Autostubs — Whenever another source you control deals exactly 1 damage to a permanent or player, Ghyrson Starn deals 2 damage to that permanent 
     1  'Stowage'  e.g. [40k 2022 | Artifact — Vehicle] Goliath Truck #0:0 (triggered_ability/ability): Stowage — Whenever this Vehicle attacks, put two +1/+1 counters on another target attacking creature.
     1  'Reverberating Summons'  e.g. [40k 2022 | Creature — Demon] Great Unclean One #0:0 (triggered_ability/ability): Reverberating Summons — At the beginning of your end step, each opponent loses 2 life. Then for each opponent who has less life than you, create a 1/3
     1  'Rites of Banishment'  e.g. [40k 2022 | Creature — Astartes Knight] Grey Knight Paragon #0:1 (triggered_ability/ability): Rites of Banishment — When this creature enters, destroy target attacking creature. If that creature is a Demon, exile it instead.
     1  'Rapacious Hunger'  e.g. [40k 2022 | Creature — Tyranid] Haruspex #0:0 (triggered_ability/ability): Rapacious Hunger — Whenever another creature dies, put a +1/+1 counter on this creature.
     1  'Devouring Monster'  e.g. [40k 2022 | Creature — Tyranid] Haruspex #0:1 (activated_ability/ability): Devouring Monster — {T}, Remove X +1/+1 counters from this creature: Add X mana of any one color.
     1  'Sarcophagus'  e.g. [40k 2022 | Artifact Creature — Astartes D] Helbrute #0:1 (spell_or_static_text/ability): Sarcophagus — You may cast this card from your graveyard by exiling another creature card from your graveyard in addition to paying its other costs.
     1  'Locus of Slaanesh'  e.g. [40k 2022 | Creature — Demon] Herald of Slaanesh #0:0 (spell_or_static_text/ability): Locus of Slaanesh — Demon spells you cast cost {2} less to cast.
     1  'Multi-threat Eliminator'  e.g. [40k 2022 | Artifact Creature — Necron] Hexmark Destroyer #0:0 (spell_or_static_text/ability): Multi-threat Eliminator — This creature can't be blocked except by six or more creatures.
     1  'Frenzied Metabolism'
     1  'Titanic'
     1  'Endless Swarm'  e.g. [40k 2022 | Creature — Tyranid] Hormagaunt Horde #0:1 (triggered_ability/ability): Endless Swarm — Whenever a land you control enters, you may pay {2}{G}. If you do, return this card from your graveyard to your hand.
     1  'Secrets of the Soul'  e.g. [40k 2022 | Legendary Artifact Creature — ] Illuminor Szeras #0:0 (activated_ability/ability): Secrets of the Soul — {T}, Sacrifice another creature: Add an amount of {B} equal to the sacrificed creature's mana value.
     1  'Phaeron'  e.g. [40k 2022 | Legendary Artifact Creature — ] Imotekh the Stormlord #0:0 (triggered_ability/ability): Phaeron — Whenever one or more artifact cards leave your graveyard, create two 2/2 black Necron Warrior artifact creature tokens.
     1  'Grand Strategist'  e.g. [40k 2022 | Legendary Artifact Creature — ] Imotekh the Stormlord #0:1 (triggered_ability/ability): Grand Strategist — At the beginning of combat on your turn, another target artifact creature you control gets +2/+2 and gains menace until end of turn
     1  'Unquestionable Wisdom'  e.g. [40k 2022 | Legendary Creature — Human Inq] Inquisitor Greyfax #0:1 (spell_or_static_text/ability): Unquestionable Wisdom — Other creatures you control get +1/+0 and have vigilance.
     1  'Hunt for Heresy'  e.g. [40k 2022 | Legendary Creature — Human Inq] Inquisitor Greyfax #0:2 (activated_ability/ability): Hunt for Heresy — {1}, {T}: Tap target creature an opponent controls. Investigate.
     1  'Inquisition Agents'  e.g. [40k 2022 | Artifact — Equipment] Inquisitorial Rosette #0:0 (triggered_ability/ability): Inquisition Agents — Whenever equipped creature attacks, create a 2/2 white Astartes Warrior creature token with vigilance that's attacking. Then atta
     1  'Symphony of Pain'  e.g. [40k 2022 | Creature — Demon] Keeper of Secrets #0:2 (triggered_ability/ability): Symphony of Pain — Whenever you cast a spell from anywhere other than your hand, this creature deals damage equal to that spell's mana value to target
     1  'Berzerker'  e.g. [40k 2022 | Legendary Creature — Astartes ] Khârn the Betrayer #0:0 (spell_or_static_text/ability): Berzerker — Khârn the Betrayer attacks or blocks each combat if able.
     1  'Sigil of Corruption'  e.g. [40k 2022 | Legendary Creature — Astartes ] Khârn the Betrayer #0:1 (triggered_ability/ability): Sigil of Corruption — When you lose control of Khârn the Betrayer, draw two cards.
     1  'The Betrayer'  e.g. [40k 2022 | Legendary Creature — Astartes ] Khârn the Betrayer #0:2 (prevention_effect/ability): The Betrayer — If damage would be dealt to Khârn the Betrayer, prevent that damage and an opponent of your choice gains control of it.
     1  'Rapid-fire Battle Cannon'  e.g. [40k 2022 | Artifact — Vehicle] Knight Paladin #0:1 (triggered_ability/ability): Rapid-fire Battle Cannon — When this Vehicle enters, it deals 4 damage to each opponent.
     1  'Frenzied Rampage'  e.g. [40k 2022 | Artifact Creature — Knight] Knight Rampager #0:1 (triggered_ability/ability): Frenzied Rampage — At the beginning of combat on your turn, choose an opponent at random. This creature attacks that player this combat if able.
     1  'Pheromone Trail'  e.g. [40k 2022 | Creature — Tyranid] Lictor #0:1 (triggered_ability/ability): Pheromone Trail — When this creature enters, if a creature entered the battlefield under an opponent's control this turn, create a 3/3 green Tyranid W
     1  'Enmitic Exterminator'
     1  'Architect of Deception'  e.g. [40k 2022 | Creature — Demon] Lord of Change #0:2 (triggered_ability/ability): Architect of Deception — When this creature enters, draw three cards.
     1  'Armour of Shrieking Souls'
     1  'Guardian Protocols'  e.g. [40k 2022 | Artifact Creature — Necron] Lychguard #0:0 (activated_ability/ability): Guardian Protocols — {3}{B}, Sacrifice this creature: Return all legendary creature cards from your graveyard to your hand.
     1  'Unearthly Power'  e.g. [40k 2022 | Legendary Creature — Demon Pri] Magnus the Red #0:1 (spell_or_static_text/ability): Unearthly Power — Instant and sorcery spells you cast cost {1} less to cast for each creature token you control.
     1  'Blade of Magnus'  e.g. [40k 2022 | Legendary Creature — Demon Pri] Magnus the Red #0:2 (triggered_ability/ability): Blade of Magnus — Whenever Magnus the Red deals combat damage to a player, create a 3/3 red Spawn creature token.
     1  'Spiritual Leader'  e.g. [40k 2022 | Legendary Creature — Human Tyr] Magus Lucea Kane #0:0 (triggered_ability/ability): Spiritual Leader — At the beginning of combat on your turn, put a +1/+1 counter on target creature.
     1  'Psychic Stimulus'  e.g. [40k 2022 | Legendary Creature — Human Tyr] Magus Lucea Kane #0:1 (activated_ability/ability): Psychic Stimulus — {T}: Add {C}{C}.
     1  'Scavenge the Dead'  e.g. [40k 2022 | Creature — Tyranid] Malanthrope #0:1 (triggered_ability/ability): Scavenge the Dead — When this creature enters, exile target player's graveyard. Put a +1/+1 counter on this creature for each creature card exiled thi
     1  'Master Tactician'
     1  'Chapter Master'
     1  'Terror from the Deep'  e.g. [40k 2022 | Creature — Tyranid] Mawloc #0:1 (triggered_ability/ability): Terror from the Deep — When this creature enters, it fights up to one target creature an opponent controls. If that creature would die this turn, exil
     1  'Primarch of the Death Guard'  e.g. [40k 2022 | Legendary Creature — Demon Pri] Mortarion, Daemon Primarch #0:1 (triggered_ability/ability): Primarch of the Death Guard — At the beginning of your end step, you may pay {X}. If you do, create X 2/2 black Astartes Warrior creature tokens with 
     1  'Warp Vortex'  e.g. [40k 2022 | Creature — Mutant Beast] Mutalith Vortex Beast #0:1 (triggered_ability/ability): Warp Vortex — When this creature enters, flip a coin for each opponent you have. For each flip you win, draw a card. For each flip you lose, this crea
     1  'Synaptic Disintegrator'  e.g. [40k 2022 | Artifact Creature — Necron] Necron Deathmark #0:1 (triggered_ability/ability): Synaptic Disintegrator — When this creature enters, destroy up to one target creature and target player mills three cards.
     1  'Eternity Gate'  e.g. [40k 2022 | Artifact — Vehicle] Necron Monolith #0:2 (triggered_ability/ability): Eternity Gate — Whenever this Vehicle attacks, mill three cards. For each creature card milled this way, create a 2/2 black Necron Warrior artifact cr
     1  'Relentless March'  e.g. [40k 2022 | Artifact Creature — Necron Nob] Necron Overlord #0:0 (activated_ability/ability): Relentless March — {X}, {T}, Tap X untapped artifacts you control: Target opponent loses X life.
     1  'Strategic Coordinator'  e.g. [40k 2022 | Creature — Human Tyranid Advis] Nexos #0:0 (spell_or_static_text/ability): Strategic Coordinator — Basic lands you control have "{T}: Add {C}{C}. Spend this mana only on costs that contain {X}."
     1  'Rogue Trader'  e.g. [40k 2022 | Legendary Creature — Human Rog] Neyam Shai Murad #0:0 (triggered_ability/ability): Rogue Trader — Whenever Neyam Shai Murad deals combat damage to a player, you may have that player return target permanent card from their graveyard t
     1  'Invasion Beams'  e.g. [40k 2022 | Artifact — Vehicle] Night Scythe #0:1 (triggered_ability/ability): Invasion Beams — When this Vehicle enters, create a 2/2 black Necron Warrior artifact creature token.
     1  'Sonic Blaster'  e.g. [40k 2022 | Creature — Astartes Warrior] Noise Marine #0:1 (triggered_ability/ability): Sonic Blaster — When this creature enters, it deals damage equal to the number of spells you've cast this turn to any target.
     1  'Fast Healing'  e.g. [40k 2022 | Legendary Creature — Tyranid] Old One Eye #0:3 (triggered_ability/ability): Fast Healing — At the beginning of your first main phase, you may discard two cards. If you do, return this card from your graveyard to your hand.
     1  'Coruscating Flames'  e.g. [40k 2022 | Creature — Demon Horror] Pink Horror #0:0 (triggered_ability/ability): Coruscating Flames — Whenever you cast an instant or sorcery spell, this creature deals 2 damage to any target.
     1  'Rot Fly'  e.g. [40k 2022 | Creature — Demon] Plague Drone #0:1 (replacement_effect/ability): Rot Fly — If an opponent would gain life, that player loses that much life instead.
     1  'Dynastic Advisor'  e.g. [40k 2022 | Artifact Creature — Necron Wiz] Plasmancer #0:1 (triggered_ability/ability): Dynastic Advisor — When this creature enters, search your library for a basic Swamp card, reveal it, put it into your hand, then shuffle.
     1  'Curse of the Walking Pox'  e.g. [40k 2022 | Creature — Zombie] Poxwalkers #0:1 (triggered_ability/ability): Curse of the Walking Pox — Whenever you cast a spell from anywhere other than your hand, return this card from your graveyard to the battlefield tappe
     1  'Rosarius'  e.g. [40k 2022 | Creature — Astartes Cleric] Primaris Chaplain #0:1 (triggered_ability/ability): Rosarius — Whenever this creature attacks, it gains indestructible until end of turn.
     1  'Harbinger of Despair'  e.g. [40k 2022 | Artifact Creature — Necron Wiz] Psychomancer #0:1 (triggered_ability/ability): Harbinger of Despair — Whenever this creature or another nontoken artifact you control is put into a graveyard from the battlefield or is put into exi
     1  'Vanguard Species'  e.g. [40k 2022 | Creature — Tyranid] Purestrain Genestealer #0:1 (triggered_ability/ability): Vanguard Species — Whenever this creature attacks, you may remove a +1/+1 counter from it. If you do, search your library for a basic land card, put i
     1  'Void Shields'  e.g. [40k 2022 | Artifact — Vehicle] Reaver Titan #0:0 (keyword_ability/ability): Void Shields — Protection from mana value 3 or less
     1  'Gatling Blaster'  e.g. [40k 2022 | Artifact — Vehicle] Reaver Titan #0:1 (triggered_ability/ability): Gatling Blaster — Whenever this Vehicle attacks, it deals 5 damage to each opponent.
     1  'Fallen Warrior'  e.g. [40k 2022 | Artifact Creature — Astartes D] Redemptor Dreadnought #0:0 (additional_cost/ability): Fallen Warrior — As an additional cost to cast this spell, you may exile a creature card from your graveyard.
     1  'Plasma Incinerator'  e.g. [40k 2022 | Artifact Creature — Astartes D] Redemptor Dreadnought #0:2 (triggered_ability/ability): Plasma Incinerator — Whenever this creature attacks, if a card is exiled with it, it gets +X/+X until end of turn, where X is the power of the exiled 
     1  'Phalanx Commander'  e.g. [40k 2022 | Artifact Creature — Necron] Royal Warden #0:0 (triggered_ability/ability): Phalanx Commander — When this creature enters, create two tapped 2/2 black Necron Warrior artifact creature tokens.
     1  'Blood Chalice'  e.g. [40k 2022 | Creature — Astartes Cleric] Sanguinary Priest #0:1 (triggered_ability/ability): Blood Chalice — Whenever another creature you control dies, this creature deals 1 damage to any target.
     1  'Elite Troops'  e.g. [40k 2022 | Artifact Creature — Necron] Sautekh Immortal #0:1 (replacement_effect/ability): Elite Troops — This creature enters with a +1/+1 counter on it for each creature that died this turn.
     1  'Bio-Plasmic Scream'
     1  'Allure of Slaanesh'  e.g. [40k 2022 | Creature — Demon] Seeker of Slaanesh #0:1 (spell_or_static_text/ability): Allure of Slaanesh — Each opponent must attack with at least one creature each combat if able.
     1  'Drain Life'  e.g. [40k 2022 | Creature — C'tan] Shard of the Nightbringer #0:1 (triggered_ability/ability): Drain Life — When this creature enters, if you cast it, target opponent loses half their life, rounded up. You gain life equal to the life lost this w
     1  'Spear of the Void Dragon'  e.g. [40k 2022 | Creature — C'tan] Shard of the Void Dragon #0:1 (triggered_ability/ability): Spear of the Void Dragon — Whenever this creature attacks, each opponent sacrifices a nonland permanent of their choice.
     1  'Matter Absorption'  e.g. [40k 2022 | Creature — C'tan] Shard of the Void Dragon #0:2 (triggered_ability/ability): Matter Absorption — Whenever an artifact is put into a graveyard from the battlefield or is put into exile from the battlefield, put two +1/+1 counter
     1  'Benediction of the Omnissiah'  e.g. [40k 2022 | Artifact Creature — Human Sold] Sicarian Infiltrator #0:2 (triggered_ability/ability): Benediction of the Omnissiah — When this creature enters, draw a card.
     1  'Medicus Ministorum'  e.g. [40k 2022 | Creature — Human Cleric] Sister Hospitaller #0:0 (triggered_ability/ability): Medicus Ministorum — When this creature enters, return target creature card from your graveyard to the battlefield. You gain life equal to its mana va
     1  'Psychic Abomination'
     1  'Martyrdom'  e.g. [40k 2022 | Creature — Human Warrior] Sister Repentia #0:0 (triggered_ability/ability): Martyrdom — When this creature dies, you gain 2 life and draw two cards.
     1  'Hyperphase Threshers'
     1  'Command Protocols'  e.g. [40k 2022 | Artifact Creature — Necron Nob] Skorpekh Lord #0:1 (spell_or_static_text/ability): Command Protocols — Other artifact creatures you control get +1/+0 and have menace.
     1  'Jolly Gutpipes'  e.g. [40k 2022 | Creature — Demon] Sloppity Bilepiper #0:0 (activated_ability/ability): Jolly Gutpipes — {2}, {T}, Sacrifice a creature: The next creature spell you cast this turn has cascade.
     1  'Grav-cannon'
     1  'Concealed Position'  e.g. [40k 2022 | Creature — Astartes Scout] Space Marine Scout #0:2 (triggered_ability/ability): Concealed Position — When this creature enters, if an opponent controls more lands than you, you may search your library for a Plains card, put it ont
     1  'Spore Chimney'  e.g. [40k 2022 | Creature — Tyranid] Sporocyst #0:2 (triggered_ability/ability): Spore Chimney — When this creature enters, search your library for up to X basic land cards, put them onto the battlefield tapped, then shuffle.
     1  'My Will Be Done'  e.g. [40k 2022 | Legendary Artifact Creature — ] Szarekh, the Silent King #0:1 (triggered_ability/ability): My Will Be Done — Whenever Szarekh attacks, mill three cards. You may put an artifact creature card or Vehicle card from among the cards milled this w
     1  'The Seven-fold Chant'  e.g. [40k 2022 | Creature — Astartes Warrior] Tallyman of Nurgle #0:1 (triggered_ability/ability): The Seven-fold Chant — At the beginning of your end step, if a creature died this turn, you draw a card and you lose 1 life. If seven or more creature
     1  'Death Frenzy'  e.g. [40k 2022 | Creature — Tyranid] Termagant Swarm #0:1 (triggered_ability/ability): Death Frenzy — When this creature dies, create a number of 1/1 green Tyranid creature tokens equal to this creature's power.
     1  'Spawn Termagants'  e.g. [40k 2022 | Creature — Tyranid] Tervigon #0:2 (triggered_ability/ability): Spawn Termagants — Whenever this creature deals combat damage to a player, create that many 1/1 green Tyranid creature tokens.
     1  'Arcane Life-support'  e.g. [40k 2022 | Legendary Artifact] The Golden Throne #0:0 (replacement_effect/ability): Arcane Life-support — If you would lose the game, instead exile The Golden Throne and your life total becomes 1.
     1  'A Thousand Souls Die Every Day'  e.g. [40k 2022 | Legendary Artifact] The Golden Throne #0:1 (activated_ability/ability): A Thousand Souls Die Every Day — {T}, Sacrifice a creature: Add three mana in any combination of colors.
     1  'Advanced Species'  e.g. [40k 2022 | Legendary Creature — Tyranid] The Red Terror #0:0 (triggered_ability/ability): Advanced Species — Whenever a red source you control deals damage to one or more permanents and/or players, put a +1/+1 counter on The Red Terror.
     1  'Rapid Regeneration'  e.g. [40k 2022 | Legendary Creature — Tyranid] The Swarmlord #0:0 (replacement_effect/ability): Rapid Regeneration — The Swarmlord enters with two +1/+1 counters on it for each time you've cast your commander from the command zone this game.
     1  'Xenos Cunning'  e.g. [40k 2022 | Legendary Creature — Tyranid] The Swarmlord #0:1 (triggered_ability/ability): Xenos Cunning — Whenever a creature you control with a counter on it dies, draw a card.
     1  'Crushing Teeth'  e.g. [40k 2022 | Creature — Astartes Warrior] Thunderwolf Cavalry #0:1 (triggered_ability/ability): Crushing Teeth — Whenever this creature deals combat damage to a player, put a +1/+1 counter on each other creature you control.
     1  'Hypertoxic Miasma'
     1  'Prismatic Gallery'  e.g. [40k 2022 | Legendary Artifact Creature — ] Trazyn the Infinite #0:1 (spell_or_static_text/ability): Prismatic Gallery — As long as Trazyn is on the battlefield, it has all activated abilities of all artifact cards in your graveyard.
     1  'Dynastic Codes'  e.g. [40k 2022 | Artifact Creature — Necron] Triarch Praetorian #0:1 (triggered_ability/ability): Dynastic Codes — When this creature enters from a graveyard, you draw two cards and you lose 2 life.
     1  'Targeting Relay'  e.g. [40k 2022 | Artifact Creature — Necron] Triarch Stalker #0:0 (triggered_ability/ability): Targeting Relay — At the beginning of combat on your turn, choose an opponent.
     1  'Praesidium Protectiva'  e.g. [40k 2022 | Creature — Human Warrior] Triumph of Saint Katherine #0:1 (triggered_ability/ability): Praesidium Protectiva — When this creature is put into your graveyard from the battlefield, exile it and the top six cards of your library in a face-d
     1  'Subterranean Assault'  e.g. [40k 2022 | Creature — Tyranid] Trygon Prime #0:0 (triggered_ability/ability): Subterranean Assault — Whenever this creature attacks, put a +1/+1 counter on it and a +1/+1 counter on up to one other target attacking creature. Tha
     1  'Shrieking Gargoyles'  e.g. [40k 2022 | Creature — Tyranid] Tyranid Harridan #0:2 (triggered_ability/ability): Shrieking Gargoyles — Whenever this creature or another Tyranid you control deals combat damage to a player, create a 1/1 blue Tyranid Gargoyle creatu
     1  'Synapse Creature'  e.g. [40k 2022 | Creature — Tyranid] Tyranid Prime #0:1 (spell_or_static_text/ability): Synapse Creature — Other creatures you control have evolve.
     1  'Shieldwall'  e.g. [40k 2022 | Creature — Tyranid] Tyrant Guard #0:1 (activated_ability/ability): Shieldwall — Sacrifice this creature: Creatures you control with counters on them gain hexproof and indestructible until end of turn.
     1  'Sorcerous Elixir'  e.g. [40k 2022 | Creature — Mutant Shaman] Tzaangor Shaman #0:1 (triggered_ability/ability): Sorcerous Elixir — Whenever this creature deals combat damage to a player, copy the next instant or sorcery spell you cast this turn when you cast it.
     1  'Suppressing Fire'  e.g. [40k 2022 | Creature — Astartes Warrior] Vanguard Suppressor #0:2 (triggered_ability/ability): Suppressing Fire — Whenever this creature deals combat damage to a player, draw a card.
     1  'Devourer of Souls'
     1  'Aegis of the Emperor'  e.g. [40k 2022 | Creature — Custodes Warrior] Vexilus Praetor #0:2 (spell_or_static_text/ability): Aegis of the Emperor — Commanders you control have protection from everything.
     1  'The Will of the Hive Mind'  e.g. [40k 2022 | Creature — Tyranid] Winged Hive Tyrant #0:2 (spell_or_static_text/ability): The Will of the Hive Mind — Other creatures you control with counters on them have flying and haste.
     1  'Warp Blast'  e.g. [40k 2022 | Creature — Tyranid] Zoanthrope #0:3 (triggered_ability/ability): Warp Blast — When this creature enters, it deals X damage to any target.
     1  'Crash Land'  e.g. [unf 2022 | Legendary Creature — Human Pil] Captain Rex Nebula #0:1 (triggered_ability/granted): Crash Land — Whenever this Vehicle deals damage, roll a six-sided die. If the result is equal to this Vehicle's mana value, sacrifice this Vehicle, th
     1  'Gear Up, sponsored by Wizards of the Coast'  e.g. [unf 2022 | Artifact — Equipment] Souvenir T-Shirt #0:0 (replacement_effect/ability): Gear Up, sponsored by Wizards of the Coast — As this Equipment enters, roll two six-sided dice. For each Magic-branded item you're wearing, roll an ad
     1  'Parade!'  e.g. [unf 2022 | Enchantment] Starlight Spectacular #0:0 (triggered_ability/ability): Parade! — At the beginning of combat on your turn, choose creatures you control one at a time until each creature you control has been chosen. Each of
     1  'Suspend N'  e.g. [unk 2025 | Sorcery] 17-Year Cicadas #0:1 (keyword_ability/ability): Suspend 17 — {0}
     1  'Channelstorm'  e.g. [unk 2025 | Land — Barnyard] Blustering Barnyard #0:2 (activated_ability/ability): Channelstorm — {5}, Discard this card: Choose {P} worth of modes equal to the number of spells that have been cast this turn.
     1  'Mirran Victory'  e.g. [unk 2023 | Legendary Creature — Elephant] Delphia, Undecided #0:1 (spell_or_static_text/ability): Mirran Victory — Mirrodin, Darksteel, Fifth Dawn, and Scars of Mirrodin.
     1  'Phyrexian Victory'  e.g. [unk 2023 | Legendary Creature — Elephant] Delphia, Undecided #0:2 (spell_or_static_text/ability): Phyrexian Victory — Mirrodin Besieged, New Phyrexia, All Will be One, and March of the Machine.
     1  'Commander Suspend N'  e.g. [unk 2023 | Legendary Creature — Ooze] Groaaaaag, Hungry Monster #0:0 (keyword_ability/ability): Commander Suspend 4 — {B}{G}
     1  'Beeeeeeep, Beeeeeeep, Beeeeeeep'  e.g. [unk 2023 | Artifact — Vehicle] Huge Truck #0:0 (triggered_ability/ability): Beeeeeeep, Beeeeeeep, Beeeeeeep — Whenever another creature you control becomes the target of a backup ability, Huge Truck permanently gains all of th
     1  'Corrupted Metalcraft'  e.g. [unk 2023 | Artifact Creature — Phyrexian ] Incisor Steed #0:1 (spell_or_static_text/ability): Corrupted Metalcraft — As long as you control three or more artifacts and an opponent has three or more poison counters, Incisor Steed gets +3/+0.
     1  'Landerfall'  e.g. [unk 2025 | Legendary Artifact Creature — ] Lander Rizzi #0:1 (triggered_ability/ability): Landerfall — Whenever a lander you control enters, put a +1/+1 counter on Lander.
     1  'From Downtown'  e.g. [unk 2025 | Legendary Creature — Ox Athlet] Mijo, the Bull #0:1 (replacement_effect/ability): From Downtown — If a source you control would deal exactly 2 damage to a permanent or player, it deals 3 damage instead.
     1  'Scald'  e.g. [unk 2026 | Legendary Creature — Human Adv] The Bearded Teakeeper #0:2 (activated_ability/granted): Scald — {2}, {T}, Sacrifice this artifact: It deals 2 damage to target opponent.
     1  'Rule Zero'  e.g. [unk 2024 | Legendary Creature — Human Wiz] The Clever Magician #0:0 (triggered_ability/ability): Rule Zero — Whenever this creature enters the battlefield, and before the game begins if this creature is your commander, you may propose a new rule t
     1  'Old Companion'  e.g. [unk 2023 | Legendary Creature — Beast Nob] The Companion of the Wilds #0:0 (spell_or_static_text/ability): Old Companion — Your starting deck contains only cards from WOE, WOC, and playtest cards.
     1  'Grumpy Co-play'  e.g. [unk 2024 | Legendary Creature — Dinosaur ] The Egotistical Velociraptor #0:0 (triggered_ability/ability): Grumpy Co-play — When The Egotistical Velociraptor enters the battlefield, if you cast it, you may choose another Magic player not currently in a game
     1  'I Denial'  e.g. [unk 2026 | Enchantment — Saga] The Five Stages of Grief #0:1 (spell_or_static_text/ability): I Denial — Until your next turn, noncreature spells cost {2} more to cast.
     1  'II Anger'  e.g. [unk 2026 | Enchantment — Saga] The Five Stages of Grief #0:2 (spell_or_static_text/ability): II Anger — Goad target creature.
     1  'III Bargaining'  e.g. [unk 2026 | Enchantment — Saga] The Five Stages of Grief #0:3 (spell_or_static_text/ability): III Bargaining — Each player sacrifices an artifact, enchantment, or token.
     1  'IV Depression'  e.g. [unk 2026 | Enchantment — Saga] The Five Stages of Grief #0:4 (spell_or_static_text/ability): IV Depression — Target creature loses all abilities until your next turn. Create a Food token.
     1  'V Acceptance'  e.g. [unk 2026 | Enchantment — Saga] The Five Stages of Grief #0:5 (spell_or_static_text/ability): V Acceptance — The next spell you cast has epic.
     1  'Establishing Shot'  e.g. [unk 2026 | Legendary Creature — Human Gam] The Good Gamers #0:0 (triggered_ability/ability): Establishing Shot — At the beginning of your first upkeep, if The Good Gamers is in the command zone, it deals 2 damage to each opponent.
     1  'Ew-minance'  e.g. [unk 2024 | Legendary Creature — Slug Game] The Gunky Runner #0:0 (triggered_ability/ability): Ew-minance — When The Gunky Runner enters and at the beginning of the first upkeep in a game where it's your commander, shuffle three Gunk cards into 
     1  'Fixed commander ninjutsu'  e.g. [unk 2025 | Legendary Creature — Phyrexian] The Multifaceted Phyrexian #0:2 (keyword_ability/ability): Fixed commander ninjutsu — {B}{B}, Discard a card
     1  'Advertising'  e.g. [unk 2025 | Legendary Artifact] The Mysterious Sphere #0:0 (activated_ability/ability): Advertising — {T}: Exile the top card of your library. You gain 1 life.
     1  'Show'  e.g. [unk 2025 | Legendary Artifact] The Mysterious Sphere #0:1 (triggered_ability/ability): Show — At the beginning of combat on your turn, if there are three or more cards exiled with The Mysterious Sphere, you may put them into your graveya
     1  'Max Speed'  e.g. [unk 2025 | Land] The Mystery Raceway #0:2 (activated_ability/ability): Max Speed — {W}{U}{B}{R}{G}, {T}: Mill seven cards. Choose a playtest card from among them. You may cast it without paying its mana cost. If Team Clou
     1  'Spring'  e.g. [unk 2026 | Legendary Creature — God] The Underworld Fantasy Queen #0:2 (triggered_ability/ability): Spring — Whenever Her Majesty enters or attacks, you may sacrifice another creature. If you do, draw a card.
     1  'Summer'  e.g. [unk 2026 | Legendary Creature — God] The Underworld Fantasy Queen #0:3 (triggered_ability/ability): Summer — Whenever a creature an opponent controls dies, put a +1/+1 counter on Her Majesty.
     1  'Fall'  e.g. [unk 2026 | Legendary Creature — God] The Underworld Fantasy Queen #0:4 (triggered_ability/ability): Fall — Whenever another creature dies, target opponent loses 1 life and you gain 1 life.
     1  'Winter'  e.g. [unk 2026 | Legendary Creature — God] The Underworld Fantasy Queen #0:5 (spell_or_static_text/ability): Winter — Her Majesty can't attack or block.
     1  'Judge Call!'  e.g. [unk 2024 | Legendary Creature — Elder Sab] The Wise Sable #0:0 (triggered_ability/ability): Judge Call! — When The Wise Sable enters the battlefield, choose one of the following six cards at random: Blood Moon; Enraging Licid; Humility; Life 
     1  'Nitro-N'  e.g. [who 2023 | Legendary Creature — Human Reb] Ace, Fearless Rebel #0:0 (triggered_ability/ability): Nitro-9 — Whenever Ace attacks, you may sacrifice an artifact.
     1  'Ultimate Sacrifice'  e.g. [who 2023 | Legendary Creature — Human Art] Adric, Mathematical Genius #0:1 (activated_ability/ability): Ultimate Sacrifice — {1}{U}, Sacrifice Adric: Counter target activated or triggered ability.
     1  'Byzantium Radiation'  e.g. [who 2023 | Plane — Alfava Metraxis] Aplan Mortarium #0:0 (triggered_ability/ability): Byzantium Radiation — At the beginning of your upkeep, put an exposure counter on Aplan Mortarium. Then you lose life equal to the number of exposure 
     1  'Brand-new Sky'
     1  'History Teacher'  e.g. [who 2023 | Legendary Creature — Human Adv] Barbara Wright #0:0 (spell_or_static_text/ability): History Teacher — Sagas you control have read ahead.
     1  'Impossible Girl'  e.g. [who 2023 | Legendary Creature — Human Adv] Clara Oswald #0:0 (spell_or_static_text/ability): Impossible Girl — If Clara Oswald is your commander, choose a color before the game begins. Clara Oswald is the chosen color.
     1  'Exterminate!'  e.g. [who 2023 | Artifact Creature — Dalek] Dalek Drone #0:2 (triggered_ability/ability): Exterminate! — When this creature enters, destroy target creature an opponent controls. That player loses 3 life.
     1  'The Most Important Punch in History'  e.g. [who 2023 | Legendary Creature — Human Det] Duggan, Private Detective #0:2 (activated_ability/ability): The Most Important Punch in History — {1}{G}, {T}: Duggan deals damage equal to twice its power to another target creature. Activate only once.
     1  'Each opponent faces a villainous choice'  e.g. [who 2023 | Sorcery] Ensnared by the Mara #0:0 (spell_or_static_text/ability): Each opponent faces a villainous choice — They exile cards from the top of their library until they exile a nonland card, then you may cast that card 
     1  'Suspended Animation'  e.g. [who 2023 | Plane — Necros] Gardens of Tranquil Repose #0:0 (triggered_ability/ability): Suspended Animation — Whenever a creature dies, exile it. Its controller scries 1.
     1  'Praise Him'  e.g. [who 2023 | Plane — Spacecraft] Hotel of Fears #0:1 (triggered_ability/ability): Praise Him — Whenever chaos ensues, choose a color. Put X +1/+1 counters on target creature you control, where X is your devotion to that color. Then 
     1  'Science Teacher'  e.g. [who 2023 | Legendary Creature — Human Sci] Ian Chesterton #0:0 (spell_or_static_text/ability): Science Teacher — Each Saga spell you cast has replicate. The replicate cost is equal to its mana cost.
     1  'Negative'  e.g. [who 2023 | Legendary Artifact Creature — ] K-9, Mark I #0:0 (spell_or_static_text/ability): Negative — As long as K-9 is untapped, other legendary creatures you control have ward {1}.
     1  'Affirmative'  e.g. [who 2023 | Legendary Artifact Creature — ] K-9, Mark I #0:1 (activated_ability/ability): Affirmative — {1}{U}, {T}: Target legendary creature can't be blocked this turn.
     1  'Still Point in Time'  e.g. [who 2023 | Plane — Earth] Lake Silencio #0:0 (spell_or_static_text/ability): Still Point in Time — All spells have split second.
     1  'Woman Who Walked the Earth'  e.g. [who 2023 | Legendary Creature — Human Cle] Martha Jones #0:0 (triggered_ability/ability): Woman Who Walked the Earth — When Martha Jones enters, investigate.
     1  'Midnight Entity'  e.g. [who 2023 | Artifact — Vehicle] Midnight Crusader Shuttle #0:0 (triggered_ability/ability): Midnight Entity — Whenever this Vehicle attacks, defending player faces a villainous choice — That player sacrifices a creature of their choice, or yo
     1  'Sonic Booster'  e.g. [who 2023 | Legendary Creature — Human Sci] Nyssa of Traken #0:1 (triggered_ability/ability): Sonic Booster — Whenever Nyssa of Traken attacks, sacrifice any number of artifacts.
     1  'Song of the Ood'  e.g. [who 2023 | Plane — Horsehead Nebula] Ood Sphere #0:0 (spell_or_static_text/ability): Song of the Ood — Noncreature spells have convoke.
     1  'Red-Eye'  e.g. [who 2023 | Plane — Horsehead Nebula] Ood Sphere #0:1 (triggered_ability/ability): Red-Eye — Whenever chaos ensues, for each opponent, goad up to one target creature that opponent controls. Until your next turn, those creatures can't
     1  'Meet in Reverse'  e.g. [who 2023 | Legendary Creature — Human Tim] River Song #0:0 (spell_or_static_text/ability): Meet in Reverse — You draw cards from the bottom of your library rather than the top.
     1  'Spoilers'  e.g. [who 2023 | Legendary Creature — Human Tim] River Song #0:1 (triggered_ability/ability): Spoilers — Whenever an opponent scries, surveils, or searches their library, put a +1/+1 counter on River Song. Then River Song deals damage to that p
     1  'The Last Centurion'  e.g. [who 2023 | Legendary Creature — Human Sol] Rory Williams #0:3 (triggered_ability/ability): The Last Centurion — When you cast this spell from anywhere other than exile, exile it with three time counters on it. It gains suspend. Then investig
     1  'Bad Wolf'  e.g. [who 2023 | Legendary Creature — Human] Rose Tyler #0:1 (triggered_ability/ability): Bad Wolf — Whenever Rose Tyler attacks, put a time counter on it for each suspended card you own and each other permanent you control with a time coun
     1  'Share Intelligence'  e.g. [who 2023 | Legendary Creature — Human Sol] Sergeant John Benton #0:2 (triggered_ability/ability): Share Intelligence — Whenever Sergeant John Benton deals combat damage to a player, you and that player each draw that many cards.
     1  'Temporal Foresight'  e.g. [who 2023 | Creature — Human Warlock] Sibylline Soothsayer #0:0 (triggered_ability/ability): Temporal Foresight — When this creature enters, reveal cards from the top of your library until you reveal a nonland card with mana value 3 or greater
     1  'Grenades!'
     1  'Glory of Battle'
     1  'Sanctified Rules of Combat'  e.g. [who 2023 | Creature — Alien Soldier] Sycorax Commander #0:2 (triggered_ability/ability): Sanctified Rules of Combat — When this creature enters, each opponent faces a villainous choice — That opponent discards all the cards in their hand, 
     1  'Brave Heart'  e.g. [who 2023 | Legendary Creature — Human] Tegan Jovanka #0:0 (triggered_ability/ability): Brave Heart — Whenever you attack, target attacking historic creature gets +1/+1 and gains indestructible until end of turn.
     1  'Sixty-Six Seconds'  e.g. [who 2023 | Plane — Spacecraft] The Dining Car #0:1 (triggered_ability/ability): Sixty-Six Seconds — At the beginning of your upkeep, sacrifice a creature with the least toughness among creatures you control. Then investigate.
     1  'Peaceful Coexistence'  e.g. [who 2023 | Legendary Creature — Time Lord] The Fifth Doctor #0:0 (triggered_ability/ability): Peaceful Coexistence — At the beginning of your end step, put a +1/+1 counter on each creature you control that didn't attack or enter this turn. Unta
     1  'Water Always Wins'  e.g. [who 2023 | Creature — Alien Zombie Horror] The Flood of Mars #0:1 (triggered_ability/ability): Water Always Wins — Whenever this creature attacks, put a flood counter on another target creature or land. If it's a creature, it becomes a copy of t
     1  'Make Them Pay'  e.g. [who 2023 | Legendary Creature — Time Lord] The Master, Gallifrey's End #0:0 (triggered_ability/ability): Make Them Pay — Whenever a nontoken artifact creature you control dies, you may exile it. If you do, choose an opponent with the most life among your 
     1  'Low Gravity'  e.g. [who 2023 | Plane — Moon] The Moonbase #0:0 (spell_or_static_text/ability): Low Gravity — All creatures have "{2}: This creature gains flying until end of turn. Activate only as a sorcery."
     1  'Into the TARDIS'  e.g. [who 2023 | Legendary Creature — Time Lord] The Ninth Doctor #0:1 (triggered_ability/ability): Into the TARDIS — Whenever The Ninth Doctor becomes untapped during your untap step, you get an additional upkeep step after this step.
     1  'How Civil of You'  e.g. [who 2023 | Legendary Creature — Time Lord] The Second Doctor #0:1 (triggered_ability/ability): How Civil of You — At the beginning of your end step, each player may draw a card. Each opponent who does can't attack you or permanents you control d
     1  "Time Lord's Prerogative"  e.g. [who 2023 | Legendary Creature — Time Lord] The Sixth Doctor #0:0 (triggered_ability/ability): Time Lord's Prerogative — Whenever you cast a historic spell, copy it, except the copy isn't legendary. This ability triggers only once each turn.
     1  'Allons-y!'  e.g. [who 2023 | Legendary Creature — Time Lord] The Tenth Doctor #0:0 (triggered_ability/ability): Allons-y! — Whenever you attack, exile cards from the top of your library until you exile a nonland card. Put three time counters on it. If it doesn't
     1  'Timey-Wimey'  e.g. [who 2023 | Legendary Creature — Time Lord] The Tenth Doctor #0:1 (activated_ability/ability): Timey-Wimey — {7}: Time travel three times. Activate only as a sorcery.
     1  'Team TARDIS'  e.g. [who 2023 | Legendary Creature — Time Lord] The Thirteenth Doctor #0:1 (triggered_ability/ability): Team TARDIS — At the beginning of your end step, untap each creature you control with a counter on it.
     1  'Bear Witness'  e.g. [who 2023 | Creature — Alien Cleric] Thijarian Witness #0:1 (triggered_ability/ability): Bear Witness — Whenever another creature dies, if it was attacking or blocking alone, exile it and investigate.
     1  'Parallel Universe'  e.g. [who 2023 | Creature — Alien Insect] Time Beetle #0:1 (triggered_ability/ability): Parallel Universe — Whenever this creature deals combat damage to a player, time travel.
     1  'Consume Anomaly'  e.g. [who 2023 | Creature — Alien Horror] Time Reaper #0:2 (triggered_ability/ability): Consume Anomaly — Whenever this creature deals combat damage to a player, put target face-up card they own in exile on the bottom of their library. If
     1  'Deal with the Black Guardian'  e.g. [who 2023 | Legendary Creature — Rogue] Vislor Turlough #0:0 (triggered_ability/ability): Deal with the Black Guardian — When Vislor Turlough enters, you may have an opponent gain control of it. If you do, it's goaded for as long as they co
     1  'Look to the Stars'  e.g. [who 2023 | Legendary Creature — Human Sol] Wilfred Mott #0:0 (triggered_ability/ability): Look to the Stars — At the beginning of your upkeep, put a time counter on Wilfred Mott. Then look at the top X cards of your library, where X is the 
     1  'Body-print'  e.g. [who 2023 | Creature — Alien Shapeshifter ] Zygon Infiltrator #0:0 (activated_ability/ability): Body-print — {2}{U}: Tap another target creature and put a stun counter on it. This creature becomes a copy of that creature for as long as that creat
     1  '♦'  e.g. [punk 2024 | Phenomenon] High and Dry Black Market #0:1 (spell_or_static_text/ability): ♦ — Create a Treasure Token.
     1  '♦ ♦'  e.g. [punk 2024 | Phenomenon] High and Dry Black Market #0:2 (spell_or_static_text/ability): ♦ ♦ — Draw a card.
     1  '♦ ♦ ♦'  e.g. [punk 2024 | Phenomenon] High and Dry Black Market #0:3 (spell_or_static_text/ability): ♦ ♦ ♦ — Create a 3/2 colorless Shapeshifter creature token with Changeling.
     1  'You can never leave'  e.g. [punk 2024 | Plane — Duskmourn] No Way Out #0:1 (replacement_effect/ability): You can never leave — If a player would planeswalk while this plane has dread counters, chaos ensues instead.
     1  'A Murder at Markov Manor'  e.g. [punk 2025 | Plane — Innistrad] Sorin's Remastered Manor #0:2 (triggered_ability/ability): A Murder at Markov Manor — Whenever chaos ensues, target spooky creature you control bites target opponent or creature an opponent controls.
     1  'For Auld Lang Syne'  e.g. [pip 2024 | Legendary Creature — Human Doc] Arcade Gannon #0:1 (spell_or_static_text/ability): For Auld Lang Syne — Once during each of your turns, you may cast an artifact or Human spell from your graveyard with mana value less than or equal to
     1  'Tunnel Snakes Rule!'  e.g. [pip 2024 | Legendary Creature — Human Rog] Butch DeLoria, Tunnel Snake #0:1 (triggered_ability/ability): Tunnel Snakes Rule! — Whenever Butch DeLoria attacks, it gets +1/+1 until end of turn for each other Rogue and/or Snake you control.
     1  'One for My Baby'  e.g. [pip 2024 | Legendary Creature — Human Sol] Craig Boone, Novac Guard #0:2 (triggered_ability/ability): One for My Baby — Whenever you attack with two or more creatures, put two quest counters on Craig Boone.
     1  'Hunters for Hire'  e.g. [pip 2024 | Legendary Creature — Human Cit] Duchess, Wayward Tavernkeep #0:0 (triggered_ability/ability): Hunters for Hire — Whenever a creature you control deals combat damage to a player, put a quest counter on it.
     1  'ED-E My Love'  e.g. [pip 2024 | Legendary Artifact Creature — ] ED-E, Lonesome Eyebot #0:1 (triggered_ability/ability): ED-E My Love — Whenever you attack, if the number of attacking creatures is greater than the number of quest counters on ED-E, put a quest counter on 
     1  'Blind Betrayal'  e.g. [pip 2024 | Legendary Creature — Human Kni] Elder Arthur Maxson #0:1 (activated_ability/ability): Blind Betrayal — Sacrifice another creature: Elder Arthur Maxson gains indestructible until end of turn.
     1  'Come Fly With Me'  e.g. [pip 2024 | Legendary Creature — Zombie Mu] Jason Bright, Glowing Prophet #0:1 (activated_ability/ability): Come Fly With Me — {2}, Sacrifice a creature: Put a +1/+1 counter on target creature you control. It gains flying until end of turn.
     1  'Decimate'  e.g. [pip 2024 | Legendary Creature — Human Sol] Legate Lanius, Caesar's Ace #0:0 (triggered_ability/ability): Decimate — When Legate Lanius enters, each opponent sacrifices a tenth of the creatures they control of their choice, rounded up.
     1  'First Contact'  e.g. [pip 2024 | Legendary Creature — Human Adv] Overseer of Vault 76 #0:0 (triggered_ability/ability): First Contact — Whenever Overseer of Vault 76 or another creature you control with power 3 or less enters, put a quest counter on Overseer of Vault 76
     1  'Alluring Eyes'  e.g. [pip 2024 | Legendary Creature — Crab Muta] Red Death, Shipwrecker #0:0 (activated_ability/ability): Alluring Eyes — {T}: Goad target creature an opponent controls. That player draws a card. You add {R}.
     1  'The Nuka-Cola Challenge'  e.g. [pip 2024 | Legendary Creature — Human Cit] Sierra, Nuka's Biggest Fan #0:0 (triggered_ability/ability): The Nuka-Cola Challenge — Whenever one or more creatures you control deal combat damage to a player, put a quest counter on Sierra and create a Food t
     1  'Wild Card'  e.g. [pip 2024 | Legendary Artifact Creature — ] Yes Man, Personal Securitron #0:1 (triggered_ability/ability): Wild Card — When Yes Man leaves the battlefield, its owner creates a tapped 1/1 white Soldier creature token for each quest counter on it.
     1  '+ {M}{M}{M}'  e.g. [otj 2024 | Instant] Final Showdown #0:3 (spell_or_static_text/ability): + {3}{W}{W} — Destroy all creatures.
     1  'Allies'  e.g. [acr 2024 | Legendary Creature — Human Nob] Cleopatra, Exiled Pharaoh #0:0 (triggered_ability/ability): Allies — At the beginning of your end step, put a +1/+1 counter on each of up to two other target legendary creatures.
     1  'Betrayal'  e.g. [acr 2024 | Legendary Creature — Human Nob] Cleopatra, Exiled Pharaoh #0:1 (triggered_ability/ability): Betrayal — Whenever a legendary creature with counters on it dies, draw a card for each counter on it. You lose 2 life.
     1  'Sage Project'  e.g. [acr 2024 | Legendary Creature — God Warri] Havi, the All-Father #0:1 (triggered_ability/ability): Sage Project — Whenever Havi or another legendary creature you control dies, return target legendary creature card with lesser mana value from your gr
     1  'Leap Strike'  e.g. [acr 2024 | Legendary Creature — Human Ass] Shao Jun #0:0 (spell_or_static_text/ability): Leap Strike — During your turn, Shao Jun has flying and first strike.
     1  'Rope Dart'  e.g. [acr 2024 | Legendary Creature — Human Ass] Shao Jun #0:1 (activated_ability/ability): Rope Dart — Tap two untapped artifacts you control: Shao Jun deals 1 damage to each opponent.
     1  'Sokratic Dialogue'
     1  'Those Who Came Before'  e.g. [acr 2024 | Legendary Creature — God Artif] The Capitoline Triad #0:0 (spell_or_static_text/ability): Those Who Came Before — This spell costs {1} less to cast for each historic card in your graveyard.
     1  'Leap of Faith'  e.g. [acr 2024 | Artifact Creature — Wall] Towering Viewpoint #0:2 (activated_ability/ability): Leap of Faith — {3}: Target creature gains flying until end of turn.
     1  'Trade Routes'  e.g. [mb2 2024 | World Enchantment] Alberix, the Trade Planet #0:1 (triggered_ability/ability): Trade Routes — At the beginning of your precombat main phase, choose one —
     1  '☐ Diving Gear'  e.g. [mb2 2024 | Enchantment — Quest] Map to Lorthos's Temple #0:1 (spell_or_static_text/ability): ☐ Diving Gear — An artifact enters the battlefield under your control.
     1  '☐ Merfolk'  e.g. [mb2 2024 | Enchantment — Quest] Map to Lorthos's Temple #0:2 (spell_or_static_text/ability): ☐ Merfolk — A Merfolk enters the battlefield under your control.
     1  '☐ Ritual'  e.g. [mb2 2024 | Enchantment — Quest] Map to Lorthos's Temple #0:3 (spell_or_static_text/ability): ☐ Ritual — You cast an instant or sorcery spell.
     1  'Reward'  e.g. [mb2 2024 | Conspiracy — Secret Mission] Marchesa's Surprise Party #0:5 (spell_or_static_text/ability): Reward — Draw a card.
     1  'Scryfall'
     1  'Raise'  e.g. [fic 2025 | Legendary Creature — Human Cle] Aerith, Last Ancient #0:1 (triggered_ability/ability): Raise — At the beginning of your end step, if you gained life this turn, return target creature card from your graveyard to your hand. If you gained 7
     1  'Dualcast'  e.g. [fic 2025 | Legendary Creature — Elf Wizar] Alisaie Leveilleur #0:2 (spell_or_static_text/ability): Dualcast — The second spell you cast each turn costs {2} less to cast.
     1  'Eukrasia'  e.g. [fic 2025 | Legendary Creature — Elf Wizar] Alphinaud Leveilleur #0:2 (triggered_ability/ability): Eukrasia — Whenever you cast your second spell each turn, draw a card.
     1  'No Mercy'  e.g. [fic 2025 | Legendary Creature — Human Mon] Amarant Coral #0:2 (triggered_ability/ability): No Mercy — Whenever Amarant Coral deals combat damage to an opponent, it deals that much damage to each other opponent.
     1  'Shooting Star'  e.g. [fic 2025 | Legendary Creature — Human Spi] Auron, Venerated Guardian #0:1 (triggered_ability/ability): Shooting Star — Whenever Auron attacks, put a +1/+1 counter on it.
     1  'Pray'  e.g. [fic 2025 | Legendary Creature — Human Cle] Banon, the Returners' Leader #0:0 (spell_or_static_text/ability): Pray — Once during each of your turns, you may cast a creature spell from among cards in your graveyard that were put there from anywhere other than t
     1  'Avalanche!'  e.g. [fic 2025 | Legendary Creature — Human Reb] Barret, Avalanche Leader #0:1 (triggered_ability/ability): Avalanche! — Whenever an Equipment you control enters, create a 2/2 red Rebel creature token.
     1  'Go for the Goal!'  e.g. [fic 2025 | Artifact] Blitzball Stadium #0:1 (activated_ability/ability): Go for the Goal! — {3}, {T}: Until end of turn, target creature gains "Whenever this creature deals combat damage to a player, draw a card for each ki
     1  'Spirit of the Whalaqee'  e.g. [fic 2025 | Artifact — Equipment] Blue Mage's Cane #0:3 (keyword_ability/ability): Spirit of the Whalaqee — Equip {2}
     1  'Lucky Slots'  e.g. [fic 2025 | Legendary Artifact Creature — ] Cait Sith, Fortune Teller #0:0 (triggered_ability/ability): Lucky Slots — At the beginning of combat on your turn, scry 1, then exile the top card of your library. You may play that card this turn.
     1  'Light Party'  e.g. [fic 2025 | Enchantment] Champions from Beyond #0:1 (triggered_ability/ability): Light Party — Whenever you attack with four or more creatures, scry 2, then draw a card.
     1  'Full Party'  e.g. [fic 2025 | Enchantment] Champions from Beyond #0:2 (triggered_ability/ability): Full Party — Whenever you attack with eight or more creatures, those creatures get +4/+4 until end of turn.
     1  'Krishna'  e.g. [fic 2025 | Artifact — Equipment] Dancer's Chakrams #0:3 (keyword_ability/ability): Krishna — Equip {3}
     1  'Tools'  e.g. [fic 2025 | Legendary Creature — Human Art] Edgar, Master Machinist #0:1 (triggered_ability/ability): Tools — Whenever Edgar attacks, it gets +X/+0 until end of turn, where X is the greatest mana value among artifacts you control.
     1  'Throw Wide the Gates'
     1  'Rage'  e.g. [fic 2025 | Legendary Creature — Human Ber] Gau, Feral Youth #0:0 (triggered_ability/ability): Rage — Whenever Gau attacks, put a +1/+1 counter on it.
     1  "Shiva's Aid"  e.g. [fic 2025 | Legendary Creature — Elder Dra] Hraesvelgr of the First Brood #0:3 (triggered_ability/ability): Shiva's Aid — When Hraesvelgr enters and whenever you cast a noncreature spell, target creature gets +1/+0 until end of turn and can't be blocked this
     1  'Ronso Rage'  e.g. [fic 2025 | Legendary Creature — Cat Warri] Kimahri, Valiant Guardian #0:1 (triggered_ability/ability): Ronso Rage — At the beginning of combat on your turn, put a +1/+1 counter on Kimahri and tap target creature an opponent controls. Then you may have K
     1  'Trace Aether'  e.g. [fic 2025 | Legendary Creature — Dwarf Wiz] Krile Baldesion #0:1 (triggered_ability/ability): Trace Aether — Whenever you cast a noncreature spell, you may return target creature card with mana value equal to that spell's mana value from your g
     1  'Mug'  e.g. [fic 2025 | Legendary Creature — Human Rog] Locke, Treasure Hunter #0:1 (triggered_ability/ability): Mug — Whenever Locke attacks, each player mills a card. If a land card was milled this way, create a Treasure token. Until end of turn, you may cast a
     1  'Dance'
     1  'Warp-Strike'  e.g. [fic 2025 | Legendary Creature — Human Nob] Noctis, Heir Apparent #0:1 (activated_ability/ability): Warp-Strike — {3}: Exile Noctis.
     1  'Death Sickle'  e.g. [fic 2025 | Artifact — Equipment] Reaper's Scythe #0:3 (keyword_ability/ability): Death Sickle — Equip {2}
     1  'Cosmo Memory'  e.g. [fic 2025 | Legendary Creature — Beast War] Red XIII, Proud Warrior #0:3 (triggered_ability/ability): Cosmo Memory — When Red XIII enters, return target Aura or Equipment card from your graveyard to your hand.
     1  'Steal'  e.g. [fic 2025 | Legendary Creature — Human Art] Rikku, Resourceful Guardian #0:1 (activated_ability/ability): Steal — {1}, {T}: Move a counter from target creature an opponent controls onto target creature you control. Activate only as a sorcery.
     1  'Jenova Cells'  e.g. [fic 2025 | Legendary Creature — Human Ava] Sephiroth, Fallen Hero #0:0 (triggered_ability/ability): Jenova Cells — Whenever Sephiroth attacks, you may put a cell counter on target creature. Until end of turn, each modified creature you control has ba
     1  'The Reunion'  e.g. [fic 2025 | Legendary Creature — Human Ava] Sephiroth, Fallen Hero #0:1 (activated_ability/ability): The Reunion — {3}, Sacrifice a modified creature: Return this card from your graveyard to the battlefield tapped.
     1  'Throw'  e.g. [fic 2025 | Legendary Creature — Human Ass] Shadow, Mysterious Assassin #0:1 (triggered_ability/ability): Throw — Whenever Shadow deals combat damage to a player, you may sacrifice another nonland permanent. If you do, draw two cards and each opponent lose
     1  'Unlock Ability'
     1  'Sketch and Lore'  e.g. [fic 2025 | Legendary Creature — Human Wiz] Strago and Relm #0:0 (activated_ability/ability): Sketch and Lore — {2}{R}, {T}: Target opponent exiles cards from the top of their library until they exile an instant, sorcery, or creature card. You 
     1  "Scions' Secretary"  e.g. [fic 2025 | Legendary Creature — Dwarf Adv] Tataru Taru #0:1 (triggered_ability/ability): Scions' Secretary — Whenever an opponent draws a card, if it isn't that player's turn, create a tapped Treasure token. This ability triggers only once
     1  'Royal Guard'  e.g. [fic 2025 | Legendary Creature — Human War] Thancred Waters #0:1 (triggered_ability/ability): Royal Guard — When Thancred Waters enters, another target legendary permanent you control gains indestructible for as long as you control Thancred Wat
     1  'Cheer'  e.g. [fic 2025 | Legendary Creature — Human War] Tidus, Yuna's Guardian #0:1 (triggered_ability/ability): Cheer — Whenever one or more creatures you control with counters on them deal combat damage to a player, you may draw a card and proliferate. Do this 
     1  'Draw Arcanum'  e.g. [fic 2025 | Legendary Creature — Elf Advis] Urianger Augurelt #0:1 (activated_ability/ability): Draw Arcanum — {T}: Look at the top card of your library. You may exile it face down.
     1  'Play Arcanum'  e.g. [fic 2025 | Legendary Creature — Elf Advis] Urianger Augurelt #0:2 (activated_ability/ability): Play Arcanum — {T}: Until end of turn, you may play cards exiled with Urianger Augurelt. Spells you cast this way cost {2} less to cast.
     1  'Chaos'  e.g. [fic 2025 | Legendary Creature — Assassin] Vincent, Vengeful Atoner #0:2 (triggered_ability/ability): Chaos — Whenever Vincent deals combat damage to an opponent, it deals that much damage to each other opponent if Vincent's power is 7 or greater.
     1  'Blitzball Captain'
     1  'Grand Summon'  e.g. [fic 2025 | Legendary Creature — Human Cle] Yuna, Grand Summoner #0:0 (activated_ability/ability): Grand Summon — {T}: Add one mana of any color.
     1  'Starscourge'  e.g. [fin 2025 | Legendary Creature — Elder Hum] Ardyn, the Usurper #0:1 (triggered_ability/ability): Starscourge — At the beginning of combat on your turn, exile up to one target creature card from a graveyard. If you exiled a card this way, create a 
     1  'Diana'  e.g. [fin 2025 | Artifact — Equipment] Astrologian's Planisphere #0:3 (keyword_ability/ability): Diana — Equip {2}
     1  "Perseus's Bow"  e.g. [fin 2025 | Artifact — Equipment] Bard's Bow #0:2 (keyword_ability/ability): Perseus's Bow — Equip {6}
     1  'Blow Up'  e.g. [fin 2025 | Creature — Elemental] Blazing Bomb #0:1 (activated_ability/ability): Blow Up — {T}, Sacrifice this creature: It deals damage equal to its power to target creature. Activate only as a sorcery.
     1  'GOOOOAAAALLL!'  e.g. [fin 2025 | Artifact] Blitzball #0:1 (activated_ability/ability): GOOOOAAAALLL! — {T}, Sacrifice this artifact: Draw two cards. Activate only if an opponent was dealt combat damage by a legendary creature this turn.
     1  'Darkness'  e.g. [fin 2025 | Legendary Creature — Human Kni] Cecil, Dark Knight // Cecil, Redeemed Paladin #0:1 (triggered_ability/ability): Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life total is less than or equal to half your starting life total, untap
     1  'Protect'  e.g. [fin 2025 | Legendary Creature — Human Kni] Cecil, Dark Knight // Cecil, Redeemed Paladin #1:3 (triggered_ability/ability): Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.
     1  'Particle Beam'  e.g. [fin 2025 | Legendary Creature — Avatar] Cloud of Darkness #0:1 (triggered_ability/ability): Particle Beam — When Cloud of Darkness enters, target creature an opponent controls gets -X/-X until end of turn, where X is the number of permanent c
     1  'Chaosbringer'  e.g. [fin 2025 | Artifact — Equipment] Dark Knight's Greatsword #0:2 (spell_or_static_text/ability): Chaosbringer — Equip—Pay 3 life. Activate only once each turn.
     1  'Immune'  e.g. [fin 2025 | Legendary Artifact Creature — ] Diamond Weapon #0:2 (prevention_effect/ability): Immune — Prevent all combat damage that would be dealt to Diamond Weapon.
     1  'Dragonfire Dive'  e.g. [fin 2025 | Legendary Creature — Human Nob] Dion, Bahamut's Dominant // Bahamut, Warden of Light #0:0 (spell_or_static_text/ability): Dragonfire Dive — During your turn, Dion and other Knights you control have flying.
     1  'Gae Bolg'  e.g. [fin 2025 | Artifact — Equipment] Dragoon's Lance #0:3 (keyword_ability/ability): Gae Bolg — Equip {4}
     1  'Two-Headed Coin'  e.g. [fin 2025 | Legendary Creature — Human Art] Edgar, King of Figaro #0:1 (spell_or_static_text/ability): Two-Headed Coin — The first time you flip one or more coins each turn, those coins come up heads and you win those flips.
     1  'Echo of the Lost'  e.g. [fin 2025 | Legendary Creature — Avatar] Emet-Selch, Unsundered // Hades, Sorcerer of Eld #1:4 (spell_or_static_text/ability): Echo of the Lost — During your turn, you may play cards from your graveyard.
     1  'The Allagan Eye'  e.g. [fin 2025 | Legendary Creature — Cat Arche] G'raha Tia #0:1 (triggered_ability/ability): The Allagan Eye — Whenever one or more other creatures and/or artifacts you control die, draw a card. This ability triggers only once each turn.
     1  "I've Come Up with a New Recipe!"  e.g. [fin 2025 | Legendary Creature — Human Adv] Ignis Scientia #0:1 (activated_ability/ability): I've Come Up with a New Recipe! — {1}{G}{U}, {T}: Exile target card from a graveyard. If a creature card was exiled this way, create a Food token.
     1  'N,N Needles'  e.g. [fin 2025 | Creature — Plant] Jumbo Cactuar #0:0 (triggered_ability/ability): 10,000 Needles — Whenever this creature attacks, it gets +9999/+0 until end of turn.
     1  'Flare Star'  e.g. [fin 2025 | Legendary Creature — Avatar Wi] Kuja, Genome Sorcerer // Trance Kuja, Fate Defied #1:2 (replacement_effect/ability): Flare Star — If a Wizard you control would deal damage to a permanent or player, it deals double that damage instead.
     1  'Stagger'  e.g. [fin 2025 | Legendary Creature — Human Sol] Lightning, Army of One #0:3 (triggered_ability/ability): Stagger — Whenever Lightning deals combat damage to a player, until your next turn, if a source would deal damage to that player or a permanent that p
     1  'Machina'  e.g. [fin 2025 | Artifact — Equipment] Machinist's Arsenal #0:2 (keyword_ability/ability): Machina — Equip {4}
     1  'A Test of Your Reflexes!'  e.g. [fin 2025 | Artifact — Equipment] Magitek Scythe #0:0 (triggered_ability/ability): A Test of Your Reflexes! — When this Equipment enters, you may attach it to target creature you control. If you do, that creature gains first strike u
     1  'Bad Breath'  e.g. [fin 2025 | Creature — Plant Horror] Malboro #0:0 (triggered_ability/ability): Bad Breath — When this creature enters, each opponent discards a card, loses 2 life, and exiles the top three cards of their library.
     1  'Mutsunokami'  e.g. [fin 2025 | Artifact — Equipment] Ninja's Blades #0:3 (keyword_ability/ability): Mutsunokami — Equip {2}
     1  'Wave Cannon'  e.g. [fin 2025 | Legendary Artifact Creature — ] Omega, Heartless Evolution #0:0 (triggered_ability/ability): Wave Cannon — When Omega enters, for each opponent, tap up to one target nonland permanent that opponent controls. Put X stun counters on each of thos
     1  "Lightbringer and Hero's Shield"  e.g. [fin 2025 | Artifact — Equipment] Paladin's Arms #0:2 (keyword_ability/ability): Lightbringer and Hero's Shield — Equip {4}
     1  'Selfie Shot'  e.g. [fin 2025 | Legendary Creature — Human Sco] Prompto Argentum #0:1 (triggered_ability/ability): Selfie Shot — Whenever you cast a noncreature spell, if at least four mana was spent to cast it, create a Treasure token.
     1  'Blue Magic'  e.g. [fin 2025 | Legendary Creature — Human Wiz] Quistis Trepe #0:0 (triggered_ability/ability): Blue Magic — When Quistis Trepe enters, you may cast target instant or sorcery card from a graveyard, and mana of any type can be spent to cast that s
     1  'Angelo Cannon'  e.g. [fin 2025 | Legendary Creature — Human Reb] Rinoa Heartilly #0:1 (triggered_ability/ability): Angelo Cannon — Whenever Rinoa Heartilly attacks, another target creature you control gets +1/+1 until end of turn for each creature you control.
     1  'Summon'  e.g. [fin 2025 | Legendary Creature — Human Sha] Rydia, Summoner of Mist #0:1 (activated_ability/ability): Summon — {X}, {T}: Return target Saga card with mana value X from your graveyard to the battlefield with a finality counter on it. It gains haste unti
     1  'Hagneia'  e.g. [fin 2025 | Artifact — Equipment] Sage's Nouliths #0:3 (keyword_ability/ability): Hagneia — Equip {3}
     1  'Murasame'  e.g. [fin 2025 | Artifact — Equipment] Samurai's Katana #0:2 (keyword_ability/ability): Murasame — Equip {5}
     1  'Fire Cross'  e.g. [fin 2025 | Legendary Creature — Human Kni] Seifer Almasy #0:1 (triggered_ability/ability): Fire Cross — Whenever Seifer Almasy deals combat damage to a player, you may cast target instant or sorcery card with mana value 3 or less from your g
     1  'Super Nova'  e.g. [fin 2025 | Legendary Creature — Angel Nig] Sephiroth, Fabled SOLDIER // Sephiroth, One-Winged Angel #1:3 (spell_or_static_text/ability): Super Nova — As this creature transforms into Sephiroth, One-Winged Angel, you get an emblem with "Whenever a creature dies, target opponent loses 1 l
     1  'Double Overdrive'  e.g. [fin 2025 | Legendary Artifact — Equipment] Sidequest: Play Blitzball // World Champion, Celestial Weapon #1:2 (spell_or_static_text/ability): Double Overdrive — Equipped creature gets +2/+0 and has double strike.
     1  'Rough Divide'  e.g. [fin 2025 | Legendary Creature — Human Kni] Squall, SeeD Mercenary #0:0 (triggered_ability/ability): Rough Divide — Whenever a creature you control attacks alone, it gains double strike until end of turn.
     1  'Abraxas'  e.g. [fin 2025 | Artifact — Book Equipment] Summoner's Grimoire #0:3 (keyword_ability/ability): Abraxas — Equip {3}
     1  'Starfall'  e.g. [fin 2025 | Legendary Creature — Demon Nob] The Emperor of Palamecia // The Lord Master of Hell #1:2 (triggered_ability/ability): Starfall — Whenever The Lord Master of Hell attacks, it deals X damage to each opponent, where X is the number of noncreature, nonland cards in your g
     1  "The Minstrel's Ballad"  e.g. [fin 2025 | Legendary Creature — Human Bar] The Wandering Minstrel #0:1 (triggered_ability/ability): The Minstrel's Ballad — At the beginning of combat on your turn, if you control five or more Towns, create a 2/2 Elemental creature token that's all c
     1  "Chef's Knife"  e.g. [fin 2025 | Creature — Salamander Horror] Tonberry #0:1 (spell_or_static_text/ability): Chef's Knife — During your turn, this creature has first strike and deathtouch.
     1  'Time Compression'  e.g. [fin 2025 | Legendary Creature — Nightmare] Ultimecia, Time Sorceress // Ultimecia, Omnipotent #1:3 (triggered_ability/ability): Time Compression — When this creature transforms into Ultimecia, Omnipotent, take an extra turn after this one.
     1  'Rat Tail'
     1  "Hero's Sundering"  e.g. [fin 2025 | Legendary Creature — Elder Wiz] Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal #0:1 (activated_ability/ability): Hero's Sundering — {7}, {T}: Exile target nonland permanent. Transform Venat. Activate only as a sorcery.
     1  'Blessing of Light'  e.g. [fin 2025 | Legendary Creature — God] Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal #1:3 (triggered_ability/ability): Blessing of Light — At the beginning of combat on your turn, put a +1/+1 counter on another target creature you control. Until your next turn, it gain
     1  'My First Friend'  e.g. [fin 2025 | Legendary Creature — Human Nob] Zenos yae Galvus // Shinryu, Transcendent Rival #0:0 (triggered_ability/ability): My First Friend — When Zenos yae Galvus enters, choose a creature an opponent controls. Until end of turn, creatures other than Zenos yae Galvus and t
     1  'Burning Chains'  e.g. [fin 2025 | Legendary Creature — Dragon] Zenos yae Galvus // Shinryu, Transcendent Rival #1:4 (triggered_ability/ability): Burning Chains — When the chosen player loses the game, you win the game.
     1  'Share'  e.g. [spm 2025 | Artifact — Food] Bagel and Schmear #0:0 (activated_ability/ability): Share — {W}, {T}, Sacrifice this artifact: Put a +1/+1 counter on up to one target creature. Draw a card. Activate only as a sorcery.
     1  'Nosh'  e.g. [spm 2025 | Artifact — Food] Bagel and Schmear #0:1 (activated_ability/ability): Nosh — {2}, {T}, Sacrifice this artifact: You gain 3 life and draw a card.
     1  'Smear Campaign'  e.g. [spm 2025 | Land] Daily Bugle Building #0:2 (activated_ability/ability): Smear Campaign — {1}, {T}: Target legendary creature gains menace until end of turn. Activate only as a sorcery.
     1  'Web Support'  e.g. [spm 2025 | Creature — Human Advisor] Guy in the Chair #0:1 (activated_ability/ability): Web Support — {2}{G}, {T}: Put a +1/+1 counter on target Spider. Activate only as a sorcery.
     1  'Top of the Food Chain'  e.g. [spm 2025 | Legendary Creature — Human War] Kraven, Proud Predator #0:1 (spell_or_static_text/ability): Top of the Food Chain — Kraven's power is equal to the greatest mana value among permanents you control.
     1  'Lizard Formula'  e.g. [spm 2025 | Legendary Creature — Lizard Vi] Lizard, Connors's Curse #0:1 (triggered_ability/ability): Lizard Formula — When Lizard, Connors's Curse enters, up to one other target creature loses all abilities and becomes a green Lizard creature with bas
     1  'Camouflage'  e.g. [spm 2025 | Legendary Creature — Spider Hu] Miles Morales // Ultimate Spider-Man #1:4 (activated_ability/ability): Camouflage — {2}: Put a +1/+1 counter on Ultimate Spider-Man. He gains hexproof and becomes colorless until end of turn.
     1  'Darkforce Inversion'  e.g. [spm 2025 | Legendary Creature — Human Vil] Mister Negative #0:2 (triggered_ability/ability): Darkforce Inversion — When Mister Negative enters, you may exchange life totals with target opponent. If you lost life this way, draw that many cards.
     1  'Goblin Formula'  e.g. [spm 2025 | Legendary Creature — Goblin Hu] Norman Osborn // Green Goblin #1:6 (spell_or_static_text/ability): Goblin Formula — Each nonland card in your graveyard has mayhem. The mayhem cost is equal to its mana cost.
     1  'Fateful Bite'
     1  'Sensational Save'  e.g. [spm 2025 | Legendary Creature — Spider Hu] Scarlet Spider, Ben Reilly #0:2 (replacement_effect/ability): Sensational Save — If Scarlet Spider was cast using web-slinging, he enters with X +1/+1 counters on him, where X is the mana value of the returned cr
     1  'Unreliable Visions'
     1  'Undying Vengeance'
     1  'Vibro-Shock Gauntlets'  e.g. [spm 2025 | Legendary Creature — Human Rog] Shocker, Unshakable #0:1 (triggered_ability/ability): Vibro-Shock Gauntlets — When Shocker enters, he deals 2 damage to target creature and 2 damage to that creature's controller.
     1  'Sonic Blast'  e.g. [spm 2025 | Legendary Creature — Mutant Vi] Shriek, Treblemaker #0:2 (triggered_ability/ability): Sonic Blast — Whenever a creature an opponent controls dies, Shriek deals 1 damage to that player.
     1  'Animal May-Ham'  e.g. [spm 2025 | Legendary Creature — Spider Bo] Spider-Ham, Peter Porker #0:1 (spell_or_static_text/ability): Animal May-Ham — Other Spiders, Boars, Bats, Bears, Birds, Cats, Dogs, Frogs, Jackals, Lizards, Mice, Otters, Rabbits, Raccoons, Rats, Squirrels, Turt
     1  'From the Future'  e.g. [spm 2025 | Legendary Creature — Spider Hu] Spider-Man 2099 #0:0 (spell_or_static_text/ability): From the Future — You can't cast Spider-Man 2099 during your first, second, or third turns of the game.
     1  "Pavitr's Sevā"  e.g. [spm 2025 | Legendary Creature — Spider Hu] Spider-Man India #0:1 (triggered_ability/ability): Pavitr's Sevā — Whenever you cast a creature spell, put a +1/+1 counter on target creature you control. It gains flying until end of turn.
     1  'Venom Blast'  e.g. [spm 2025 | Legendary Creature — Spider Hu] Spider-Woman, Stunning Savior #0:1 (replacement_effect/ability): Venom Blast — Artifacts and creatures your opponents control enter tapped.
     1  'Dinosaur Formula'  e.g. [spm 2025 | Legendary Creature — Dinosaur ] Stegron the Dinosaur Man #0:1 (activated_ability/ability): Dinosaur Formula — {1}{R}, Discard this card: Until end of turn, target creature you control gets +3/+1 and becomes a Dinosaur in addition to its othe
     1  'Mind Swap'  e.g. [spm 2025 | Legendary Creature — Spider Hu] Superior Spider-Man #0:0 (replacement_effect/ability): Mind Swap — You may have Superior Spider-Man enter as a copy of any creature card in a graveyard, except his name is Superior Spider-Man and he's a 4/
     1  'Find New Host'  e.g. [spm 2025 | Legendary Creature — Symbiote ] Symbiote Spider-Man #0:1 (activated_ability/ability): Find New Host — {2}{U/B}, Exile this card from your graveyard: Put a +1/+1 counter on target creature you control. It gains this card's other abilitie
     1  'Fear Gas'  e.g. [spm 2025 | Legendary Creature — Human Det] Wraith, Vicious Vigilante #0:1 (spell_or_static_text/ability): Fear Gas — Wraith can't be blocked.
     1  'In You, All Things Are Possible'
     1  'Family Gathering'  e.g. [sld 2024 | Legendary Creature — Pony] Applejack #0:0 (triggered_ability/ability): Family Gathering — At the beginning of your end step, put a toy you own onto the battlefield as a 2/2 creature token with that toy's name, colors, and
     1  'Survey the Realm'  e.g. [sld 2024 | Legendary Creature — Human Nob] Black Panther, Wakandan King #0:1 (triggered_ability/ability): Survey the Realm — Whenever Black Panther or another creature you control enters, put a +1/+1 counter on target land you control.
     1  'Mine Vibranium'  e.g. [sld 2024 | Legendary Creature — Human Nob] Black Panther, Wakandan King #0:2 (activated_ability/ability): Mine Vibranium — {3}: Move all +1/+1 counters from target land you control onto target creature. If one or more +1/+1 counters are moved this way, you
     1  'Distract the Horde'  e.g. [sld 2025 | Legendary Creature — Human Sur] Ellie, Brick Master #0:0 (triggered_ability/ability): Distract the Horde — Whenever a player attacks one of your opponents, that attacking player creates a tapped 1/1 black Fungus Zombie creature token na
     1  'Genius Industrialist'  e.g. [sld 2024 | Legendary Artifact Creature — ] Iron Man, Titan of Innovation #0:2 (triggered_ability/ability): Genius Industrialist — Whenever Iron Man attacks, create a Treasure token, then you may sacrifice a noncreature artifact. If you do, search your libra
     1  'Treasure Hunter'  e.g. [sld 2025 | Legendary Creature — Echidna W] Knuckles the Echidna #0:4 (triggered_ability/ability): Treasure Hunter — At the beginning of your upkeep, if you control thirty or more artifacts, you win the game.
     1  'Golden Rule'  e.g. [sld 2026 | Legendary Creature — Human Sur] Lucy MacLean, Positively Armed #0:0 (triggered_ability/ability): Golden Rule — Whenever a token enters, you may have target player other than its controller create a token that's a copy of it, then you draw a card i
     1  "Everypony's Invited"  e.g. [sld 2024 | Legendary Creature — Pony] Pinkie Pie #0:1 (spell_or_static_text/ability): Everypony's Invited — Your party consists of each creature you control, and your party is always full.
     1  'Sonic Rainboom'
     1  'Chaos Control'  e.g. [sld 2025 | Legendary Creature — Hedgehog ] Shadow the Hedgehog #0:2 (spell_or_static_text/ability): Chaos Control — Each spell you cast has split second if mana from an artifact was spent to cast it.
     1  'Gotta Go Fast'  e.g. [sld 2025 | Legendary Creature — Hedgehog ] Sonic the Hedgehog #0:1 (triggered_ability/ability): Gotta Go Fast — Whenever Sonic the Hedgehog attacks, put a +1/+1 counter on each creature you control with flash or haste.
     1  'Ceaseless Tempest'  e.g. [sld 2024 | Legendary Creature — Mutant He] Storm, Force of Nature #0:2 (triggered_ability/ability): Ceaseless Tempest — Whenever Storm deals combat damage to a player, the next instant or sorcery spell you cast this turn has storm.
     1  'Unrivaled Lethality'  e.g. [sld 2024 | Legendary Creature — Mutant Be] Wolverine, Best There Is #0:0 (spell_or_static_text/ability): Unrivaled Lethality — Double all damage Wolverine would deal.
     1  'Covercast'  e.g. [ysos 2026 | Instant] Summitfest Closing Ceremony #0:1 (triggered_ability/ability): Covercast — Whenever you cast another instant or sorcery spell, if five or more mana was spent to cast it, this card intensifies.
     1  'Sense the Good'  e.g. [msc 2026 | Legendary Creature — Human Art] Alicia Masters, Skilled Sculptor #0:1 (triggered_ability/ability): Sense the Good — At the beginning of your end step, each player gains control of all creatures they own.
     1  'Lethal Voice'  e.g. [msc 2026 | Legendary Creature — Inhuman N] Black Bolt, Inhuman King #0:2 (triggered_ability/ability): Lethal Voice — Whenever Black Bolt becomes the target of a spell or ability an opponent controls, destroy target nonland permanent that player control
     1  'Origami-Fu'  e.g. [msc 2026 | Legendary Creature — Mutant He] Flatman #0:0 (activated_ability/ability): Origami-Fu — {2}{G}: Switch Flatman's power and toughness until end of turn.
     1  'Insatiable Hunger'  e.g. [msc 2026 | Legendary Creature — Elder Ali] Galactus, Devourer of Worlds #0:4 (spell_or_static_text/ability): Insatiable Hunger — Galactus attacks an opponent with the most life among your opponents each combat if able unless you control a creature named Silve
     1  'Cosmic Awareness'  e.g. [msh 2026 | Legendary Creature — Kree Sold] Captain Mar-Vell, Space-Born #0:2 (spell_or_static_text/ability): Cosmic Awareness — As long as an opponent has cast a spell this turn, you may cast spells as though they had flash.
     1  'Radar Sense'  e.g. [msh 2026 | Legendary Creature — Human Her] Daredevil, Man Without Fear #0:2 (spell_or_static_text/ability): Radar Sense — You may look at the top card of your library any time.
     1  'Avian Telepathy'  e.g. [msh 2026 | Legendary Creature — Human Her] Falcon, Winged Wonder #0:1 (triggered_ability/ability): Avian Telepathy — When Falcon enters, create Redwing, a legendary 1/1 blue Bird Scout creature token with flying and "Whenever Redwing attacks, survei
     1  'Intangibility'  e.g. [msh 2026 | Legendary Creature — Human Rog] Ghost, Spectral Saboteur #0:1 (spell_or_static_text/ability): Intangibility — Ghost can't be blocked.
     1  'Trick Arrows'  e.g. [msh 2026 | Legendary Creature — Human Arc] Hawkeye, Master Marksman #0:2 (triggered_ability/ability): Trick Arrows — Whenever Hawkeye becomes tapped, you may pay {1} up to three times.
     1  'Sonic Attack'  e.g. [msh 2026 | Legendary Creature — Human Rog] Klaw, Sonic Subjugator #0:0 (triggered_ability/ability): Sonic Attack — When Klaw enters, target player reveals a number of cards from their hand equal to one plus the number of creature cards in your gravey
     1  'Unbreakable Skin'  e.g. [msh 2026 | Legendary Creature — Human Her] Luke Cage, Power Man #0:0 (triggered_ability/ability): Unbreakable Skin — Whenever Luke Cage attacks alone, he gets +2/+0 and gains indestructible until end of turn.
     1  'Mental Organism'
     1  'Designed Only for Killing'
     1  'Embiggen Fist'  e.g. [msh 2026 | Legendary Creature — Mutant In] Ms. Marvel, Kamala Khan #0:3 (triggered_ability/ability): Embiggen Fist — Whenever you cast a spell that targets a creature you control, draw a card. Until end of turn, Ms. Marvel gains "Ms. Marvel's base pow
     1  'Seismic Takedown'  e.g. [msh 2026 | Legendary Creature — Inhuman S] Quake, Agent of S.H.I.E.L.D. #0:0 (triggered_ability/ability): Seismic Takedown — Whenever you cast a noncreature spell, tap target creature or land.
     1  'Brontosaurus'  e.g. [msh 2026 | Legendary Creature — Human Her] Reptil, Dinomorpher #0:0 (activated_ability/ability): Brontosaurus — {3}: Until end of turn, Reptil becomes a Dinosaur Hero with base power and toughness 3/5 and gains reach and vigilance.
     1  'Tyrannosaurus Rex'  e.g. [msh 2026 | Legendary Creature — Human Her] Reptil, Dinomorpher #0:1 (activated_ability/ability): Tyrannosaurus Rex — {6}: Until end of turn, Reptil becomes a Dinosaur Hero with base power and toughness 6/6 and gains trample.
     1  'No One Dies!'  e.g. [msh 2026 | Legendary Creature — Spider Hu] Spider-Man, To the Rescue #0:3 (triggered_ability/ability): No One Dies! — When Spider-Man enters, you may tap him.
     1  'Photographic Reflexes'  e.g. [msh 2026 | Legendary Creature — Human Mer] Taskmaster, Mercenary Mimic #0:0 (triggered_ability/ability): Photographic Reflexes — At the beginning of your first main phase, until your next turn, Taskmaster becomes a copy of up to one target creature on the
     1  'Do You Like Squirrels?'  e.g. [msh 2026 | Legendary Creature — Squirrel ] The Unbeatable Squirrel Girl #0:0 (triggered_ability/ability): Do You Like Squirrels? — Whenever The Unbeatable Squirrel Girl enters or attacks, create a 1/1 green Squirrel creature token.
     1  'I LOVE Squirrels!'  e.g. [msh 2026 | Legendary Creature — Squirrel ] The Unbeatable Squirrel Girl #0:1 (activated_ability/ability): I LOVE Squirrels! — {1}{G}{G}{G}: Create X 1/1 green Squirrel creature tokens, where X is the number of Squirrels you control.
     1  "Wasp's Sting"  e.g. [msh 2026 | Legendary Creature — Human Her] The Wondrous Wasp #0:2 (triggered_ability/ability): Wasp's Sting — When The Wondrous Wasp enters, tap up to one target creature. It loses all abilities for as long as The Wondrous Wasp remains on the ba
     1  'Cybernetic Senses'
     1  'Strange new worlds'  e.g. [trc 2026 | Legendary Creature — Human Doc] Christine Chapel, Combat Medic #0:0 (triggered_ability/ability): Strange new worlds — When Christine Chapel enters, you gain life equal to the number of differently named lands you control.
     1  'Reckless Behavior'  e.g. [trk 2026 | Legendary Creature — Human Off] Beckett Mariner, Impetuous Ensign #0:1 (triggered_ability/ability): Reckless Behavior — Whenever Beckett Mariner becomes tapped, put a promotion counter on her. Then if there are three or more promotion counters on her

## P3.3 Anomaly flags on fired prefixes (every occurrence; the S11 rare-result inspection list)
flagged: 417 (pool 22); by flag: Counter({'punct/symbol': 282, 'numeral': 114, 'long>25': 18, 'comma': 6, 'clause-like': 3})
  ['numeral']: prefix='N' :: [ugl 1998 | Instant] Goblin Tutor #0:1 (keyword_ability/ability): 2 — A card named Goblin Tutor
  ['numeral']: prefix='N' :: [ugl 1998 | Instant] Goblin Tutor #0:2 (keyword_ability/ability): 3 — An enchantment
  ['numeral']: prefix='N' :: [ugl 1998 | Instant] Goblin Tutor #0:3 (keyword_ability/ability): 4 — An artifact
  ['numeral']: prefix='N' :: [ugl 1998 | Instant] Goblin Tutor #0:4 (keyword_ability/ability): 5 — A creature
  ['numeral']: prefix='N' :: [ugl 1998 | Instant] Goblin Tutor #0:5 (keyword_ability/ability): 6 — An instant or sorcery
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:1 (spell_or_static_text/ability): 1 — Sacrifice this artifact and you lose 5 life.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:2 (spell_or_static_text/ability): 2 — Add {W}.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:3 (spell_or_static_text/ability): 3 — Add {U}.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:4 (spell_or_static_text/ability): 4 — Add {B}.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:5 (spell_or_static_text/ability): 5 — Add {R}.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact] Jack-in-the-Mox #0:6 (spell_or_static_text/ability): 6 — Add {G}.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:1 (spell_or_static_text/ability): 1 — Do nothing.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:2 (spell_or_static_text/ability): 2 — Destroy all artifacts.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:3 (spell_or_static_text/ability): 3 — Destroy all lands.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:4 (spell_or_static_text/ability): 4 — Strategy, Schmategy deals 3 damage to each creature and each player.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:5 (spell_or_static_text/ability): 5 — Each player discards their hand and draws seven cards.
  ['numeral']: prefix='N' :: [ugl 1998 | Sorcery] Strategy, Schmategy #0:6 (spell_or_static_text/ability): 6 — Repeat this process two more times.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:1 (spell_or_static_text/ability): 1 — It gets -2/-2 until end of turn.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:2 (prevention_effect/ability): 2 — Prevent all combat damage it would deal this turn.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:3 (spell_or_static_text/ability): 3 — It gains vigilance until end of turn.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:4 (spell_or_static_text/ability): 4 — It gains first strike until end of turn.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:5 (spell_or_static_text/ability): 5 — It gains flying until end of turn.
  ['numeral']: prefix='N' :: [ugl 1998 | Artifact Creature — Construct] Urza's Science Fair Project #0:6 (spell_or_static_text/ability): 6 — It gets +2/+2 until end of turn.
  ['clause-like']: prefix='Choose one' :: [tfth 2013 | Sorcery] Noxious Hydra Breath #0:0 (spell_or_static_text/ability): Choose one — Noxious Hydra Breath deals 5 damage to each player; or destroy each tapped non-Head creature.
  ['numeral']: prefix='Stage N' :: [cmb1 2019 | Creature — Lobster] Loopy Lobster #0:2 (keyword_ability/ability): Stage 2 — Evolve
  ['numeral']: prefix='Stage N' :: [cmb1 2019 | Creature — Lobster] Loopy Lobster #0:3 (activated_ability/ability): Stage 3 — {U}: Loopy Lobster gets +1/-1 until end of turn.
  ['numeral']: prefix='Stage N' :: [cmb1 2019 | Creature — Lobster] Loopy Lobster #0:4 (activated_ability/ability): Stage 4 — Vigilance. {T}: Draw 2 cards.
  ['numeral']: prefix='N | Trapped!' :: [afr 2021 | Artifact] Treasure Chest #0:1 (spell_or_static_text/ability): 1 | Trapped! — You lose 3 life.
  ['long>25']: prefix='Lord of the Pyrrhian Legions' :: [40k 2022 | Legendary Artifact Creature — ] Anrakyr the Traveller #0:0 (triggered_ability/ability): Lord of the Pyrrhian Legions — Whenever Anrakyr the Traveller attacks, you may cast an artifact spell from your hand or graveyard by paying life equal
  ['long>25']: prefix='Primarch of the Death Guard' :: [40k 2022 | Legendary Creature — Demon Pri] Mortarion, Daemon Primarch #0:1 (triggered_ability/ability): Primarch of the Death Guard — At the beginning of your end step, you may pay {X}. If you do, create X 2/2 black Astartes Warrior creature tokens with 
  ['long>25']: prefix='Benediction of the Omnissiah' :: [40k 2022 | Artifact Creature — Human Sold] Sicarian Infiltrator #0:2 (triggered_ability/ability): Benediction of the Omnissiah — When this creature enters, draw a card.
  ['long>25']: prefix='A Thousand Souls Die Every Day' :: [40k 2022 | Legendary Artifact] The Golden Throne #0:1 (activated_ability/ability): A Thousand Souls Die Every Day — {T}, Sacrifice a creature: Add three mana in any combination of colors.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:0 (keyword_ability/ability): {TK}{TK} — Afflict 2
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Flying
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:0 (activated_ability/ability): {TK}{TK} — Sacrifice this permanent: Draw two cards.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:1 (triggered_ability/ability): {TK}{TK}{TK} — Whenever this creature attacks, proliferate.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:0 (activated_ability/ability): {TK}{TK} — {T}: Target creature gains haste until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Deathtouch, lifelink
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/5
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:0 (triggered_ability/ability): {TK}{TK} — When this permanent leaves the battlefield, draw a card.
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK}{TK} — Whenever a creature enters under your control, this permanent becomes a 13/13 Eldrazi creature in addition to its other types u
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 5/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:0 (keyword_ability/ability): {TK}{TK} — Prowess, prowess
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:1 (activated_ability/ability): {TK}{TK}{TK}{TK}{TK} — {2}, {T}: This permanent deals 2 damage to any target.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/8
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, scry 1.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:1 (triggered_ability/ability): {TK}{TK}{TK} — Whenever this permanent leaves the battlefield, you may destroy target artifact or enchantment.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 4/5
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 8/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:0 (keyword_ability/ability): {TK}{TK} — Outlast {1}
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:1 (triggered_ability/ability): {TK}{TK}{TK} — When this permanent dies, you get seven {TK}.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 9/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:0 (keyword_ability/ability): {TK}{TK} — Haste
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:1 (spell_or_static_text/ability): {TK}{TK}{TK}{TK}{TK} — You may cast this card from your graveyard by paying 2 life in addition to paying its other costs.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:0 (activated_ability/ability): {TK}{TK} — Exile this permanent: You may cast target nonland card from your graveyard this turn.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever a creature you control dies, each opponent loses 1 life and you gain 1 life.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature deals combat damage to a player, exile target creature you control, then return it to the battlefield under its owne
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:1 (activated_ability/ability): {TK}{TK}{TK} — {T}: Untap another target permanent.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 4/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/8
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, untap target permanent.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever a creature enters under your control, creatures you control get +1/+1 until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:0 (activated_ability/ability): {TK}{TK} — {2}: This creature gets +2/+0 until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever a creature enters under your control, this permanent deals 2 damage to target player.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Giant Mana Cake #0:0 (triggered_ability/ability): {TK}{TK} — When this permanent leaves the battlefield, create two Food tokens.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Giant Mana Cake #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — When this permanent dies, destroy target nonland permanent.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Giant Mana Cake #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Giant Mana Cake #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/2
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Goblin Coward Parade #0:0 (keyword_ability/ability): {TK}{TK} — Mentor
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Goblin Coward Parade #0:1 (triggered_ability/ability): {TK}{TK}{TK} — When this permanent leaves the battlefield, you may destroy target creature with power 4 or greater.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Goblin Coward Parade #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Goblin Coward Parade #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 8/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:0 (activated_ability/ability): {TK}{TK} — {T}: Add {C}{C}. Spend this mana only to cast noncreature spells.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Infect
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:0 (triggered_ability/ability): {TK}{TK} — Whenever you cast a spell, this creature gets +X/+X until end of turn, where X is the amount of generic mana in that spell's mana cost.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Hexproof
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:0 (keyword_ability/ability): {TK}{TK} — Lifelink
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:1 (spell_or_static_text/ability): {TK}{TK}{TK} — This creature must be blocked if able. Whenever this creature becomes blocked, it gets +1/+1 until end of turn for each creature blocki
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/8
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, you gain 2 life.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Vigilance, reach
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:0 (keyword_ability/ability): {TK}{TK} — Menace
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Persist
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK}{TK} — 10/10
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Notorious Sliver War #0:0 (activated_ability/ability): {TK}{TK} — {5}: Creatures you control get +1/+1 until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Notorious Sliver War #0:1 (spell_or_static_text/ability): {TK}{TK}{TK} — Protection from creatures with two or more creature types
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Notorious Sliver War #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Notorious Sliver War #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 9/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, you get {TK}.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Undying
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/9
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:0 (activated_ability/ability): {TK}{TK} — {T}: Draw a card, then discard a card.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature attacks, you gain 3 life and draw a card.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 4/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Primal Elder Kitty #0:0 (activated_ability/ability): {TK}{TK} — {1}: This creature gets +1/-1 until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Primal Elder Kitty #0:1 (triggered_ability/ability): {TK}{TK}{TK} — When this creature dies, you may put X +1/+1 counters on target creature, where X is this creature's power.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Primal Elder Kitty #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Primal Elder Kitty #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, create a Treasure token.
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:1 (activated_ability/ability): {TK}{TK}{TK}{TK}{TK} — {3}: Target creature gains flying until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK}{TK} — 10/10
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:0 (keyword_ability/ability): {TK}{TK} — Bushido 2
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Double strike
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 5/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:0 (activated_ability/ability): {TK}{TK} — {1}: Target creature gains all creature types until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:1 (spell_or_static_text/ability): {TK}{TK}{TK} — Magecraft — Whenever you cast or copy an instant or sorcery spell, draw a card.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Spooky Clown Mox #0:0 (keyword_ability/ability): {TK}{TK} — Vigilance
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Spooky Clown Mox #0:1 (activated_ability/ability): {TK}{TK}{TK}{TK} — {1}, {T}: Tap target creature.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Spooky Clown Mox #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Spooky Clown Mox #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Squid Fire Knight #0:0 (activated_ability/ability): {TK}{TK} — {T}: The next time target player would roll one or more dice this turn, instead they roll that many dice plus one, then you choose one of t
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Squid Fire Knight #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Protection from odd mana values
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Squid Fire Knight #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Squid Fire Knight #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:0 (keyword_ability/ability): {TK}{TK} — Ward {2}
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Provoke
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this permanent dies, you may return target creature to its owner's hand.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature attacks, creatures you control get +1/+1 until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Trained Blessed Mind #0:0 (activated_ability/ability): {TK}{TK} — {T}: Exile target card from a graveyard.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Trained Blessed Mind #0:1 (spell_or_static_text/ability): {TK}{TK}{TK} — Threshold — As long as seven or more cards are in your graveyard, this creature gets +4/+0 and has trample.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Trained Blessed Mind #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Trained Blessed Mind #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/9
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:0 (keyword_ability/ability): {TK}{TK} — Deathtouch
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK}{TK} — Whenever this creature deals combat damage to a player, create that many 1/1 green Squirrel creature tokens.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:0 (triggered_ability/ability): {TK}{TK} — When this permanent dies, return target noncreature, nonland card from your graveyard to your hand.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature deals combat damage to a player, that player mills twice that many cards.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 7/2
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:0 (activated_ability/ability): {TK}{TK} — {T}: Add {2}. Spend this mana only to cast creature spells.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — At the beginning of combat on your turn, target noncreature artifact you control becomes a 4/4 artifact creature with flying until 
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:0 (activated_ability/ability): {TK}{TK} — {T}: You gain 1 life.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature attacks, tap each creature an opponent controls with the same power and/or same toughness as this creature.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/6
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unique Charmed Pants #0:0 (activated_ability/ability): {TK}{TK} — {T}: Add one mana of any color.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Unique Charmed Pants #0:1 (triggered_ability/ability): {TK}{TK}{TK} — Whenever this creature attacks, if it's not a Brushwagg, it gets +X/+0 until end of turn, where X is the number of supertypes, card typ
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unique Charmed Pants #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unique Charmed Pants #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/8
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, bolster 1.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Indestructible
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:0 (activated_ability/ability): {TK}{TK} — {1}: Switch this creature's power and toughness until end of turn.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature attacks, it gets +5/+5 until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/7
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:0 (keyword_ability/ability): {TK}{TK} — Exalted, exalted
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Shadow
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 7/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Vampire Champion Fury #0:0 (spell_or_static_text/ability): {TK}{TK} — Hellbent — This creature gets +3/+3 as long as you have no cards in hand.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Vampire Champion Fury #0:1 (activated_ability/ability): {TK}{TK}{TK}{TK} — {2}, Sacrifice this creature: It deals X damage divided as you choose among any number of target creatures, where X is its power.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Vampire Champion Fury #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Vampire Champion Fury #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/3
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Weird Angel Flame #0:0 (spell_or_static_text/ability): {TK}{TK} — Heroic — Whenever you cast a spell that targets this permanent, put two +1/+1 counters on it.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Weird Angel Flame #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Protection from even mana values
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Weird Angel Flame #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Weird Angel Flame #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/8
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:0 (spell_or_static_text/ability): {TK}{TK} — Landfall — Whenever a land enters under your control, put a +1/+1 counter on this permanent.
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever a creature blocks this creature, that creature gets -4/-4 until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/5
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:0 (triggered_ability/ability): {TK}{TK} — Whenever this creature attacks, put a +1/+1 counter on it.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:1 (spell_or_static_text/ability): {TK}{TK}{TK} — Metalcraft — This permanent has protection from noncreature permanents as long as you control three or more artifacts.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 7/4
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:0 (triggered_ability/ability): {TK}{TK} — When this permanent leaves the battlefield, target player discards a card.
  ['punct/symbol']: prefix='{M}{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK}{TK} — When this permanent leaves the battlefield, create five 1/1 white Clown Robot artifact creature tokens.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/5
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:0 (keyword_ability/ability): {TK}{TK} — First strike
  ['punct/symbol']: prefix='{M}{M}{M}{M}' :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:1 (triggered_ability/ability): {TK}{TK}{TK}{TK} — Whenever this creature deals combat damage to a player, draw that many cards.
  ['punct/symbol']: prefix='{M}{M}' :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  ['punct/symbol']: prefix='{M}{M}{M}' :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/2
  ['numeral']: prefix='N' :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:1 (keyword_ability/mode): 2 — menace
  ['numeral']: prefix='N' :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:2 (keyword_ability/mode): 3 — vigilance
  ['numeral']: prefix='N' :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:3 (keyword_ability/mode): 4 — lifelink
  ['numeral']: prefix='N' :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:4 (keyword_ability/mode): 5 — flying
  ['numeral']: prefix='N' :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:5 (keyword_ability/mode): 6 — indestructible
  ['numeral']: prefix='N or N' :: [unf 2022 | Legendary Planeswalker — Comet] Comet, Stellar Pup #0:1 (spell_or_static_text/ability): 1 or 2 — [+2], then create two 1/1 green Squirrel creature tokens. They gain haste until end of turn.
  ['numeral']: prefix='N' :: [unf 2022 | Legendary Planeswalker — Comet] Comet, Stellar Pup #0:2 (spell_or_static_text/ability): 3 — [−1], then return a card with mana value 2 or less from your graveyard to your hand.
  ['numeral']: prefix='N or N' :: [unf 2022 | Legendary Planeswalker — Comet] Comet, Stellar Pup #0:3 (spell_or_static_text/ability): 4 or 5 — Comet deals damage equal to the number of loyalty counters on him to a creature or player, then [−2].
  ['numeral']: prefix='N' :: [unf 2022 | Legendary Planeswalker — Comet] Comet, Stellar Pup #0:4 (spell_or_static_text/ability): 6 — [+1], and you may activate Comet's loyalty ability two more times this turn.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:1 (spell_or_static_text/ability): 1 — It has base toughness 1 until end of turn.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:2 (spell_or_static_text/ability): 2 — Put two -1/-1 counters on it.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:3 (spell_or_static_text/ability): 3 — Six-Sided Die deals 3 damage to it and you gain 3 life.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:4 (spell_or_static_text/ability): 4 — It gets -4/-4 until end of turn.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:5 (spell_or_static_text/ability): 5 — Destroy it.
  ['numeral']: prefix='N' :: [unf 2022 | Instant] Six-Sided Die #0:6 (spell_or_static_text/ability): 6 — Exile it.
  ['comma', 'long>25']: prefix='Gear Up, sponsored by Wizards of the Coast' :: [unf 2022 | Artifact — Equipment] Souvenir T-Shirt #0:0 (replacement_effect/ability): Gear Up, sponsored by Wizards of the Coast — As this Equipment enters, roll two six-sided dice. For each Magic-branded item you're wearing, roll an ad
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [ybro 2022 | Artifact Creature — Construct] Warzone Duplicator #0:0 (keyword_ability/ability): Prototype {3}{U} — 3/3
  ['numeral']: prefix='Suspend N' :: [unk 2025 | Sorcery] 17-Year Cicadas #0:1 (keyword_ability/ability): Suspend 17 — {0}
  ['punct/symbol']: prefix='{M}' :: [unk 2025 | Land — Barnyard] Blustering Barnyard #0:3 (spell_or_static_text/ability): {P} — Create a 1/1 white Bird token with flying.
  ['punct/symbol']: prefix='{M}{M}' :: [unk 2025 | Land — Barnyard] Blustering Barnyard #0:4 (spell_or_static_text/ability): {P}{P} — Create a 2/2 white Horse token with horsemanship.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [unk 2025 | Land — Barnyard] Blustering Barnyard #0:5 (spell_or_static_text/ability): {P}{P}{P} — Create a 3/3 green Ox token with trample.
  ['long>25']: prefix='Could you repeat that again?' :: [unk 2025 | Sorcery] Dialogue Tree #0:3 (spell_or_static_text/mode): Could you repeat that again? — Exile Dialogue Tree.
  ['numeral']: prefix='Commander Suspend N' :: [unk 2023 | Legendary Creature — Ooze] Groaaaaag, Hungry Monster #0:0 (keyword_ability/ability): Commander Suspend 4 — {B}{G}
  ['comma', 'long>25']: prefix='Beeeeeeep, Beeeeeeep, Beeeeeeep' :: [unk 2023 | Artifact — Vehicle] Huge Truck #0:0 (triggered_ability/ability): Beeeeeeep, Beeeeeeep, Beeeeeeep — Whenever another creature you control becomes the target of a backup ability, Huge Truck permanently gains all of th
  ['long>25']: prefix='That Could Actually Be Dangerous' :: [unk 2023 | Instant] More of That Strange Oil... #0:2 (spell_or_static_text/mode): That Could Actually Be Dangerous — Counter target creature, artifact, or planeswalker spell. Scry 1.
  ['numeral']: prefix='Nitro-N' :: [who 2023 | Legendary Creature — Human Reb] Ace, Fearless Rebel #0:0 (triggered_ability/ability): Nitro-9 — Whenever Ace attacks, you may sacrifice an artifact.
  ['long>25']: prefix='The Most Important Punch in History' :: [who 2023 | Legendary Creature — Human Det] Duggan, Private Detective #0:2 (activated_ability/ability): The Most Important Punch in History — {1}{G}, {T}: Duggan deals damage equal to twice its power to another target creature. Activate only once.
  ['long>25', 'clause-like']: prefix='Each opponent faces a villainous choice' :: [who 2023 | Sorcery] Ensnared by the Mara #0:0 (spell_or_static_text/ability): Each opponent faces a villainous choice — They exile cards from the top of their library until they exile a nonland card, then you may cast that card 
  ['long>25']: prefix='Woman Who Walked the Earth' :: [who 2023 | Legendary Creature — Human Cle] Martha Jones #0:0 (triggered_ability/ability): Woman Who Walked the Earth — When Martha Jones enters, investigate.
  ['long>25']: prefix='Sanctified Rules of Combat' :: [who 2023 | Creature — Alien Soldier] Sycorax Commander #0:2 (triggered_ability/ability): Sanctified Rules of Combat — When this creature enters, each opponent faces a villainous choice — That opponent discards all the cards in their hand, 
  ['long>25']: prefix='Deal with the Black Guardian' :: [who 2023 | Legendary Creature — Rogue] Vislor Turlough #0:0 (triggered_ability/ability): Deal with the Black Guardian — When Vislor Turlough enters, you may have an opponent gain control of it. If you do, it's goaded for as long as they co
  ['numeral']: prefix='Descend N' :: [lcc 2023 | Sorcery] Bygone Marvels #0:0 (triggered_ability/ability): Descend 8 — When you cast this spell, if there are eight or more permanent cards in your graveyard, copy this spell twice. You may choose new targets 
  ['numeral']: prefix='Descend N' :: [lci 2023 | Legendary Creature — Fungus] Akawalli, the Seething Tower #0:0 (spell_or_static_text/ability): Descend 4 — As long as there are four or more permanent cards in your graveyard, Akawalli gets +2/+2 and has trample.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Legendary Creature — Fungus] Akawalli, the Seething Tower #0:1 (spell_or_static_text/ability): Descend 8 — As long as there are eight or more permanent cards in your graveyard, Akawalli gets an additional +2/+2 and can't be blocked by more than 
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Capybara] Basking Capybara #0:0 (spell_or_static_text/ability): Descend 4 — This creature gets +3/+0 as long as there are four or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Raccoon] Coati Scavenger #0:0 (triggered_ability/ability): Descend 4 — When this creature enters, if there are four or more permanent cards in your graveyard, return target permanent card from your graveyard t
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Spirit Advisor] Council of Echoes #0:1 (triggered_ability/ability): Descend 4 — When this creature enters, if there are four or more permanent cards in your graveyard, return up to one target nonland permanent other th
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Spirit Cleric] Didact Echo #0:1 (spell_or_static_text/ability): Descend 4 — This creature has flying as long as there are four or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Salamander Wurm] Frilled Cave-Wurm #0:0 (spell_or_static_text/ability): Descend 4 — This creature gets +2/+0 as long as there are four or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Instant] Join the Dead #0:1 (spell_or_static_text/ability): Descend 4 — That creature gets -10/-10 until end of turn instead if there are four or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Cat Warrior] Malamet Veteran #0:1 (triggered_ability/ability): Descend 4 — Whenever this creature attacks, if there are four or more permanent cards in your graveyard, put a +1/+1 counter on target creature.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Spirit Horror] Starving Revenant #0:1 (triggered_ability/ability): Descend 8 — Whenever you draw a card, if there are eight or more permanent cards in your graveyard, target opponent loses 1 life and you gain 1 life.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Creature — Insect Horror] Stinging Cave Crawler #0:1 (triggered_ability/ability): Descend 4 — Whenever this creature attacks, if there are four or more permanent cards in your graveyard, you draw a card and you lose 1 life.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Legendary Creature — Spirit Go] The Ancient One #0:0 (spell_or_static_text/ability): Descend 8 — The Ancient One can't attack or block unless there are eight or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Legendary Artifact] The Everflowing Well // The Myriad Pools #0:1 (triggered_ability/ability): Descend 8 — At the beginning of your upkeep, if there are eight or more permanent cards in your graveyard, transform The Everflowing Well.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Legendary Creature — Skeleton ] Uchbenbak, the Great Mistake #0:2 (activated_ability/ability): Descend 8 — {4}{U}{B}: Return this card from your graveyard to the battlefield with a finality counter on it. Activate only if there are eight or more
  ['numeral']: prefix='Descend N' :: [lci 2023 | Sorcery] Wail of the Forgotten #0:0 (spell_or_static_text/ability): Descend 8 — Choose one. If there are eight or more permanent cards in your graveyard as you cast this spell, choose one or more instead.
  ['numeral']: prefix='Descend N' :: [lci 2023 | Artifact — Vehicle] Waterlogged Hulk // Watertight Gondola #1:3 (spell_or_static_text/ability): Descend 8 — This Vehicle can't be blocked as long as there are eight or more permanent cards in your graveyard.
  ['numeral']: prefix='Descend N' :: [ylci 2023 | Creature — Insect Horror] Chitinous Crawler #0:1 (activated_ability/ability): Descend 8 — Exile a permanent card from your graveyard: You may play it. Activate only as a sorcery and only if there are eight or more permanent card
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Wizard] Arcane Proxy #0:0 (keyword_ability/ability): Prototype {1}{U}{U} — 2/1
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Assembly-W] Autonomous Assembler #0:0 (keyword_ability/ability): Prototype {1}{W} — 2/2
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Construct] Blitz Automaton #0:0 (keyword_ability/ability): Prototype {2}{R} — 3/2
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Golem] Boulderbranch Golem #0:0 (keyword_ability/ability): Prototype {3}{G} — 3/3
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Construct] Combat Thresher #0:0 (keyword_ability/ability): Prototype {2}{W} — 1/1
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Golem] Cradle Clearcutter #0:0 (keyword_ability/ability): Prototype {2}{G} — 1/3
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Dragon] Fallaji Dragon Engine #0:0 (keyword_ability/ability): Prototype {2}{R} — 1/3
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Construct] Goring Warplow #0:0 (keyword_ability/ability): Prototype {1}{B} — 1/1
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Shapeshift] Hulking Metamorph #0:0 (keyword_ability/ability): Prototype {2}{U}{U} — 3/3
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Wurm] Iron-Craw Crusher #0:0 (keyword_ability/ability): Prototype {2}{G}{G} — 2/5
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Phyrexian ] Phyrexian Fleshgorger #0:0 (keyword_ability/ability): Prototype {1}{B}{B} — 3/3
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Golem] Rootwire Amalgam #0:0 (keyword_ability/ability): Prototype {1}{G} — 2/3
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Construct] Rust Goliath #0:0 (keyword_ability/ability): Prototype {3}{G}{G} — 3/5
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [bro 2022 | Artifact Creature — Thopter] Spotter Thopter #0:0 (keyword_ability/ability): Prototype {3}{U} — 2/3
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Angel] Steel Seraph #0:0 (keyword_ability/ability): Prototype {1}{W}{W} — 3/3
  ['punct/symbol']: prefix='Prototype {M}{M}{M}' :: [bro 2022 | Artifact Creature — Construct] Woodcaller Automaton #0:0 (keyword_ability/ability): Prototype {2}{G}{G} — 3/3
  ['clause-like']: prefix='You can never leave' :: [punk 2024 | Plane — Duskmourn] No Way Out #0:1 (replacement_effect/ability): You can never leave — If a player would planeswalk while this plane has dread counters, chaos ensues instead.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Caught in the Crossfire #0:1 (spell_or_static_text/ability): + {1} — Caught in the Crossfire deals 2 damage to each outlaw creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Caught in the Crossfire #0:2 (spell_or_static_text/ability): + {1} — Caught in the Crossfire deals 2 damage to each non-outlaw creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Dance of the Tumbleweeds #0:1 (spell_or_static_text/ability): + {1} — Search your library for a basic land card or a Desert card, put it onto the battlefield, then shuffle.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Dance of the Tumbleweeds #0:2 (spell_or_static_text/ability): + {3} — Create an X/X green Elemental creature token, where X is the number of lands you control.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Explosive Derailment #0:1 (spell_or_static_text/ability): + {2} — Explosive Derailment deals 4 damage to target creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Explosive Derailment #0:2 (spell_or_static_text/ability): + {2} — Destroy target artifact.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Final Showdown #0:1 (spell_or_static_text/ability): + {1} — All creatures lose all abilities until end of turn.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Final Showdown #0:2 (spell_or_static_text/ability): + {1} — Choose a creature you control. It gains indestructible until end of turn.
  ['punct/symbol']: prefix='+ {M}{M}{M}' :: [otj 2024 | Instant] Final Showdown #0:3 (spell_or_static_text/ability): + {3}{W}{W} — Destroy all creatures.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Getaway Glamer #0:1 (spell_or_static_text/ability): + {1} — Exile target nontoken creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Getaway Glamer #0:3 (spell_or_static_text/ability): + {2} — Destroy target creature if no other creature has greater power.
  ['punct/symbol']: prefix='+ {M}{M}' :: [otj 2024 | Instant] Great Train Heist #0:1 (spell_or_static_text/ability): + {2}{R} — Untap all creatures you control. If it's your combat phase, there is an additional combat phase after this phase.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Great Train Heist #0:2 (spell_or_static_text/ability): + {2} — Creatures you control get +1/+0 and gain first strike until end of turn.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Great Train Heist #0:3 (spell_or_static_text/ability): + {R} — Choose target opponent.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Insatiable Avarice #0:1 (spell_or_static_text/ability): + {2} — Search your library for a card, then shuffle and put that card on top.
  ['punct/symbol']: prefix='+ {M}{M}' :: [otj 2024 | Sorcery] Insatiable Avarice #0:2 (spell_or_static_text/ability): + {B}{B} — Target player draws three cards and loses 3 life.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Jailbreak Scheme #0:1 (spell_or_static_text/ability): + {3} — Put a +1/+1 counter on target creature. It can't be blocked this turn.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Jailbreak Scheme #0:2 (spell_or_static_text/ability): + {2} — Target artifact or creature's owner puts it on their choice of the top or bottom of their library.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Lively Dirge #0:1 (spell_or_static_text/ability): + {1} — Search your library for a card, put it into your graveyard, then shuffle.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Lively Dirge #0:2 (spell_or_static_text/ability): + {2} — Return up to two creature cards with total mana value 4 or less from your graveyard to the battlefield.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Metamorphic Blast #0:1 (spell_or_static_text/ability): + {1} — Until end of turn, target creature becomes a white Rabbit with base power and toughness 0/1.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Metamorphic Blast #0:2 (spell_or_static_text/ability): + {3} — Target player draws two cards.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] One Last Job #0:1 (spell_or_static_text/ability): + {2} — Return target creature card from your graveyard to the battlefield.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] One Last Job #0:2 (spell_or_static_text/ability): + {1} — Return target Mount or Vehicle card from your graveyard to the battlefield.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] One Last Job #0:3 (spell_or_static_text/ability): + {1} — Return target Aura or Equipment card from your graveyard to the battlefield attached to a creature you control.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Phantom Interference #0:1 (spell_or_static_text/ability): + {3} — Create a 2/2 white Spirit creature token with flying.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Phantom Interference #0:2 (spell_or_static_text/ability): + {1} — Counter target spell unless its controller pays {2}.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Requisition Raid #0:1 (spell_or_static_text/ability): + {1} — Destroy target artifact.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Requisition Raid #0:2 (spell_or_static_text/ability): + {1} — Destroy target enchantment.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Requisition Raid #0:3 (spell_or_static_text/ability): + {1} — Put a +1/+1 counter on each creature target player controls.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Return the Favor #0:1 (spell_or_static_text/ability): + {1} — Copy target instant spell, sorcery spell, activated ability, or triggered ability. You may choose new targets for the copy.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Return the Favor #0:2 (spell_or_static_text/ability): + {1} — Change the target of target spell or ability with a single target.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Rush of Dread #0:1 (spell_or_static_text/ability): + {1} — Target opponent sacrifices half the creatures they control of their choice, rounded up.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Rush of Dread #0:2 (spell_or_static_text/ability): + {2} — Target opponent discards half the cards in their hand, rounded up.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Rush of Dread #0:3 (spell_or_static_text/ability): + {2} — Target opponent loses half their life, rounded up.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Rustler Rampage #0:1 (spell_or_static_text/ability): + {1} — Untap all creatures target player controls.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Rustler Rampage #0:2 (spell_or_static_text/ability): + {1} — Target creature gains double strike until end of turn.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Shifting Grift #0:1 (spell_or_static_text/ability): + {2} — Exchange control of two target creatures.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Shifting Grift #0:2 (spell_or_static_text/ability): + {1} — Exchange control of two target artifacts.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Sorcery] Shifting Grift #0:3 (spell_or_static_text/ability): + {1} — Exchange control of two target enchantments.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Smuggler's Surprise #0:1 (spell_or_static_text/ability): + {2} — Mill four cards. You may put up to two creature and/or land cards from among the milled cards into your hand.
  ['punct/symbol']: prefix='+ {M}{M}' :: [otj 2024 | Instant] Smuggler's Surprise #0:2 (spell_or_static_text/ability): + {4}{G} — You may put up to two creature cards from your hand onto the battlefield.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Smuggler's Surprise #0:3 (spell_or_static_text/ability): + {1} — Creatures you control with power 4 or greater gain hexproof and indestructible until end of turn.
  ['punct/symbol']: prefix='+ {M}{M}' :: [otj 2024 | Instant] Three Steps Ahead #0:1 (spell_or_static_text/ability): + {1}{U} — Counter target spell.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Three Steps Ahead #0:2 (spell_or_static_text/ability): + {3} — Create a token that's a copy of target artifact or creature you control.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Three Steps Ahead #0:3 (spell_or_static_text/ability): + {2} — Draw two cards, then discard a card.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Trash the Town #0:1 (spell_or_static_text/ability): + {2} — Put two +1/+1 counters on target creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Trash the Town #0:2 (spell_or_static_text/ability): + {1} — Target creature gains trample until end of turn.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Trash the Town #0:3 (spell_or_static_text/ability): + {1} — Until end of turn, target creature gains "Whenever this creature deals combat damage to a player, draw two cards."
  ['punct/symbol']: prefix='+ {M}{M}' :: [otj 2024 | Instant] Unfortunate Accident #0:1 (spell_or_static_text/ability): + {2}{B} — Destroy target creature.
  ['punct/symbol']: prefix='+ {M}' :: [otj 2024 | Instant] Unfortunate Accident #0:2 (spell_or_static_text/ability): + {1} — Create a 1/1 red Mercenary creature token with "{T}: Target creature you control gets +1/+0 until end of turn. Activate only as a sorcery."
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  ['numeral']: prefix='N' :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:10 (spell_or_static_text/ability): 4 — Create two Treasure tokens *and* draw a card.
  ['punct/symbol']: prefix='Prototype {M}{M}' :: [mh3 2024 | Artifact Creature — Frog Myr] Frogmyr Enforcer #0:0 (keyword_ability/ability): Prototype {3}{R} — 2/2
  ['punct/symbol']: prefix='{M}' :: [blb 2024 | Sorcery] Season of Gathering #0:1 (spell_or_static_text/ability): {P} — Put a +1/+1 counter on a creature you control. It gains vigilance and trample until end of turn.
  ['punct/symbol']: prefix='{M}{M}' :: [blb 2024 | Sorcery] Season of Gathering #0:2 (spell_or_static_text/ability): {P}{P} — Choose artifact or enchantment. Destroy all permanents of the chosen type.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [blb 2024 | Sorcery] Season of Gathering #0:3 (spell_or_static_text/ability): {P}{P}{P} — Draw cards equal to the greatest power among creatures you control.
  ['punct/symbol']: prefix='{M}' :: [blb 2024 | Sorcery] Season of Loss #0:1 (spell_or_static_text/ability): {P} — Each player sacrifices a creature of their choice.
  ['punct/symbol']: prefix='{M}{M}' :: [blb 2024 | Sorcery] Season of Loss #0:2 (spell_or_static_text/ability): {P}{P} — Draw a card for each creature that died under your control this turn.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [blb 2024 | Sorcery] Season of Loss #0:3 (spell_or_static_text/ability): {P}{P}{P} — Each opponent loses X life, where X is the number of creature cards in your graveyard.
  ['punct/symbol']: prefix='{M}' :: [blb 2024 | Sorcery] Season of the Bold #0:1 (spell_or_static_text/ability): {P} — Create a tapped Treasure token.
  ['punct/symbol']: prefix='{M}{M}' :: [blb 2024 | Sorcery] Season of the Bold #0:2 (spell_or_static_text/ability): {P}{P} — Exile the top two cards of your library. Until the end of your next turn, you may play them.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [blb 2024 | Sorcery] Season of the Bold #0:3 (spell_or_static_text/ability): {P}{P}{P} — Until the end of your next turn, whenever you cast a spell, Season of the Bold deals 2 damage to up to one target creature.
  ['punct/symbol']: prefix='{M}' :: [blb 2024 | Sorcery] Season of the Burrow #0:1 (spell_or_static_text/ability): {P} — Create a 1/1 white Rabbit creature token.
  ['punct/symbol']: prefix='{M}{M}' :: [blb 2024 | Sorcery] Season of the Burrow #0:2 (spell_or_static_text/ability): {P}{P} — Exile target nonland permanent. Its controller draws a card.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [blb 2024 | Sorcery] Season of the Burrow #0:3 (spell_or_static_text/ability): {P}{P}{P} — Return target permanent card with mana value 3 or less from your graveyard to the battlefield with an indestructible counter on it.
  ['punct/symbol']: prefix='{M}' :: [blb 2024 | Sorcery] Season of Weaving #0:1 (spell_or_static_text/ability): {P} — Draw a card.
  ['punct/symbol']: prefix='{M}{M}' :: [blb 2024 | Sorcery] Season of Weaving #0:2 (spell_or_static_text/ability): {P}{P} — Choose an artifact or creature you control. Create a token that's a copy of it.
  ['punct/symbol']: prefix='{M}{M}{M}' :: [blb 2024 | Sorcery] Season of Weaving #0:3 (spell_or_static_text/ability): {P}{P}{P} — Return each nonland, nontoken permanent to its owner's hand.
  ['numeral']: prefix='Take N Flights of Stairs' :: [fin 2025 | Sorcery] Aerith Rescue Mission #0:2 (spell_or_static_text/mode): Take 59 Flights of Stairs — Tap up to three target creatures. Put a stun counter on one of them.
  ['long>25']: prefix="I've Come Up with a New Recipe!" :: [fin 2025 | Legendary Creature — Human Adv] Ignis Scientia #0:1 (activated_ability/ability): I've Come Up with a New Recipe! — {1}{G}{U}, {T}: Exile target card from a graveyard. If a creature card was exiled this way, create a Food token.
  ['numeral', 'comma']: prefix='N,N Needles' :: [fin 2025 | Creature — Plant] Jumbo Cactuar #0:0 (triggered_ability/ability): 10,000 Needles — Whenever this creature attacks, it gets +9999/+0 until end of turn.
  ['long>25']: prefix="Lightbringer and Hero's Shield" :: [fin 2025 | Artifact — Equipment] Paladin's Arms #0:2 (keyword_ability/ability): Lightbringer and Hero's Shield — Equip {4}
  ['comma']: prefix='Temba, His Arms Wide' :: [trk 2026 | Instant] Dathon and Picard at El-Adrel #0:1 (spell_or_static_text/mode): Temba, His Arms Wide — Target creature gets +3/+3 and gains trample until end of turn.
  ['comma', 'long>25']: prefix='Shaka, When the Walls Fell' :: [trk 2026 | Instant] Dathon and Picard at El-Adrel #0:2 (spell_or_static_text/mode): Shaka, When the Walls Fell — Each opponent sacrifices a nontoken artifact or enchantment of their choice.

## P3.4 Non-firing em-dash units (false-negative search)
units containing an em dash but no prefix: 1182; with a spaced ` — `: 16; unspaced only (keyword `Suspend N—{M}`-style): 1166
unspaced top templates: [('Choose one —', 342), ('When ~ enters, choose one —', 71), ('Choose one or both —', 53), ('Choose two —', 38), ('Suspend N—{M}{M}', 31), ('Suspend N—{M}', 25), ('Ward—Pay N life.', 24), ('Choose one or more —', 18)]

### period/colon before the dash: 12 (pool 1)
     1  'Yes, I understand my mission.'  e.g. [unk 2025 | Sorcery] Dialogue Tree #0:2 (spell_or_static_text/mode): Yes, I understand my mission. — Draw a card.
     1  'At the beginning of your end step, create a N/N black Dalek artifact creature token with m'  e.g. [who 2023 | Legendary Artifact Creature — ] Davros, Dalek Creator #0:1 (triggered_ability/ability): At the beginning of your end step, create a 3/3 black Dalek artifact creature token with menace if an opponent lost 3 or more life this turn. Then eac
     1  'Draw three cards. Then target opponent faces a villainous choice'  e.g. [who 2023 | Sorcery] Great Intelligence's Plan #0:0 (spell_or_static_text/ability): Draw three cards. Then target opponent faces a villainous choice — They discard three cards, or you may cast a spell from your hand without paying its
     1  "Choose up to four target creatures you don't control. For each of them, that creature's co"  e.g. [who 2023 | Sorcery] Hunted by The Family #0:0 (spell_or_static_text/ability): Choose up to four target creatures you don't control. For each of them, that creature's controller faces a villainous choice — That creature becomes a
     1  'I. AM. TALKING!'  e.g. [who 2023 | Legendary Creature — Time Lord] The Eleventh Doctor #0:0 (spell_or_static_text/ability): I. AM. TALKING! — Whenever The Eleventh Doctor deals combat damage to a player, you may exile a card from your hand with a number of time counters on 
     1  'Would You Like A...?'  e.g. [who 2023 | Legendary Creature — Time Lord] The Fourth Doctor #0:1 (spell_or_static_text/ability): Would You Like A...? — Once each turn, you may play a historic land or cast a historic spell from the top of your library.
     1  '−N: Heist!'  e.g. [ph23 2025 | Legendary Planeswalker — Monop] Mr. Monopoly, On the Go #0:1 (activated_ability/ability): −2: Heist! — Exile the top two cards of target opponent's library. Until end of turn, you may play those cards, and mana of any type can be spent to c
     1  '−N: Shut Down!'  e.g. [ph23 2025 | Legendary Planeswalker — Monop] Mr. Monopoly, On the Go #0:2 (activated_ability/ability): −4: Shut Down! — Destroy target artifact.
     1  '−N: Pass Go'  e.g. [ph23 2025 | Legendary Planeswalker — Monop] Mr. Monopoly, On the Go #0:3 (activated_ability/ability): −40: Pass Go — Create 200 Treasure tokens.
     1  'Throw ...'  e.g. [sld 2024 | Legendary Creature — Human Sol] Captain America, First Avenger #0:0 (activated_ability/ability): Throw ... — {3}, Unattach an Equipment from Captain America: He deals damage equal to that Equipment's mana value divided as you choose among one, two
     1  '... Catch'  e.g. [sld 2024 | Legendary Creature — Human Sol] Captain America, First Avenger #0:1 (spell_or_static_text/ability): ... Catch — At the beginning of combat on your turn, attach up to one target Equipment you control to Captain America.
     1  'At the beginning of your end step, draw a card. Then each opponent faces a villainous choi'

### pre-dash text longer than 45: 4 (pool 0)
     1  'At the beginning of your end step, each opponent faces a villainous choice'  e.g. [who 2023 | Legendary Creature — Time Lord] Missy #0:1 (triggered_ability/ability): At the beginning of your end step, each opponent faces a villainous choice — Each artifact creature you control deals 1 damage to that opponent, or yo
     1  'At the beginning of combat on your turn, each opponent faces a villainous choice'  e.g. [who 2023 | Legendary Artifact Creature — ] The Dalek Emperor #0:2 (triggered_ability/ability): At the beginning of combat on your turn, each opponent faces a villainous choice — That player sacrifices a creature of their choice, or you create a 
     1  "Target creature's owner shuffles it into their library, then faces a villainous choice"  e.g. [who 2023 | Instant] This Is How It Ends #0:0 (spell_or_static_text/ability): Target creature's owner shuffles it into their library, then faces a villainous choice — They lose 5 life, or they shuffle another creature they own i
     1  'Whenever Damocles Base deals combat damage to a player, that player faces a villainous cho'  e.g. [msc 2026 | Legendary Artifact — Vehicle] Damocles Base, Sword of Kang #0:2 (triggered_ability/ability): Whenever Damocles Base deals combat damage to a player, that player faces a villainous choice — They sacrifice a nontoken creature of their choice, or

## P3.5 Chapter units on Saga faces not classified triggered_ability
count: 0

## P3.6 Before/after kind transitions on fired units (matched by oracle_id, face, source_line, unit_text)
matched 3570, unmatched 2; transitions: {'spell_or_static_text->triggered_ability': 1786, 'spell_or_static_text->spell_or_static_text': 1036, 'activated_ability->activated_ability': 422, 'spell_or_static_text->keyword_ability': 210, 'replacement_effect->replacement_effect': 55, 'replacement_effect->triggered_ability': 36, 'spell_or_static_text->characteristic_defining_ability': 10, 'prevention_effect->triggered_ability': 5, 'prevention_effect->prevention_effect': 4, 'spell_or_static_text->replacement_effect': 2, 'spell_or_static_text->additional_cost': 2, 'spell_or_static_text->cast_restriction': 1, 'activated_ability->triggered_ability': 1}
changed: 2053; of which the routine hidden-trigger-word recovery spell_or_static_text -> triggered_ability: 1786 (not listed); every other change (non-pool):
  spell_or_static_text -> keyword_ability :: [ugl 1998 | Instant] Goblin Tutor #0:1 (keyword_ability/ability): 2 — A card named Goblin Tutor
  spell_or_static_text -> keyword_ability :: [ugl 1998 | Instant] Goblin Tutor #0:2 (keyword_ability/ability): 3 — An enchantment
  spell_or_static_text -> keyword_ability :: [ugl 1998 | Instant] Goblin Tutor #0:3 (keyword_ability/ability): 4 — An artifact
  spell_or_static_text -> keyword_ability :: [ugl 1998 | Instant] Goblin Tutor #0:4 (keyword_ability/ability): 5 — A creature
  spell_or_static_text -> keyword_ability :: [ugl 1998 | Instant] Goblin Tutor #0:5 (keyword_ability/ability): 6 — An instant or sorcery
  spell_or_static_text -> characteristic_defining_ability :: [eve 2008 | Creature — Elemental] Primalcrux #0:1 (characteristic_defining_ability/ability): Chroma — This creature's power and toughness are each equal to the number of green mana symbols in the mana costs of permanents you control.
  spell_or_static_text -> characteristic_defining_ability :: [eve 2008 | Creature — Elemental] Umbra Stalker #0:0 (characteristic_defining_ability/ability): Chroma — Umbra Stalker's power and toughness are each equal to the number of black mana symbols in the mana costs of cards in your graveyard.
  spell_or_static_text -> characteristic_defining_ability :: [con 2009 | Creature — Bird Soldier] Aven Trailblazer #0:1 (characteristic_defining_ability/ability): Domain — Aven Trailblazer's toughness is equal to the number of basic land types among lands you control.
  spell_or_static_text -> characteristic_defining_ability :: [con 2009 | Creature — Human Warrior] Matca Rioters #0:0 (characteristic_defining_ability/ability): Domain — Matca Rioters's power and toughness are each equal to the number of basic land types among lands you control.
  prevention_effect -> triggered_ability :: [ths 2013 | Creature — Human Soldier] Favored Hoplite #0:0 (triggered_ability/ability): Heroic — Whenever you cast a spell that targets this creature, put a +1/+1 counter on this creature and prevent all damage that would be dealt to it t
  prevention_effect -> triggered_ability :: [jou 2014 | Enchantment Creature — Nymph] Harvestguard Alseids #0:0 (triggered_ability/ability): Constellation — Whenever this creature or another enchantment you control enters, prevent all damage that would be dealt to target creature this turn.
  replacement_effect -> triggered_ability :: [bfz 2015 | Creature — Dragon] Akoum Hellkite #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, this creature deals 1 damage to any target. If that land is a Mountain, this creature deals 2 damage in
  replacement_effect -> triggered_ability :: [bfz 2015 | Creature — Angel] Emeria Shepherd #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, you may return target nonland permanent card from your graveyard to your hand. If that land is a Plains
  replacement_effect -> triggered_ability :: [bfz 2015 | Creature — Vampire] Guul Draz Overseer #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, other creatures you control get +1/+0 until end of turn. If that land is a Swamp, those creatures get +
  replacement_effect -> triggered_ability :: [bfz 2015 | Creature — Hydra] Oran-Rief Hydra #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, put a +1/+1 counter on this creature. If that land is a Forest, put two +1/+1 counters on this creature
  replacement_effect -> triggered_ability :: [dom 2018 | Enchantment — Saga] The Flame of Keld #0:3 (triggered_ability/ability): III — If a red source you control would deal damage to a permanent or player this turn, it deals that much damage plus 2 to that permanent or player i
  prevention_effect -> triggered_ability :: [c18 2018 | Creature — Unicorn] Loyal Unicorn #0:1 (triggered_ability/ability): Lieutenant — At the beginning of combat on your turn, if you control your commander, prevent all combat damage that would be dealt to creatures you co
  replacement_effect -> triggered_ability :: [ph18 2019 | Legendary Enchantment — Saga] The Legend of Arena #0:2 (triggered_ability/ability): III — Search your library for a planeswalker card, put it onto the battlefield, then shuffle your library. It enters with an additional loyalty counte
  spell_or_static_text -> keyword_ability :: [cmb1 2019 | Creature — Lobster] Loopy Lobster #0:2 (keyword_ability/ability): Stage 2 — Evolve
  spell_or_static_text -> keyword_ability :: [cmb1 2019 | Creature — Squid Pirate] Squidnapper #0:1 (keyword_ability/ability): Ransom — {6} and 2 life
  replacement_effect -> triggered_ability :: [znr 2020 | Creature — Insect] Scute Swarm #0:0 (triggered_ability/ability): Landfall — Whenever a land you control enters, create a 1/1 green Insect creature token. If you control six or more lands, create a token that's a cop
  replacement_effect -> triggered_ability :: [khm 2021 | Enchantment — Saga] Ascent of the Worthy #0:1 (triggered_ability/ability): I, II — Choose a creature you control. Until your next turn, all damage that would be dealt to creatures you control is dealt to that creature instead
  spell_or_static_text -> replacement_effect :: [mh2 2021 | Artifact — Vehicle] Dermotaxi #0:0 (replacement_effect/ability): Imprint — As this Vehicle enters, exile a creature card from a graveyard.
  replacement_effect -> triggered_ability :: [mh2 2021 | Creature — Giant Wizard] Prophetic Titan #0:0 (triggered_ability/ability): Delirium — When this creature enters, choose one. If there are four or more card types among cards in your graveyard, choose both instead.
  spell_or_static_text -> characteristic_defining_ability :: [mh2 2021 | Creature — Kavu] Territorial Kavu #0:0 (characteristic_defining_ability/ability): Domain — Territorial Kavu's power and toughness are each equal to the number of basic land types among lands you control.
  spell_or_static_text -> cast_restriction :: [afr 2021 | Creature — Goblin Warlock] Grim Wanderer #0:1 (cast_restriction/ability): Tragic Backstory — Cast this spell only if a creature died this turn.
  replacement_effect -> triggered_ability :: [neo 2022 | Enchantment — Saga] Kumano Faces Kakkazan // Etching of Kumano #0:2 (triggered_ability/ability): II — When you next cast a creature spell this turn, that creature enters with an additional +1/+1 counter on it.
  replacement_effect -> triggered_ability :: [snc 2022 | Creature — Elf Wizard] Rumor Gatherer #0:0 (triggered_ability/ability): Alliance — Whenever another creature you control enters, scry 1. If this is the second time this ability has resolved this turn, draw a card instead.
  spell_or_static_text -> characteristic_defining_ability :: [dmu 2022 | Creature — Cat Warrior] Nishoba Brawler #0:1 (characteristic_defining_ability/ability): Domain — Nishoba Brawler's power is equal to the number of basic land types among lands you control.
  spell_or_static_text -> characteristic_defining_ability :: [dmu 2022 | Creature — Elemental] Territorial Maro #0:0 (characteristic_defining_ability/ability): Domain — Territorial Maro's power and toughness are each equal to twice the number of basic land types among lands you control.
  replacement_effect -> triggered_ability :: [40k 2022 | Creature — Astartes Knight] Grey Knight Paragon #0:1 (triggered_ability/ability): Rites of Banishment — When this creature enters, destroy target attacking creature. If that creature is a Demon, exile it instead.
  replacement_effect -> triggered_ability :: [40k 2022 | Creature — Tyranid] Mawloc #0:1 (triggered_ability/ability): Terror from the Deep — When this creature enters, it fights up to one target creature an opponent controls. If that creature would die this turn, exil
  spell_or_static_text -> keyword_ability :: [40k 2022 | Artifact — Vehicle] Reaver Titan #0:0 (keyword_ability/ability): Void Shields — Protection from mana value 3 or less
  spell_or_static_text -> additional_cost :: [40k 2022 | Artifact Creature — Astartes D] Redemptor Dreadnought #0:0 (additional_cost/ability): Fallen Warrior — As an additional cost to cast this spell, you may exile a creature card from your graveyard.
  replacement_effect -> triggered_ability :: [40k 2022 | Creature — Astartes Warrior] Tallyman of Nurgle #0:1 (triggered_ability/ability): The Seven-fold Chant — At the beginning of your end step, if a creature died this turn, you draw a card and you lose 1 life. If seven or more creature
  replacement_effect -> triggered_ability :: [40k 2022 | Enchantment — Saga] The First Tyrannic War #0:1 (triggered_ability/ability): I — You may put a creature card from your hand onto the battlefield. If its mana cost contains {X}, it enters with a number of +1/+1 counters on it eq
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:0 (keyword_ability/ability): {TK}{TK} — Afflict 2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Flying
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Ancestral Hot Dog Minotaur #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Carnival Elephant Meteor #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Deathtouch, lifelink
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Contortionist Otter Storm #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Cool Fluffy Loxodon #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 5/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:0 (keyword_ability/ability): {TK}{TK} — Prowess, prowess
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Cursed Firebreathing Yogurt #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/8
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 4/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Deep-Fried Plague Myr #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 8/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:0 (keyword_ability/ability): {TK}{TK} — Outlast {1}
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Demonic Tourist Laser #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 9/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:0 (keyword_ability/ability): {TK}{TK} — Haste
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Eldrazi Guacamole Tightrope #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Elemental Time Flamingo #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 4/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Eternal Acrobat Toast #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/8
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Familiar Beeble Mascot #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Geek Lotus Warrior #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Giant Mana Cake #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Giant Mana Cake #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Goblin Coward Parade #0:0 (keyword_ability/ability): {TK}{TK} — Mentor
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Goblin Coward Parade #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Goblin Coward Parade #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 8/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Infect
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Happy Dead Squirrel #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Hexproof
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Misunderstood Trapeze Elf #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:0 (keyword_ability/ability): {TK}{TK} — Lifelink
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:2 (keyword_ability/ability): {TK}{TK} — 1/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Mystic Doom Sandwich #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/8
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Vigilance, reach
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Narrow-Minded Baloney Fireworks #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:0 (keyword_ability/ability): {TK}{TK} — Menace
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Persist
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Night Brushwagg Ringmaster #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK}{TK} — 10/10
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Notorious Sliver War #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Notorious Sliver War #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 9/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Undying
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Phyrexian Midway Bamboozle #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/9
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Playable Delusionary Hydra #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 4/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Primal Elder Kitty #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Primal Elder Kitty #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Sassy Gremlin Blood #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK}{TK} — 10/10
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:0 (keyword_ability/ability): {TK}{TK} — Bushido 2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Double strike
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Slimy Burrito Illusion #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 5/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Snazzy Aether Homunculus #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Spooky Clown Mox #0:0 (keyword_ability/ability): {TK}{TK} — Vigilance
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Spooky Clown Mox #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Spooky Clown Mox #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squid Fire Knight #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Protection from odd mana values
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squid Fire Knight #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squid Fire Knight #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:0 (keyword_ability/ability): {TK}{TK} — Ward {2}
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Provoke
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:2 (keyword_ability/ability): {TK}{TK}{TK} — 5/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Squishy Sphinx Ninja #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Sticky Kavu Daredevil #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Trained Blessed Mind #0:2 (keyword_ability/ability): {TK}{TK} — 4/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Trained Blessed Mind #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 6/9
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:0 (keyword_ability/ability): {TK}{TK} — Deathtouch
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Trendy Circus Pirate #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unassuming Gelatinous Serpent #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 7/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:2 (keyword_ability/ability): {TK}{TK} — 2/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unglued Pea-Brained Dinosaur #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 8/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unhinged Beast Hunt #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/6
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unique Charmed Pants #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unique Charmed Pants #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 4/8
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:1 (keyword_ability/ability): {TK}{TK}{TK}{TK} — Indestructible
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unsanctioned Ancient Juggler #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 5/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:2 (keyword_ability/ability): {TK}{TK} — 3/2
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Unstable Robot Dragon #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 2/7
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:0 (keyword_ability/ability): {TK}{TK} — Exalted, exalted
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Shadow
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Urza's Dark Cannonball #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 7/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Vampire Champion Fury #0:2 (keyword_ability/ability): {TK}{TK} — 1/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Vampire Champion Fury #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Weird Angel Flame #0:1 (keyword_ability/ability): {TK}{TK}{TK} — Protection from even mana values
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Weird Angel Flame #0:2 (keyword_ability/ability): {TK}{TK} — 2/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Weird Angel Flame #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK}{TK} — 7/8
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:2 (keyword_ability/ability): {TK}{TK} — 4/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Werewolf Lightning Mage #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 3/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:2 (keyword_ability/ability): {TK}{TK} — 5/1
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Wild Ogre Bupkis #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 7/4
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Yawgmoth Merfolk Soul #0:3 (keyword_ability/ability): {TK}{TK}{TK}{TK} — 6/5
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:0 (keyword_ability/ability): {TK}{TK} — First strike
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:2 (keyword_ability/ability): {TK}{TK} — 3/3
  spell_or_static_text -> keyword_ability :: [sunf 2022 | Stickers] Zombie Cheese Magician #0:3 (keyword_ability/ability): {TK}{TK}{TK} — 6/2
  spell_or_static_text -> keyword_ability :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:1 (keyword_ability/mode): 2 — menace
  spell_or_static_text -> keyword_ability :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:2 (keyword_ability/mode): 3 — vigilance
  spell_or_static_text -> keyword_ability :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:3 (keyword_ability/mode): 4 — lifelink
  spell_or_static_text -> keyword_ability :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:4 (keyword_ability/mode): 5 — flying
  spell_or_static_text -> keyword_ability :: [unf 2022 | Artifact Creature — Clown Robo] Celebr-8000 #0:5 (keyword_ability/mode): 6 — indestructible
  spell_or_static_text -> replacement_effect :: [unf 2022 | Artifact — Equipment] Souvenir T-Shirt #0:0 (replacement_effect/ability): Gear Up, sponsored by Wizards of the Coast — As this Equipment enters, roll two six-sided dice. For each Magic-branded item you're wearing, roll an ad
  spell_or_static_text -> keyword_ability :: [ybro 2022 | Artifact Creature — Construct] Warzone Duplicator #0:0 (keyword_ability/ability): Prototype {3}{U} — 3/3
  spell_or_static_text -> keyword_ability :: [unk 2025 | Sorcery] 17-Year Cicadas #0:1 (keyword_ability/ability): Suspend 17 — {0}
  spell_or_static_text -> keyword_ability :: [unk 2023 | Legendary Creature — Ooze] Groaaaaag, Hungry Monster #0:0 (keyword_ability/ability): Commander Suspend 4 — {B}{G}
  replacement_effect -> triggered_ability :: [unk 2024 | Legendary Creature — Human Wiz] The Clever Magician #0:0 (triggered_ability/ability): Rule Zero — Whenever this creature enters the battlefield, and before the game begins if this creature is your commander, you may propose a new rule t
  spell_or_static_text -> keyword_ability :: [unk 2025 | Legendary Creature — Phyrexian] The Multifaceted Phyrexian #0:2 (keyword_ability/ability): Fixed commander ninjutsu — {B}{B}, Discard a card
  activated_ability -> triggered_ability :: [unk 2024 | Legendary Creature — Elder Sab] The Wise Sable #0:0 (triggered_ability/ability): Judge Call! — When The Wise Sable enters the battlefield, choose one of the following six cards at random: Blood Moon; Enraging Licid; Humility; Life 
  replacement_effect -> triggered_ability :: [yone 2023 | Creature — Phyrexian Insect] Norn's Fetchling #0:1 (triggered_ability/ability): Corrupted — When Norn's Fetchling enters, conjure a card named Plains into your hand. If an opponent has three or more poison counters, you may seek a
  replacement_effect -> triggered_ability :: [mom 2023 | Enchantment — Saga] Urabrask // The Great Work #1:6 (triggered_ability/ability): III — Until end of turn, you may cast instant and sorcery spells from any graveyard. If a spell cast this way would be put into a graveyard, exile it 
  replacement_effect -> triggered_ability :: [cmm 2023 | Creature — Satyr Bard] Composer of Spring #0:0 (triggered_ability/ability): Constellation — Whenever an enchantment you control enters, you may put a land card from your hand onto the battlefield tapped. If you control six or 
  replacement_effect -> triggered_ability :: [who 2023 | Artifact — Book] River Song's Diary #0:0 (triggered_ability/ability): Imprint — Whenever a player casts an instant or sorcery spell from their hand, exile it instead of putting it into a graveyard as it resolves.
  spell_or_static_text -> characteristic_defining_ability :: [lci 2023 | Creature — Spirit] Souls of the Lost #0:1 (characteristic_defining_ability/ability): Fathomless descent — Souls of the Lost's power is equal to the number of permanent cards in your graveyard and its toughness is equal to that number p
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Wizard] Arcane Proxy #0:0 (keyword_ability/ability): Prototype {1}{U}{U} — 2/1
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Assembly-W] Autonomous Assembler #0:0 (keyword_ability/ability): Prototype {1}{W} — 2/2
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Construct] Blitz Automaton #0:0 (keyword_ability/ability): Prototype {2}{R} — 3/2
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Golem] Boulderbranch Golem #0:0 (keyword_ability/ability): Prototype {3}{G} — 3/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Construct] Combat Thresher #0:0 (keyword_ability/ability): Prototype {2}{W} — 1/1
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Golem] Cradle Clearcutter #0:0 (keyword_ability/ability): Prototype {2}{G} — 1/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Dragon] Fallaji Dragon Engine #0:0 (keyword_ability/ability): Prototype {2}{R} — 1/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Construct] Goring Warplow #0:0 (keyword_ability/ability): Prototype {1}{B} — 1/1
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Shapeshift] Hulking Metamorph #0:0 (keyword_ability/ability): Prototype {2}{U}{U} — 3/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Wurm] Iron-Craw Crusher #0:0 (keyword_ability/ability): Prototype {2}{G}{G} — 2/5
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Phyrexian ] Phyrexian Fleshgorger #0:0 (keyword_ability/ability): Prototype {1}{B}{B} — 3/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Golem] Rootwire Amalgam #0:0 (keyword_ability/ability): Prototype {1}{G} — 2/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Construct] Rust Goliath #0:0 (keyword_ability/ability): Prototype {3}{G}{G} — 3/5
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Thopter] Spotter Thopter #0:0 (keyword_ability/ability): Prototype {3}{U} — 2/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Angel] Steel Seraph #0:0 (keyword_ability/ability): Prototype {1}{W}{W} — 3/3
  spell_or_static_text -> keyword_ability :: [bro 2022 | Artifact Creature — Construct] Woodcaller Automaton #0:0 (keyword_ability/ability): Prototype {2}{G}{G} — 3/3
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Eriana, Wrecking Ball // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Frankie The Fang // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Gorra Tash and Silas // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Lyssa, Sterling Collector // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Miron Tillas Jr. // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Paq, Fleeting Filcher // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Rissa "Blades" Lee // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Sleepy Sovka // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Squeakers the Sly // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: The Outsider // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:7 (keyword_ability/ability): 1 — Create a Treasure token
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:8 (keyword_ability/ability): 2 — Create two Treasure tokens
  spell_or_static_text -> keyword_ability :: [totc 2024 | Card] Bounty: Vara Beth Hannifer // Wanted! #1:9 (keyword_ability/ability): 3 — Create two Treasure tokens *or* draw a card
  spell_or_static_text -> keyword_ability :: [mh3 2024 | Artifact Creature — Frog Myr] Frogmyr Enforcer #0:0 (keyword_ability/ability): Prototype {3}{R} — 2/2
  replacement_effect -> triggered_ability :: [mh3 2024 | Legendary Creature — Plant] The Necrobloom #0:0 (triggered_ability/ability): Landfall — Whenever a land you control enters, create a 0/1 green Plant creature token. If you control seven or more lands with different names, creat
  replacement_effect -> triggered_ability :: [fdn 2024 | Creature — Spirit] Tragic Banshee #0:0 (triggered_ability/ability): Morbid — When this creature enters, target creature an opponent controls gets -1/-1 until end of turn. If a creature died this turn, that creature get
  replacement_effect -> triggered_ability :: [j25 2024 | Creature — Cat] Scythecat Cub #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, put a +1/+1 counter on target creature you control. If this is the second time this ability has resolve
  replacement_effect -> triggered_ability :: [fic 2025 | Legendary Creature — Human Cle] Aerith, Last Ancient #0:1 (triggered_ability/ability): Raise — At the beginning of your end step, if you gained life this turn, return target creature card from your graveyard to your hand. If you gained 7
  spell_or_static_text -> keyword_ability :: [fic 2025 | Artifact — Equipment] Blue Mage's Cane #0:3 (keyword_ability/ability): Spirit of the Whalaqee — Equip {2}
  spell_or_static_text -> keyword_ability :: [fic 2025 | Artifact — Equipment] Dancer's Chakrams #0:3 (keyword_ability/ability): Krishna — Equip {3}
  spell_or_static_text -> keyword_ability :: [fic 2025 | Artifact — Equipment] Reaper's Scythe #0:3 (keyword_ability/ability): Death Sickle — Equip {2}
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Astrologian's Planisphere #0:3 (keyword_ability/ability): Diana — Equip {2}
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Bard's Bow #0:2 (keyword_ability/ability): Perseus's Bow — Equip {6}
  prevention_effect -> triggered_ability :: [fin 2025 | Enchantment Creature — Saga Co] Crystal Fragments // Summon: Alexander #1:4 (triggered_ability/ability): I, II — Prevent all damage that would be dealt to creatures you control this turn.
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Dragoon's Lance #0:3 (keyword_ability/ability): Gae Bolg — Equip {4}
  replacement_effect -> triggered_ability :: [fin 2025 | Legendary Creature — Human Sol] Lightning, Army of One #0:3 (triggered_ability/ability): Stagger — Whenever Lightning deals combat damage to a player, until your next turn, if a source would deal damage to that player or a permanent that p
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Machinist's Arsenal #0:2 (keyword_ability/ability): Machina — Equip {4}
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Ninja's Blades #0:3 (keyword_ability/ability): Mutsunokami — Equip {2}
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Paladin's Arms #0:2 (keyword_ability/ability): Lightbringer and Hero's Shield — Equip {4}
  replacement_effect -> triggered_ability :: [fin 2025 | Legendary Creature — Human Wiz] Quistis Trepe #0:0 (triggered_ability/ability): Blue Magic — When Quistis Trepe enters, you may cast target instant or sorcery card from a graveyard, and mana of any type can be spent to cast that s
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Sage's Nouliths #0:3 (keyword_ability/ability): Hagneia — Equip {3}
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Equipment] Samurai's Katana #0:2 (keyword_ability/ability): Murasame — Equip {5}
  replacement_effect -> triggered_ability :: [fin 2025 | Legendary Creature — Human Kni] Seifer Almasy #0:1 (triggered_ability/ability): Fire Cross — Whenever Seifer Almasy deals combat damage to a player, you may cast target instant or sorcery card with mana value 3 or less from your g
  replacement_effect -> triggered_ability :: [fin 2025 | Enchantment Creature — Saga Wo] Summon: Fenrir #0:2 (triggered_ability/ability): II — Heavenward Howl — When you next cast a creature spell this turn, that creature enters with an additional +1/+1 counter on it.
  spell_or_static_text -> keyword_ability :: [fin 2025 | Artifact — Book Equipment] Summoner's Grimoire #0:3 (keyword_ability/ability): Abraxas — Equip {3}
  replacement_effect -> triggered_ability :: [eoe 2025 | Creature — Insect Druid] Genemorph Imago #0:1 (triggered_ability/ability): Landfall — Whenever a land you control enters, target creature has base power and toughness 3/3 until end of turn. If you control six or more lands, t
  replacement_effect -> triggered_ability :: [tle 2025 | Enchantment — Saga] Nightmares and Daydreams #0:2 (triggered_ability/ability): IV — Draw a card. If a graveyard has twenty or more cards in it, draw three cards instead.
  spell_or_static_text -> characteristic_defining_ability :: [ecl 2026 | Creature — Elemental] Squawkroaster #0:1 (characteristic_defining_ability/ability): Vivid — Squawkroaster's power is equal to the number of colors among permanents you control.
  spell_or_static_text -> characteristic_defining_ability :: [yecl 2026 | Creature — Ouphe] Opulent Clomper #0:0 (characteristic_defining_ability/ability): Vivid — Opulent Clomper's power and toughness are each equal to the number of colors among permanents you control.
  replacement_effect -> triggered_ability :: [sos 2026 | Creature — Human Wizard] Deluge Virtuoso #0:1 (triggered_ability/ability): Opus — Whenever you cast an instant or sorcery spell, this creature gets +1/+1 until end of turn. If five or more mana was spent to cast that spell, t
  replacement_effect -> triggered_ability :: [sos 2026 | Creature — Djinn Wizard] Exhibition Tidecaller #0:0 (triggered_ability/ability): Opus — Whenever you cast an instant or sorcery spell, target player mills three cards. If five or more mana was spent to cast that spell, that player 
  replacement_effect -> triggered_ability :: [sos 2026 | Creature — Elemental Whale] Spectacular Skywhale #0:1 (triggered_ability/ability): Opus — Whenever you cast an instant or sorcery spell, this creature gets +3/+0 until end of turn. If five or more mana was spent to cast that spell, p
  replacement_effect -> triggered_ability :: [sos 2026 | Creature — Orc Sorcerer] Tackle Artist #0:1 (triggered_ability/ability): Opus — Whenever you cast an instant or sorcery spell, put a +1/+1 counter on this creature. If five or more mana was spent to cast that spell, put two
  replacement_effect -> triggered_ability :: [sos 2026 | Creature — Dwarf Bard] Thunderdrum Soloist #0:1 (triggered_ability/ability): Opus — Whenever you cast an instant or sorcery spell, this creature deals 1 damage to each opponent. If five or more mana was spent to cast that spell
  prevention_effect -> triggered_ability :: [hob 2026 | Enchantment — Saga] Old Fat Spider Can't See Me #0:2 (triggered_ability/ability): II — Prevent all damage that would be dealt by up to one target creature for as long as this Saga remains on the battlefield.
  replacement_effect -> triggered_ability :: [mbc 2026 | Enchantment] Overcooked #0:1 (triggered_ability/ability): Celebration — At the beginning of your end step, create a Food token. If two or more nonland permanents entered the battlefield under your control thi
changed in pool (count only): 143

## P4.1 Instant/sorcery-face population: 12466 printed units; top-level 10780; roles Counter({'ability': 10750, 'mode': 1289, 'delayed_trigger': 225, 'granted': 202}); top-level kinds Counter({'spell_or_static_text': 9069, 'keyword_ability': 1253, 'additional_cost': 254, 'triggered_ability': 115, 'cast_restriction': 61, 'activated_ability': 24, 'ante_instruction': 4})
top-level triggered_ability: 115 = role delayed_trigger 30 (pool 2) + role ability 85 (pool 4)
Python predicate vs dumped role disagreements: 0

## P4.2 Positives (all 30; non-pool listed) by temporal form: Counter({'this turn': 19, 'next': 10, 'this combat': 1}); by decade {'1990s': 2, '2000s': 3, '2010s': 11, '2020s': 14}; multi-face 3
  [this turn] [ice 1995 | Instant] Battle Cry #0:1 (triggered_ability/delayed_trigger): Whenever a creature blocks this turn, it gets +0/+1 until end of turn.
  [this combat] [ice 1995 | Instant] Melee #0:2 (triggered_ability/delayed_trigger): Whenever a creature attacks and isn't blocked this combat, untap it and remove it from combat.
  [this turn] [chk 2004 | Sorcery] Glimpse of Nature #0:0 (triggered_ability/delayed_trigger): Whenever you cast a creature spell this turn, draw a card.
  [this turn] zone-words=['graveyard', 'hand'] [sok 2005 | Instant — Arcane] Pure Intentions #0:0 (triggered_ability/delayed_trigger): Whenever a spell or ability an opponent controls causes you to discard cards this turn, return those cards from your graveyard to your hand.
  [this turn] [mor 2008 | Instant] Graceful Reprieve #0:0 (triggered_ability/delayed_trigger): When target creature dies this turn, return that card to the battlefield under its owner's control.
  [this turn] [dgm 2013 | Sorcery] Beck // Call #0:0 (triggered_ability/delayed_trigger): Whenever a creature enters this turn, you may draw a card.
  [this turn] [tbth 2014 | Sorcery] Consuming Rage #0:0 (triggered_ability/delayed_trigger): Whenever a Minotaur attacks this turn, it gets +2/+0 until end of turn.
  [this turn] [tbth 2014 | Sorcery] Descend on the Prey #0:0 (triggered_ability/delayed_trigger): Whenever a Minotaur attacks this turn, it gains first strike until end of turn and must be blocked this turn if able.
  [next] [ktk 2014 | Sorcery] Howl of the Horde #0:0 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.
  [this turn] [tdag 2014 | Sorcery] Impulsive Charge #0:0 (triggered_ability/delayed_trigger): At the beginning of combat this turn, all Revelers gain haste until end of turn and attack this combat if able.
  [this turn] [tbth 2014 | Sorcery] Intervention of Keranos #0:0 (triggered_ability/delayed_trigger): At the beginning of combat this turn, Intervention of Keranos deals 3 damage to each creature.
  [this turn] [tbth 2014 | Sorcery] Touch of the Horned God #0:0 (triggered_ability/delayed_trigger): Whenever a Minotaur attacks this turn, it gains deathtouch until end of turn.
  [this turn] [bfz 2015 | Sorcery] Ondu Rising #0:0 (triggered_ability/delayed_trigger): Whenever a creature attacks this turn, it gains lifelink until end of turn.
  [next] [m19 2018 | Sorcery] Doublecast #0:0 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.
  [next] [m20 2019 | Instant] Repeated Reverberation #0:0 (triggered_ability/delayed_trigger): When you next cast an instant spell, cast a sorcery spell, or activate a loyalty ability this turn, copy that spell or ability twice. You may choose n
  [next] [khm 2021 | Instant] Dual Strike #0:0 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell with mana value 4 or less this turn, copy that spell. You may choose new targets for the copy.
  [this turn] [stx 2021 | Instant] First Day of Class #0:0 (triggered_ability/delayed_trigger): Whenever a creature you control enters this turn, put a +1/+1 counter on it and it gains haste until end of turn.
  [next] [mid 2021 | Instant] Galvanic Iteration #0:0 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.
  [this turn] [stx 2021 | Sorcery] Mage Hunters' Onslaught #0:1 (triggered_ability/delayed_trigger): Whenever a creature blocks this turn, its controller loses 1 life.
  [this turn] [mid 2021 | Instant] Rite of Harmony #0:0 (triggered_ability/delayed_trigger): Whenever a creature or enchantment you control enters this turn, draw a card.
  [next] [stx 2021 | Instant] Teach by Example #0:1 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.
  [this turn] [c21 2021 | Instant] Theoretical Duplication #0:0 (triggered_ability/delayed_trigger): Whenever a nontoken creature an opponent controls enters this turn, create a token that's a copy of that creature.
  [this turn] [ncc 2022 | Sorcery] Indulge // Excess #0:0 (triggered_ability/delayed_trigger): Whenever a creature you control attacks this turn, create a 1/1 green and white Citizen creature token that's tapped and attacking.
  [next] zone-words=['hand'] [ydmu 2022 | Instant] Spellchain Scatter #0:1 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, conjure a duplicate of that spell into your hand.
  [next] [mom 2023 | Instant] Complete the Circuit #0:2 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell twice. You may choose new targets for the copies.
  [this turn] [ltc 2023 | Sorcery] Forth Eorlingas! #0:1 (triggered_ability/delayed_trigger): Whenever one or more creatures you control deal combat damage to one or more players this turn, you become the monarch.
  [this turn] [dft 2025 | Sorcery] Full Throttle #0:1 (triggered_ability/delayed_trigger): At the beginning of each combat this turn, untap all creatures that attacked this turn.
  [next] [sos 2026 | Sorcery] Pigment Wrangler // Striking Palette #1:2 (triggered_ability/delayed_trigger): When you next cast an instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.

## P4.3 Negatives (top-level I/S triggered_ability keeping role = ability): 85

### no stated duration (this turn/this combat/next): 79 (pool 4)
  other-duration=[''] [leg 1994 | Sorcery] All Hallow's Eve #0:1 (triggered_ability/ability): At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream coun
  [leg 1994 | Sorcery] Psychic Purge #0:1 (triggered_ability/ability): When a spell or ability an opponent controls causes you to discard this card, that player loses 5 life.
  [all 1996 | Instant] Death Spark #0:1 (triggered_ability/ability): At the beginning of your upkeep, if this card is in your graveyard with a creature card directly above it, you may pay {1}. If you do, return this car
  [all 1996 | Instant] Guerrilla Tactics #0:1 (triggered_ability/ability): When a spell or ability an opponent controls causes you to discard this card, it deals 4 damage to any target.
  other-duration=[''] [tmp 1997 | Instant] Ertai's Meddling #0:2 (triggered_ability/ability): At the beginning of each of that player's upkeeps, if that card is exiled, remove a delay counter from it. If the card has no delay counters on it, th
  [wth 1997 | Sorcery] Gaea's Blessing #0:2 (triggered_ability/ability): When this card is put into your graveyard from your library, shuffle your graveyard into your library.
  [ons 2002 | Instant] Choking Tethers #0:2 (triggered_ability/ability): When you cycle this card, you may tap target creature.
  other-duration=[''] [ons 2002 | Instant] Death Pulse #0:2 (triggered_ability/ability): When you cycle this card, you may have target creature get -1/-1 until end of turn.
  other-duration=[''] [ons 2002 | Sorcery] Dirge of Dread #0:2 (triggered_ability/ability): When you cycle this card, you may have target creature gain fear until end of turn.
  other-duration=[''] [ons 2002 | Instant] Primal Boost #0:2 (triggered_ability/ability): When you cycle this card, you may have target creature get +1/+1 until end of turn.
  [ons 2002 | Instant] Renewed Faith #0:2 (triggered_ability/ability): When you cycle this card, you may gain 2 life.
  other-duration=[''] [ons 2002 | Sorcery] Slice and Dice #0:2 (triggered_ability/ability): When you cycle this card, you may have it deal 1 damage to each creature.
  [ons 2002 | Instant] Solar Blast #0:2 (triggered_ability/ability): When you cycle this card, you may have it deal 1 damage to any target.
  [scg 2003 | Sorcery] Decree of Annihilation #0:2 (triggered_ability/ability): When you cycle this card, destroy all lands.
  [scg 2003 | Sorcery] Decree of Justice #0:2 (triggered_ability/ability): When you cycle this card, you may pay {X}. If you do, create X 1/1 white Soldier creature tokens.
  other-duration=[''] [scg 2003 | Sorcery] Decree of Pain #0:2 (triggered_ability/ability): When you cycle this card, all creatures get -2/-2 until end of turn.
  [scg 2003 | Instant] Decree of Savagery #0:2 (triggered_ability/ability): When you cycle this card, you may put four +1/+1 counters on target creature.
  other-duration=[''] [sok 2005 | Instant — Arcane] Death of a Thousand Stings #0:1 (triggered_ability/ability): At the beginning of your upkeep, if you have more cards in hand than each opponent, you may return this card from your graveyard to your hand.
  other-duration=[''] [sok 2005 | Sorcery] Exile into Darkness #0:1 (triggered_ability/ability): At the beginning of your upkeep, if you have more cards in hand than each opponent, you may return this card from your graveyard to your hand.
  [bok 2005 | Sorcery] Sosuke's Summons #0:1 (triggered_ability/ability): Whenever a nontoken Snake you control enters, you may return this card from your graveyard to your hand.
  other-duration=[''] [gpt 2006 | Sorcery] Benediction of Moons #0:2 (triggered_ability/ability): When the creature this card haunts dies, you gain 1 life for each player.
  [gpt 2006 | Sorcery] Cry of Contrition #0:2 (triggered_ability/ability): When the creature this card haunts dies, target player discards a card.
  other-duration=[''] [tsp 2006 | Sorcery] Curse of the Cabal #0:2 (triggered_ability/ability): At the beginning of each player's upkeep, if this card is suspended, that player may sacrifice a permanent of their choice. If the player does, put tw
  [gpt 2006 | Instant] Seize the Soul #0:2 (triggered_ability/ability): When the creature this card haunts dies, destroy target nonwhite, nonblack creature. Create a 1/1 white Spirit creature token with flying.
  [plc 2007 | Instant] Dash Hopes #0:0 (triggered_ability/ability): When you cast this spell, any player may pay 5 life. If a player does, counter Dash Hopes.
  [plc 2007 | Sorcery] Temporal Extortion #0:0 (triggered_ability/ability): When you cast this spell, any player may pay half their life, rounded up. If a player does, counter Temporal Extortion.
  [fut 2007 | Sorcery] Thunderblade Charge #0:1 (triggered_ability/ability): Whenever one or more creatures you control deal combat damage to a player, if this card is in your graveyard, you may pay {2}{R}{R}{R}. If you do, you
  [mor 2008 | Kindred Instant — Treefolk] Reach of Branches #0:1 (triggered_ability/ability): Whenever a Forest you control enters, you may return this card from your graveyard to your hand.
  [eve 2008 | Sorcery] Rekindled Flame #0:1 (triggered_ability/ability): At the beginning of your upkeep, if an opponent has no cards in hand, you may return this card from your graveyard to your hand.
  other-duration=[''] [ala 2008 | Instant] Resounding Roar #0:2 (triggered_ability/ability): When you cycle this card, target creature gets +6/+6 until end of turn.
  [ala 2008 | Sorcery] Resounding Scream #0:2 (triggered_ability/ability): When you cycle this card, target player discards two cards at random.
  [ala 2008 | Instant] Resounding Silence #0:2 (triggered_ability/ability): When you cycle this card, exile up to two target attacking creatures.
  [ala 2008 | Instant] Resounding Thunder #0:2 (triggered_ability/ability): When you cycle this card, it deals 6 damage to any target.
  [ala 2008 | Instant] Resounding Wave #0:2 (triggered_ability/ability): When you cycle this card, return two target permanents to their owners' hands.
  [zen 2009 | Instant] Punishing Fire #0:1 (triggered_ability/ability): Whenever an opponent gains life, you may pay {R}. If you do, return this card from your graveyard to your hand.
  other-duration=[''] [ogw 2016 | Instant] Kozilek's Return #0:2 (triggered_ability/ability): Whenever you cast an Eldrazi creature spell with mana value 7 or greater, you may exile this card from your graveyard. If you do, this card deals 5 da
  [aer 2017 | Sorcery] Dark Intimations #0:1 (triggered_ability/ability): When you cast a Bolas planeswalker spell, exile this card from your graveyard. That planeswalker enters with an additional loyalty counter on it.
  [akh 2017 | Instant] Deem Worthy #0:2 (triggered_ability/ability): When you cycle this card, you may have it deal 2 damage to target creature.
  [akh 2017 | Sorcery] Stir the Sands #0:2 (triggered_ability/ability): When you cycle this card, create a 2/2 black Zombie creature token.
  [hou 2017 | Sorcery] Unconventional Tactics #0:1 (triggered_ability/ability): Whenever a Zombie you control enters, you may pay {W}. If you do, return this card from your graveyard to your hand.
  other-duration=[''] [grn 2018 | Sorcery] Creeping Chill #0:1 (triggered_ability/ability): When Creeping Chill is put into your graveyard from your library, you may exile it. If you do, Creeping Chill deals 3 damage to each opponent and you 
  other-duration=['', 'game'] [c18 2018 | Sorcery] Echo Storm #0:0 (triggered_ability/ability): When you cast this spell, copy it for each time you've cast your commander from the command zone this game. You may choose new targets for the copies.
  other-duration=['', 'game'] [c18 2018 | Sorcery] Empyrial Storm #0:0 (triggered_ability/ability): When you cast this spell, copy it for each time you've cast your commander from the command zone this game.
  other-duration=['', 'game'] [c18 2018 | Instant] Fury Storm #0:0 (triggered_ability/ability): When you cast this spell, copy it for each time you've cast your commander from the command zone this game. You may choose new targets for the copies.
  other-duration=['', 'game'] [c18 2018 | Sorcery] Genesis Storm #0:0 (triggered_ability/ability): When you cast this spell, copy it for each time you've cast your commander from the command zone this game.
  other-duration=['', 'game'] [c18 2018 | Sorcery] Skull Storm #0:0 (triggered_ability/ability): When you cast this spell, copy it for each time you've cast your commander from the command zone this game.
  [eld 2019 | Instant] Banish into Fable #0:0 (triggered_ability/ability): When you cast this spell from your hand, copy it if you control an artifact, then copy it if you control an enchantment. You may choose new targets fo
  [cmb1 2019 | Instant Creature — Horse] Lightning Colt #0:0 (triggered_ability/ability): When Lightning Colt enters, it deals 3 damage to any target.
  [cmb1 2019 | Elemental Instant — Fire] Trial and Error #0:0 (triggered_ability/ability): Whenever Trial and Error is countered or fizzles, you may copy it and choose new targets for the copy.
  [cmb1 2019 | Instant Creature — Alien] Visitor from Planet Q #0:1 (triggered_ability/ability): Whenever you cast another spell with two or more card types, you may draw a card, then discard a card.
  [c20 2020 | Sorcery] Dismantling Wave #0:2 (triggered_ability/ability): When you cycle this card, destroy all artifacts and enchantments.
  [afr 2021 | Instant] Critical Hit #0:1 (triggered_ability/ability): When you roll a natural 20, return this card from your graveyard to your hand.
  [vow 2021 | Sorcery] Edgar's Awakening #0:1 (triggered_ability/ability): When you discard this card, you may pay {B}.
  other-duration=[''] [mh2 2021 | Sorcery] Fractured Sanity #0:2 (triggered_ability/ability): When you cycle this card, each opponent mills four cards.
  [stx 2021 | Sorcery] Mentor's Guidance #0:0 (triggered_ability/ability): When you cast this spell, copy it if you control a planeswalker, Cleric, Druid, Shaman, Warlock, or Wizard.
  [unf 2022 | Instant] A Good Day to Pie #0:1 (triggered_ability/ability): Whenever you put a name sticker on a creature, you may return this card from your graveyard to your hand.
  [stx 2022 | Sorcery] A-Mentor's Guidance #0:0 (triggered_ability/ability): When you cast this spell, copy it if you control a planeswalker, Cleric, Druid, Shaman, Warlock, or Wizard.
  other-duration=[''] [ncc 2022 | Instant] Storm of Forms #0:0 (triggered_ability/ability): When you cast this spell, copy it for each kind of counter among permanents you control. You may choose new targets for the copies.
  [unf 2022 | Instant] Unlawful Entry #0:1 (triggered_ability/ability): Whenever you put an art sticker on a creature, you may return this card from your graveyard to your hand.
  [lcc 2023 | Sorcery] Bygone Marvels #0:0 (triggered_ability/ability): Descend 8 — When you cast this spell, if there are eight or more permanent cards in your graveyard, copy this spell twice. You may choose new targets 
  [ltr 2023 | Instant] Council's Deliberation #0:1 (triggered_ability/ability): Whenever you scry, if you control an Island, you may exile this card from your graveyard. If you do, draw a card.
  [unk 2023 | Instant — Arcane] Nevermind #0:1 (triggered_ability/ability): When this spell resolves, discard a card. Then draw a card.
  [ylci 2023 | Instant] Radiant Smite #0:2 (triggered_ability/ability): When you cycle Radiant Smite, if you weren't the starting player, you gain 2 life.
  [unk 2023 | Instant] Ring Out #0:1 (triggered_ability/ability): Whenever an opponent casts a spell named Sol Ring, if Ring Out is in your library, you may search you library for Ring Out, and cast it without paying
  [unk 2024 | Instant] Irrefutable Evidence #0:2 (triggered_ability/ability): When you collect Irrefutable Evidence as evidence, draw a card.
  [mh3 2024 | Instant] Ugin's Binding #0:2 (triggered_ability/ability): Whenever you cast a colorless spell with mana value 7 or greater, you may exile this card from your graveyard.
  [unk 2025 | Sorcery] 17-Year Cicadas #0:2 (triggered_ability/ability): Whenever you cast a spell, if this card is suspended, remove a time counter from it.
  other-duration=[''] [unk 2025 | Legendary Instant Artifact Enc] Blue Screen of Death #0:3 (triggered_ability/ability): Whenever a player spends more seconds with priority during their turn than the number of curse counters on Blue Screen of Death, sacrifice it and they
  [msc 2026 | Sorcery] Ancestral Communion #0:0 (triggered_ability/ability): When you cast this spell while you control your commander, copy this spell. You may choose a new target for the copy.
  [msc 2026 | Sorcery] Asgardian Inspiration #0:1 (triggered_ability/ability): Whenever a source you control deals noncombat damage to an opponent, you may pay {2}. If you do, return this card from your graveyard to your hand.
  [msc 2026 | Sorcery] Endless Ranks of HYDRA #0:1 (triggered_ability/ability): Whenever your commander enters or attacks, you may pay {1}{B}. If you do, return this card from your graveyard to your hand.
  [sos 2026 | Sorcery] Killian's Confidence #0:1 (triggered_ability/ability): Whenever one or more creatures you control deal combat damage to a player, you may pay {W/B}. If you do, return this card from your graveyard to your 
  [msh 2026 | Sorcery] Photon Blast Barrage #0:0 (triggered_ability/ability): When you cast this spell, copy it X times. You may choose new targets for the copies.
  [sos 2026 | Sorcery] Social Snub #0:0 (triggered_ability/ability): When you cast this spell while you control a creature, you may copy this spell.
  [ysos 2026 | Instant] Summitfest Closing Ceremony #0:1 (triggered_ability/ability): Covercast — Whenever you cast another instant or sorcery spell, if five or more mana was spent to cast it, this card intensifies.

### excluded: off-stack evidence (keyword or ~ near zone word): 3 (pool 0)
  [mir 1996 | Instant] Mangara's Blessing #0:1 (triggered_ability/ability): When a spell or ability an opponent controls causes you to discard this card, you gain 2 life, and you return this card from your graveyard to your ha
  [ons 2002 | Instant] Sunfire Balm #0:2 (triggered_ability/ability): When you cycle this card, you may prevent the next 1 damage that would be dealt to any target this turn.
  [sok 2005 | Instant — Arcane] Pure Intentions #0:1 (triggered_ability/ability): When a spell or ability an opponent controls causes you to discard this card, return this card from your graveyard to your hand at the beginning of th

### excluded: cast/resolve trigger of the spell: 3 (pool 0)
  [c14 2014 | Instant] Malicious Affliction #0:0 (triggered_ability/ability): Morbid — When you cast this spell, if a creature died this turn, you may copy Malicious Affliction and may choose a new target for the copy.
  [stx 2021 | Instant] Show of Confidence #0:0 (triggered_ability/ability): When you cast this spell, copy it for each other instant and sorcery spell you've cast this turn. You may choose new targets for the copies.
  [sos 2026 | Instant] Lumaret's Favor #0:0 (triggered_ability/ability): Infusion — When you cast this spell, copy it if you gained life this turn. You may choose new targets for the copy.

## P4.4 False-negative sweep over every I/S-face unit not already role = delayed_trigger
candidates: 59 (pool 5); by class: {'trigger word elsewhere in spell text': 43, 'duration-first trigger (`Until end of turn, whenever ...`)': 2, 'mode child': 3, 'granted quoted ability': 2, 'sentence-initial trigger word not split by P-ARN-2 (no `this turn`/`this way` in that sentence)': 2, 'trigger word after a comma/semicolon mid-sentence': 7}

### trigger word elsewhere in spell text: 43; kinds Counter({'spell_or_static_text': 43})
  [ice 1995 | Instant] Blessed Wine #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Clairvoyance #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Force Void #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Formation #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Foxfire #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Gravebind #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [hml 1995 | Instant] Headstone #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Heal #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Infuse #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [hml 1995 | Instant] Jinx #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Lightning Blow #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Sorcery] Mind Ravel #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Panic #0:2 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Sorcery] Portent #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [hml 1995 | Sorcery] Prophecy #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Ray of Erasure #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [hml 1995 | Sorcery] Renewal #0:2 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Sorcery] Touch of Death #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [ice 1995 | Instant] Touch of Vitae #0:2 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [mir 1996 | Instant] Aleatory #0:2 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [all 1996 | Instant] Arcane Denial #0:2 (spell_or_static_text/ability): You draw a card at the beginning of the next turn's upkeep.
  [mir 1996 | Instant] Bone Harvest #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [all 1996 | Instant] Burnout #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [mir 1996 | Instant] Dazzling Beauty #0:2 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.
  [all 1996 | Instant] Fevered Strength #0:1 (spell_or_static_text/ability): Draw a card at the beginning of the next turn's upkeep.

### duration-first trigger (`Until end of turn, whenever ...`): 2; kinds Counter({'spell_or_static_text': 2})
  [ice 1995 | Sorcery] Gaze of Pain #0:0 (spell_or_static_text/ability): Until end of turn, whenever a creature you control attacks and isn't blocked, you may choose to have it deal damage equal to its power to a target cre

### mode child: 3; kinds Counter({'spell_or_static_text': 2, 'triggered_ability': 1})
  [all 1996 | Sorcery] Library of Lat-Nam #0:1 (spell_or_static_text/mode): You draw three cards at the beginning of the next turn's upkeep.
  [mir 1996 | Instant] Sapphire Charm #0:1 (spell_or_static_text/mode): Target player draws a card at the beginning of the next turn's upkeep.
  [dmu 2022 | Instant] Twinferno #0:1 (triggered_ability/mode): When you cast your next instant or sorcery spell this turn, copy that spell. You may choose new targets for the copy.

### granted quoted ability: 2; kinds Counter({'triggered_ability': 2})
  [shm 2008 | Sorcery] Tower Above #0:2 (triggered_ability/granted): When this creature attacks, target creature blocks it this turn if able.
  [hbg 2022 | Instant] Valiant Farewell #0:2 (triggered_ability/granted): When you cast your next creature spell, it perpetually gets +2/+0.

### sentence-initial trigger word not split by P-ARN-2 (no `this turn`/`this way` in that sentence): 2; kinds Counter({'spell_or_static_text': 2})
  [roe 2010 | Sorcery] World at War #0:0 (spell_or_static_text/ability): After the second main phase this turn, there's an additional combat phase followed by an additional main phase. At the beginning of that combat, untap
  [tdag 2014 | Sorcery] Impulsive Return #0:0 (spell_or_static_text/ability): Return two cards named Ecstatic Piper from Xenagos's graveyard to the battlefield. At the beginning of combat this turn, Impulsive Return deals damage

### trigger word after a comma/semicolon mid-sentence: 7; kinds Counter({'spell_or_static_text': 7})
  [tfth 2013 | Sorcery] Swallow the Hero Whole #0:0 (spell_or_static_text/ability): Each player exiles a creature they control. Until the Hydra's next turn, when a Head leaves the battlefield, return the exiled cards to the battlefiel
  [ktk 2014 | Sorcery] Howl of the Horde #0:1 (spell_or_static_text/ability): Raid — If you attacked this turn, when you next cast an instant or sorcery spell this turn, copy that spell an additional time. You may choose new tar
  [dmu 2022 | Instant] Warhost's Frenzy #0:1 (spell_or_static_text/ability): Creatures you control get +2/+0 until end of turn. If this spell was kicked, whenever a creature you control dies this turn, draw a card.
  [rex 2023 | Sorcery] Don't Move #0:0 (spell_or_static_text/ability): Destroy all tapped creatures. Until your next turn, whenever a creature becomes tapped, destroy it.
  [blb 2024 | Sorcery] Season of the Bold #0:3 (spell_or_static_text/ability): {P}{P}{P} — Until the end of your next turn, whenever you cast a spell, Season of the Bold deals 2 damage to up to one target creature.
  [dsk 2024 | Sorcery] Waltz of Rage #0:0 (spell_or_static_text/ability): Target creature you control deals damage equal to its power to each other creature. Until end of turn, whenever a creature you control dies, exile the
  [tla 2025 | Sorcery — Lesson] Ruinous Waterbending #0:1 (spell_or_static_text/ability): All creatures get -2/-2 until end of turn. If this spell's additional cost was paid, whenever a creature dies this turn, you gain 1 life.

## P4.5 I/S-face units carrying the delayed_trigger_unattached_candidate signal: 47

## P4.6 Before/after for top-level I/S triggered_ability units
transitions (pre kind, pre role) -> (post kind, post role): {'triggered_ability/ability->triggered_ability/ability': 79, 'triggered_ability/ability->triggered_ability/delayed_trigger': 30, 'spell_or_static_text/ability->triggered_ability/ability': 4, 'None/None->triggered_ability/ability': 2}
  kind changed spell_or_static_text->triggered_ability: [c14 2014 | Instant] Malicious Affliction #0:0 (triggered_ability/ability): Morbid — When you cast this spell, if a creature died this turn, you may copy Malicious Affliction and may choose a new target for the copy.
  kind changed spell_or_static_text->triggered_ability: [lcc 2023 | Sorcery] Bygone Marvels #0:0 (triggered_ability/ability): Descend 8 — When you cast this spell, if there are eight or more permanent cards in your graveyard, copy this spell twice. You may choose new targets 
  kind changed spell_or_static_text->triggered_ability: [sos 2026 | Instant] Lumaret's Favor #0:0 (triggered_ability/ability): Infusion — When you cast this spell, copy it if you gained life this turn. You may choose new targets for the copy.
  kind changed spell_or_static_text->triggered_ability: [ysos 2026 | Instant] Summitfest Closing Ceremony #0:1 (triggered_ability/ability): Covercast — Whenever you cast another instant or sorcery spell, if five or more mana was spent to cast it, this card intensifies.
pre-change top-level I/S triggered role=ability: 111; post: 85 ability + 30 delayed_trigger

## X. Binary cross-check of the Python prefix rule (`segment --text` on every fired unit and every spaced-em-dash non-firing unit)
checked 3340 top-level printed units; mismatches 0
