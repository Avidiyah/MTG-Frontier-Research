# Over-segmentation check: delayed-trigger splits (commit af150b0)

Printed units: 71682; delayed_trigger children: 982/71682
Children with resolvable parent: 982/982

## 1. Split classes
- sentence: 861/982; by decade: {'1990s': 103, '2000s': 80, '2010s': 135, '2020s': 543}
- comma: 115/982; by decade: {'1990s': 40, '2000s': 30, '2010s': 26, '2020s': 19}
- colon: 6/982; by decade: {'1990s': 2, '2000s': 2, '2010s': 1, '2020s': 1}
- other: 0/982; by decade: {}

Other-class examples (non-pool, up to 10):

## 2. Comma/colon-level parents
Distinct parent templates: 70 over 121 splits
   10  When ~ dies,
   10  When ~ attacks or blocks,
    7  When ~ enters,
    6  Whenever ~ attacks or blocks,
    4  Whenever ~ blocks a creature,
    4  Whenever ~ deals combat damage to a creature,
    3  Whenever ~ blocks or becomes blocked by a non-Wall creature,
    3  Whenever ~ blocks or becomes blocked by a nonblack creature,
    3  When ~ blocks,
    3  Whenever ~ blocks or becomes blocked by a creature,
    3  When ~ attacks,
    2  Whenever ~ becomes blocked by a creature,
    2  When a spell or ability an opponent controls causes you to discard ~,
    2  Whenever an artifact is put into your graveyard from the battlefield,
    2  Whenever a creature dies,
    2  Whenever ~ attacks,
    2  {M}, Sacrifice ~:
    1  Whenever ~ deals damage to a player,
    1  Whenever ~ becomes blocked by a Wall,
    1  Whenever ~ blocks or becomes blocked by a green or white creature,
    1  When Hazezon enters, create X N/N Sand Warrior creature tokens that are red, green,
    1  Whenever ~ becomes blocked by a non-Wall creature,
    1  Whenever enchanted creature blocks or becomes blocked by a non-Wall creature,
    1  Whenever a creature dealt damage by ~ this turn dies,
    1  When a spell or ability an opponent controls causes you to discard ~, you gain N life,
    1  Whenever a creature you control attacks,
    1  When enchanted creature attacks,
    1  Whenever ~ deals damage to a creature,
    1  When enchanted creature attacks or blocks,
    1  Whenever a creature dies, if another creature is on the battlefield,
    1  Whenever a Warrior you control deals combat damage to a creature,
    1  Whenever ~ blocks or becomes blocked by a creature with fewer letters in its name,
    1  Whenever a creature with power N or less is put into your graveyard from the battlefield,
    1  Whenever ~ blocks a creature with flying,
    1  Whenever ~ deals combat damage to a player,
    1  Whenever a +N/+N counter is removed from ~,
    1  Whenever a white creature dies,
    1  Whenever a nonwhite creature dies,
    1  Whenever ~ blocks or becomes blocked by a creature with power N or less,
    1  When four or more creatures attack you,
    1  Whenever a creature you don't control dies,
    1  Whenever a creature you control with a +N/+N counter on it dies,
    1  Whenever a creature you control fights,
    1  When a non-Angel creature you control dies,
    1  Whenever a creature enters, if it entered from your graveyard or you cast it from your graveyard,
    1  Whenever you crank ~, create two N/N blue Faerie Spy creature tokens with flying, haste, and "Whenever ~ deals combat damage to a player,
    1  Whenever ~ blocks or becomes blocked by an equipped creature,
    1  Whenever an Aura or Equipment you control is put into a graveyard from the battlefield,
    1  When ~ leaves the battlefield,
    1  Whenever one or more +N/+N counters are put on a creature you control, if Moss-Pit Skeleton is in your graveyard,
    1  Split — When Ochre Jelly dies, if it had two or more +N/+N counters on it,
    1  Split — When ~ dies, if it had two or more +N/+N counters on it,
    1  Whenever a Curse is put into your graveyard from the battlefield,
    1  Whenever another nontoken creature you control dies,
    1  When Dorothea, Vengeful Victim attacks or blocks,
    1  When you sacrifice ~,
    1  When Dorothea attacks or blocks,
    1  Whenever another multicolored creature you control dies,
    1  Whenever equipped creature dies,
    1  Whenever Slicer deals combat damage to a player,

### condition_only: 108/121
  - [lea 1993] Cockatrice: PARENT `Whenever this creature blocks or becomes blocked by a non-Wall creature,` | CHILD `destroy that creature at end of combat.`
  - [lea 1993] Thicket Basilisk: PARENT `Whenever this creature blocks or becomes blocked by a non-Wall creature,` | CHILD `destroy that creature at end of combat.`
  - [arn 1993] Nafs Asp: PARENT `Whenever this creature deals damage to a player,` | CHILD `that player loses 1 life at the beginning of their next draw step unless they pay {1} before that draw step.`
  - [arn 1993] Rukh Egg: PARENT `When this creature dies,` | CHILD `create a 4/4 red Bird creature token with flying at the beginning of the next end step.`
  - [atq 1994] Battering Ram: PARENT `Whenever this creature becomes blocked by a Wall,` | CHILD `destroy that Wall at end of combat.`
  - [leg 1994] Abomination: PARENT `Whenever this creature blocks or becomes blocked by a green or white creature,` | CHILD `destroy that creature at end of combat.`
  - [leg 1994] Infernal Medusa: PARENT `Whenever this creature blocks a creature,` | CHILD `destroy that creature at end of combat.`
  - [leg 1994] Infernal Medusa: PARENT `Whenever this creature becomes blocked by a non-Wall creature,` | CHILD `destroy that creature at end of combat.`
  - [drk 1994] Venom: PARENT `Whenever enchanted creature blocks or becomes blocked by a non-Wall creature,` | CHILD `destroy the other creature at end of combat.`
  - [ice 1995] Krovikan Fetish: PARENT `When this Aura enters,` | CHILD `draw a card at the beginning of the next turn's upkeep.`

### condition_plus: 5/121
  - [leg 1994] Hazezon Tamar: PARENT `When Hazezon enters, create X 1/1 Sand Warrior creature tokens that are red, green,` | CHILD `and white at the beginning of your next upkeep, where X is the number of lands you control at that time.`
  - [mir 1996] Mangara's Blessing: PARENT `When a spell or ability an opponent controls causes you to discard this card, you gain 2 life,` | CHILD `and you return this card from your graveyard to your hand at the beginning of the next end step.`
  - [ust 2017] Faerie Aerie: PARENT `Whenever you crank this Contraption, create two 1/1 blue Faerie Spy creature tokens with flying, haste, and "W` | CHILD `draw a card." Exile them at the beginning of the next end step.`
  - [vow 2023] A-Dorothea, Vengeful Victim // A-Dorothea's Retribution: PARENT `When Dorothea, Vengeful Victim attacks or blocks,` | CHILD `sacrifice it at end of combat.`
  - [fin 2025] Firion, Wild Rose Warrior: PARENT `Whenever a nontoken Equipment you control enters, create a token that's a copy of it,` | CHILD `except it has "This Equipment's equip abilities cost {2} less to activate." Sacrifice that token at the beginn`

### cost_only: 5/121
  - [vis 1997] Giant Caterpillar: PARENT `{G}, Sacrifice this creature:` | CHILD `Create a 1/1 green Insect creature token with flying named Butterfly at the beginning of the next end step.`
  - [mmq 1999] Silent Assassin: PARENT `{3}{B}:` | CHILD `Destroy target blocking creature at end of combat.`
  - [sok 2005] Sakashima the Impostor: PARENT `{2}{U}{U}:` | CHILD `Return Sakashima the Impostor to its owner's hand at the beginning of the next end step.`
  - [cmm 2023] Vronos, Masked Inquisitor: PARENT `+1:` | CHILD `Up to two other target planeswalkers you control phase out at the beginning of the next end step.`

### effect_fragment: 3/121
  - [afr 2022] A-Ochre Jelly: PARENT `Split — When Ochre Jelly dies, if it had two or more +1/+1 counters on it,` | CHILD `create a token that's a copy of it at the beginning of the next end step. The token enters with half that many`
  - [afr 2021] Ochre Jelly: PARENT `Split — When this creature dies, if it had two or more +1/+1 counters on it,` | CHILD `create a token that's a copy of it at the beginning of the next end step. The token enters with half that many`
  - [ph17 2018] Diabolical Salvation: PARENT `Create four 4/4 red Devil creature tokens with haste and "When this creature dies, create a colorless Treasure` | CHILD `Add one mana of any color.'" Sacrifice the Devil tokens at the beginning of the next end step.`

## 3. Judgement sample (40 comma/colon splits)
  - [vow 2023] A-Dorothea, Vengeful Victim // A-Dorothea's Retribution: PARENT `When Dorothea, Vengeful Victim attacks or blocks,` | CHILD `sacrifice it at end of combat.`
  - [afr 2022] A-Ochre Jelly: PARENT `Split — When Ochre Jelly dies, if it had two or more +1/+1 counters on it,` | CHILD `create a token that's a copy of it at the beginning of the next end step. The token enters with half that many +1/+1 counters on it, rounded`
  - [leg 1994] Abomination: PARENT `Whenever this creature blocks or becomes blocked by a green or white creature,` | CHILD `destroy that creature at end of combat.`
  - [ohop 2009] Agyrem: PARENT `Whenever a white creature dies,` | CHILD `return it to the battlefield under its owner's control at the beginning of the next end step.`
  - [bbd 2018] Arcane Artisan: PARENT `When this creature leaves the battlefield,` | CHILD `exile all tokens created with it at the beginning of the next end step.`
  - [roe 2010] Arrogant Bloodlord: PARENT `Whenever this creature blocks or becomes blocked by a creature with power 1 or less,` | CHILD `destroy this creature at end of combat.`
  - [atq 1994] Battering Ram: PARENT `Whenever this creature becomes blocked by a Wall,` | CHILD `destroy that Wall at end of combat.`
  - [all 1996] Bestial Fury: PARENT `When this Aura enters,` | CHILD `draw a card at the beginning of the next turn's upkeep.`
  - [con 2009] Brackwater Elemental: PARENT `When this creature attacks or blocks,` | CHILD `sacrifice it at the beginning of the next end step.`
  - [all 1996] Carrier Pigeons: PARENT `When this creature enters,` | CHILD `draw a card at the beginning of the next turn's upkeep.`
  - [wth 1997] Cinder Wall: PARENT `When this creature blocks,` | CHILD `destroy it at end of combat.`
  - [mrd 2003] Clockwork Condor: PARENT `Whenever this creature attacks or blocks,` | CHILD `remove a +1/+1 counter from it at end of combat.`
  - [mrd 2003] Clockwork Vorrac: PARENT `Whenever this creature attacks or blocks,` | CHILD `remove a +1/+1 counter from it at end of combat.`
  - [emn 2016] Conduit of Storms // Conduit of Emrakul: PARENT `Whenever this creature attacks,` | CHILD `add {R} at the beginning of your next main phase this turn.`
  - [xln 2017] Conqueror's Galleon // Conqueror's Foothold: PARENT `When this Vehicle attacks,` | CHILD `exile it at end of combat, then return it to the battlefield transformed under your control.`
  - [dom 2018] Corrosive Ooze: PARENT `Whenever this creature blocks or becomes blocked by an equipped creature,` | CHILD `destroy all Equipment attached to that creature at end of combat.`
  - [exo 1998] Cunning: PARENT `When enchanted creature attacks or blocks,` | CHILD `sacrifice this Aura at the beginning of the next cleanup step.`
  - [tc14 2014] Daretti, Scrap Savant Emblem: PARENT `Whenever an artifact is put into your graveyard from the battlefield,` | CHILD `return that card to the battlefield at the beginning of the next end step.`
  - [m3c 2024] Desert Warfare: PARENT `Whenever you sacrifice a Desert and whenever a Desert card is put into your graveyard from your hand or library,` | CHILD `put that card onto the battlefield under your control at the beginning of your next end step.`
  - [vow 2021] Dorothea, Vengeful Victim // Dorothea's Retribution: PARENT `When Dorothea attacks or blocks,` | CHILD `sacrifice it at end of combat.`
  - [ust 2017] Faerie Aerie: PARENT `Whenever you crank this Contraption, create two 1/1 blue Faerie Spy creature tokens with flying, haste, and "Whenever th` | CHILD `draw a card." Exile them at the beginning of the next end step.`
  - [fin 2025] Firion, Wild Rose Warrior: PARENT `Whenever a nontoken Equipment you control enters, create a token that's a copy of it,` | CHILD `except it has "This Equipment's equip abilities cost {2} less to activate." Sacrifice that token at the beginning of the next upkeep.`
  - [wth 1997] Fog Elemental: PARENT `When this creature attacks or blocks,` | CHILD `sacrifice it at end of combat.`
  - [cmm 2023] Ghoulish Impetus: PARENT `When enchanted creature dies,` | CHILD `return this card to the battlefield at the beginning of the next end step.`
  - [tsp 2006] Glass Asp: PARENT `Whenever this creature deals combat damage to a player,` | CHILD `that player loses 2 life at the beginning of their next draw step unless they pay {2} before that step.`
  - [rtr 2012] Grave Betrayal: PARENT `Whenever a creature you don't control dies,` | CHILD `return it to the battlefield under your control with an additional +1/+1 counter on it at the beginning of the next end step. That creature `
  - [leg 1994] Infernal Medusa: PARENT `Whenever this creature blocks a creature,` | CHILD `destroy that creature at end of combat.`
  - [all 1996] Ivory Gargoyle: PARENT `When this creature dies,` | CHILD `return it to the battlefield under its owner's control at the beginning of the next end step and you skip your next draw step.`
  - [pcy 2000] Keldon Battlewagon: PARENT `When this creature attacks,` | CHILD `sacrifice it at end of combat.`
  - [all 1996] Krovikan Plague: PARENT `When this Aura enters,` | CHILD `draw a card at the beginning of the next turn's upkeep.`
  - [plst 2024] Liliana, Defiant Necromancer Emblem: PARENT `Whenever a creature dies,` | CHILD `return it to the battlefield under your control at the beginning of the next end step.`
  - [sth 1998] Lowland Basilisk: PARENT `Whenever this creature deals damage to a creature,` | CHILD `destroy that creature at end of combat.`
  - [mic 2021] Lynde, Cheerful Tormentor: PARENT `Whenever a Curse is put into your graveyard from the battlefield,` | CHILD `return it to the battlefield attached to you at the beginning of the next end step.`
  - [cns 2014] Marchesa, the Black Rose: PARENT `Whenever a creature you control with a +1/+1 counter on it dies,` | CHILD `return that card to the battlefield under your control at the beginning of the next end step.`
  - [plc 2007] Molten Firebird: PARENT `When this creature dies,` | CHILD `return it to the battlefield under its owner's control at the beginning of the next end step and you skip your next draw step.`
  - [oarc 2010] Nature Shields Its Own: PARENT `When four or more creatures attack you,` | CHILD `abandon this scheme at end of combat.`
  - [afr 2021] Ochre Jelly: PARENT `Split — When this creature dies, if it had two or more +1/+1 counters on it,` | CHILD `create a token that's a copy of it at the beginning of the next end step. The token enters with half that many +1/+1 counters on it, rounded`
  - [ody 2001] Phantom Whelp: PARENT `When this creature attacks or blocks,` | CHILD `return it to its owner's hand at end of combat.`
  - [soi 2016] Prized Amalgam: PARENT `Whenever a creature enters, if it entered from your graveyard or you cast it from your graveyard,` | CHILD `return this card from your graveyard to the battlefield tapped at the beginning of the next end step.`
  - [sok 2005] Pure Intentions: PARENT `When a spell or ability an opponent controls causes you to discard this card,` | CHILD `return this card from your graveyard to your hand at the beginning of the next end step.`

## 4. Children beginning lowercase or with and/or/then
Count: 115/982; starting with and/or/then: 2
   18  destroy that creature at end of combat.
    8  sacrifice it at end of combat.
    7  draw a card at the beginning of the next turn's upkeep.
    5  return it to its owner's hand at the beginning of the next e
    4  return it to its owner's hand at end of combat.
    4  remove a +N/+N counter from it at end of combat.
    3  return it to the battlefield under its owner's control at th
    3  destroy it at end of combat.
    3  return that creature to its owner's hand at end of combat.
    2  that player loses N life at the beginning of their next draw
    2  put that card onto the battlefield under your control at the
    2  return ~ from your graveyard to your hand at the beginning o
Examples (non-pool, up to 15, preferring and/or/then):
  - [leg 1994] Hazezon Tamar: PARENT `When Hazezon enters, create X 1/1 Sand Warrior creature tokens that are red, green,` | CHILD `and white at the beginning of your next upkeep, where X is the number of lands you control at that time.`
  - [mir 1996] Mangara's Blessing: PARENT ` a spell or ability an opponent controls causes you to discard this card, you gain 2 life,` | CHILD `and you return this card from your graveyard to your hand at the beginning of the next end step.`
  - [vow 2023] A-Dorothea, Vengeful Victim // A-Dorothea's Retribution: PARENT `When Dorothea, Vengeful Victim attacks or blocks,` | CHILD `sacrifice it at end of combat.`
  - [znr 2022] A-Moss-Pit Skeleton: PARENT `/+1 counters are put on a creature you control, if Moss-Pit Skeleton is in your graveyard,` | CHILD `return A-Moss-Pit Skeleton from your graveyard to your hand at the beginning of the next end step.`
  - [afr 2022] A-Ochre Jelly: PARENT `Split — When Ochre Jelly dies, if it had two or more +1/+1 counters on it,` | CHILD `create a token that's a copy of it at the beginning of the next end step. The token enters with half that many`
  - [tla 2025] Aang, at the Crossroads // Aang, Destined Savior: PARENT `When another creature you control leaves the battlefield,` | CHILD `transform Aang at the beginning of the next upkeep.`
  - [leg 1994] Abomination: PARENT `Whenever this creature blocks or becomes blocked by a green or white creature,` | CHILD `destroy that creature at end of combat.`
  - [plc 2007] Aether Membrane: PARENT `Whenever this creature blocks a creature,` | CHILD `return that creature to its owner's hand at end of combat.`
  - [ohop 2009] Agyrem: PARENT `Whenever a white creature dies,` | CHILD `return it to the battlefield under its owner's control at the beginning of the next end step.`
  - [ohop 2009] Agyrem: PARENT `Whenever a nonwhite creature dies,` | CHILD `return it to its owner's hand at the beginning of the next end step.`
  - [bbd 2018] Arcane Artisan: PARENT `When this creature leaves the battlefield,` | CHILD `exile all tokens created with it at the beginning of the next end step.`
  - [soi 2016] Archangel Avacyn // Avacyn, the Purifier: PARENT `When a non-Angel creature you control dies,` | CHILD `transform Archangel Avacyn at the beginning of the next upkeep.`
  - [roe 2010] Arrogant Bloodlord: PARENT `Whenever this creature blocks or becomes blocked by a creature with power 1 or less,` | CHILD `destroy this creature at end of combat.`
  - [mir 1996] Basalt Golem: PARENT `Whenever this creature becomes blocked by a creature,` | CHILD `that creature's controller sacrifices it at end of combat. If the player does, they create a 0/2 colorless Wal`
  - [atq 1994] Battering Ram: PARENT `Whenever this creature becomes blocked by a Wall,` | CHILD `destroy that Wall at end of combat.`

## 5. Sentence-level sample (30 across decades)
  - [lea 1993] Berserk: CHILD `At the beginning of the next end step, destroy that creature if it attacked this turn.`
  - [ice 1995] Kjeldoran Guard: CHILD `When that creature leaves the battlefield this turn, sacrifice this creature.`
  - [mir 1996] Armor of Thorns: CHILD `If you cast it any time a sorcery couldn't have been cast, the controller of the permanent it becomes sacrifices it at the beginning of the next clean`
  - [tmp 1997] Echo Chamber: CHILD `Exile the token at the beginning of the next end step.`
  - [apc 2001] Suppress: CHILD `At the beginning of the end step of that player's next turn, that player returns those cards to their hand.`
  - [bok 2005] Goryo's Vengeance: CHILD `Exile it at the beginning of the next end step.`
  - [lrw 2007] Galepowder Mage: CHILD `Return that card to the battlefield under its owner's control at the beginning of the next end step.`
  - [isd 2011] Geist of Saint Traft: CHILD `Exile that token at end of combat.`
  - [c14 2014] Wake the Dead: CHILD `Sacrifice those creatures at the beginning of the next end step.`
  - [akh 2017] Emberhorn Minotaur: CHILD `When you do, it gets +1/+1 and gains menace until end of turn.`
  - [dom 2018] Oath of Teferi: CHILD `Return it to the battlefield under its owner's control at the beginning of the next end step.`
  - [m20 2019] Cavalier of Night: CHILD `When you do, destroy target creature an opponent controls.`
  - [iko 2020] Yorion, Sky Nomad: CHILD `Return those cards to the battlefield at the beginning of the next end step.`
  - [c21 2021] Surge to Victory: CHILD `Whenever a creature you control deals combat damage to a player this turn, copy the exiled card. You may cast the copy without paying its mana cost.`
  - [vow 2021] Undead Butler: CHILD `When you do, return target creature card from your graveyard to your hand.`
  - [ncc 2022] Next of Kin: CHILD `If you do, return this card to the battlefield attached to that creature at the beginning of the next end step.`
  - [hbg 2022] Ambergris, Agent of Tyranny: CHILD `When you do, target creature an opponent controls gets -X/-X until end of turn, where X is the number of cards you've discarded this turn.`
  - [dmu 2022] Yotia Declares War: CHILD `When you do, this Saga deals that much damage to target creature or planeswalker.`
  - [j22 2022] Daring Piracy: CHILD `Exile it at the beginning of the next end step.`
  - [mom 2023] Guardian of Ghirapur: CHILD `Return it to the battlefield under its owner's control at the beginning of the next end step.`
  - [ltr 2023] Old Man Willow: CHILD `When you do, target creature an opponent controls gets -2/-2 until end of turn.`
  - [who 2023] Regenerations Restored: CHILD `When you do, take an extra turn after this one.`
  - [unk 2024] Guild Pact: CHILD `At the beginning of your next upkeep, pay one mana of each of the chosen colors. If you don't, you lose the game.`
  - [mh3 2024] Hydra Trainer: CHILD `When you do, target creature gets +X/+X until end of turn, where X is the number of counters on permanents you control.`
  - [mb2 2024] Sigardian Evangel: CHILD `Discard that card at the beginning of the next end step.`
  - [fdn 2024] Kykar, Zephyr Awakener: CHILD `Return that card to the battlefield under its owner's control at the beginning of the next end step.`
  - [tdm 2025] New Way Forward: CHILD `When damage is prevented this way, New Way Forward deals that much damage to that source's controller and you draw that many cards.`
  - [eoe 2025] Kav Landseeker: CHILD `At the beginning of the end step on your next turn, sacrifice that token.`
  - [tla 2025] Teo, Spirited Glider: CHILD `When you discard a nonland card this way, put a +1/+1 counter on target creature you control.`
  - [tmt 2026] Old Hob, Alleycat Blues: CHILD `Destroy it at the beginning of the next end step.`

## 6. Residual misses
Count: 93/71682
  of which phrase at unit start (top-level trigger, expected not split): 14
  by kind: {'triggered_ability': 20, 'spell_or_static_text': 69, 'replacement_effect': 1, 'activated_ability': 3}; by role: {'ability': 89, 'mode': 2, 'granted': 2}
  - [leg 1994] Time Elemental [triggered_ability/ability]: `When this creature attacks or blocks, at end of combat, sacrifice it and it deals 5 damage to you.`
  - [ice 1995] Blessed Wine [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Clairvoyance [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Force Void [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Formation [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Foxfire [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Gravebind [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Heal [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Infuse [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Lightning Blow [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Mind Ravel [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Panic [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`

## 7. Recurring `At end of combat, if ...` top-level triggers
Count: 7; role=ability: 5; with delayed child: 0
  - [lea 1993] Clockwork Beast: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [atq 1994] Clockwork Avian: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [hml 1995] Clockwork Steed: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [hml 1995] Clockwork Swarm: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [all 1996] Kjeldoran Home Guard: `At end of combat, if this creature attacked or blocked this combat, put a -0/-1 counter on this creature and create a 0/`
All top-level units starting `At end of combat`: 13

---
Sections 1–7 above are the verbatim output of
`scripts/python/corpus_checks/check_delayed_split.py` on the 2026-08-26
corpus dump at commit `af150b0`; sections 8–9 below are the reviewing agent's
manual judgement of the samples and were appended by hand.

## 8. Judgements (fork-pass, single reviewer)

Sample of 40 comma/colon splits (§3): child text is a delayed triggered ability
created by the parent's effect (CR 603.7a/e) in 38/40; the remaining 2 (Faerie
Aerie ust 2017, Firion fin 2025) are mangled because the split comma lies inside a
quoted granted ability — `delayed_trigger_split` finds the split with `rfind` on
the unmasked `text` (src/main.rs ~line 1049) rather than on `masked`. Same defect:
Diabolical Salvation (ph17 2018). Parent fragment is a reference unit on its own in
0/40: 108/121 comma/colon parents are a bare trigger condition (`[When/Whenever/At
...],` — CR 113.3c gives a triggered ability as `[Trigger condition], [effect]`, so
the condition alone is a slot, not an ability), 5/121 a bare cost (`{...}:`,
`+1:` — CR 602.1a), 5/121 a condition plus a partial effect (comma inside a list or
compound effect: Hazezon Tamar leg 1994, Mangara's Blessing mir 1996, the two
in-quote cases, Dorothea vow 2023 which is condition-only with a comma in the
name), 3/121 ability-word/quote fragments.

Sentence-level sample (§5): 30/30 children are delayed triggered abilities
(603.7) or reflexive triggered abilities (603.12). None is an independent trigger,
reminder text, or keyword text.

Residual misses (§6): 93/71682 units still carry a supported phrase outside a
delayed_trigger child; 14 are top-level triggers that start with the phrase
(correctly unsplit); 69 are whole-line spell text such as the Ice Age cantrip line
`Draw a card at the beginning of the next turn's upkeep.` — the entire unit is the
delayed-trigger creation, there is nothing to split off, and under the protocol the
unit is one spell ability; the remaining ~10 are triggers whose phrase follows the
condition comma directly (`When ~ attacks or blocks, at end of combat, sacrifice
it ...`, Time Elemental leg 1994) or multi-clause forms.

Recurring `At end of combat, if ...` triggers (§7): 7 units, 5 top-level, 0 split
(negative test holds). 13 top-level units begin `At end of combat`.

## 9. Recommended revision of rule (c) — generic, no card names

Keep (a) and (b) (sentence-level). Replace (c) with: within a single sentence, a
delayed-trigger phrase is split into a child only if the text before the split
point is a complete effect clause — i.e. the split point is not the comma that
closes a leading `When/Whenever/At ...` trigger condition (113.3c), not the colon
that closes an activation cost (602.1a), and never a comma or colon inside a quoted
ability (mask before `rfind`). When no such split point exists, do not split;
emit the unit whole and record the delayed-trigger creation as an in-unit slot or
signal (protocol T8 / `delayed_trigger_unattached_candidate`), so the boundary
stays one ability whose effect creates the delayed trigger. Expected effect:
113/121 comma/colon splits revert to single units (982 → ~869 delayed children),
the 3 in-quote mis-splits disappear, and top-level templates no longer include
condition-only fragments (`When ~ dies,` ×10, `When ~ attacks or blocks,` ×10 in
the corpus top-5000).
