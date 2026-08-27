# Over-segmentation check: delayed-trigger splits (commit bf9eb04)

Printed units: 71563; delayed_trigger children: 861/71563
Children with resolvable parent: 861/861

## 1. Split classes
- sentence: 861/861; by decade: {'1990s': 103, '2000s': 80, '2010s': 135, '2020s': 543}
- comma: 0/861; by decade: {}
- colon: 0/861; by decade: {}
- other: 0/861; by decade: {}

Other-class examples (non-pool, up to 10):

## 2. Comma/colon-level parents
Distinct parent templates: 0 over 0 splits

### condition_only: 0/0

### condition_plus: 0/0

### cost_only: 0/0

### effect_fragment: 0/0

## 3. Judgement sample (40 comma/colon splits)

## 4. Children beginning lowercase or with and/or/then
Count: 0/861; starting with and/or/then: 0
Examples (non-pool, up to 15, preferring and/or/then):

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
Count: 206/71563
  of which phrase at unit start (top-level trigger, expected not split): 14
  by kind: {'triggered_ability': 125, 'spell_or_static_text': 70, 'activated_ability': 8, 'replacement_effect': 3}; by role: {'ability': 198, 'mode': 2, 'granted': 6}
  - [lea 1993] Cockatrice [triggered_ability/ability]: `Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat.`
  - [lea 1993] Thicket Basilisk [triggered_ability/ability]: `Whenever this creature blocks or becomes blocked by a non-Wall creature, destroy that creature at end of combat.`
  - [arn 1993] Rukh Egg [triggered_ability/ability]: `When this creature dies, create a 4/4 red Bird creature token with flying at the beginning of the next end step.`
  - [atq 1994] Battering Ram [triggered_ability/ability]: `Whenever this creature becomes blocked by a Wall, destroy that Wall at end of combat.`
  - [leg 1994] Abomination [triggered_ability/ability]: `Whenever this creature blocks or becomes blocked by a green or white creature, destroy that creature at end of combat.`
  - [leg 1994] Infernal Medusa [triggered_ability/ability]: `Whenever this creature blocks a creature, destroy that creature at end of combat.`
  - [leg 1994] Infernal Medusa [triggered_ability/ability]: `Whenever this creature becomes blocked by a non-Wall creature, destroy that creature at end of combat.`
  - [leg 1994] Time Elemental [triggered_ability/ability]: `When this creature attacks or blocks, at end of combat, sacrifice it and it deals 5 damage to you.`
  - [drk 1994] Venom [triggered_ability/ability]: `Whenever enchanted creature blocks or becomes blocked by a non-Wall creature, destroy the other creature at end of combat.`
  - [ice 1995] Blessed Wine [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Clairvoyance [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`
  - [ice 1995] Force Void [spell_or_static_text/ability]: `Draw a card at the beginning of the next turn's upkeep.`

## 7. Recurring `At end of combat, if ...` top-level triggers
Count: 7; role=ability: 5; with delayed child: 0
  - [lea 1993] Clockwork Beast: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [atq 1994] Clockwork Avian: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [hml 1995] Clockwork Steed: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [hml 1995] Clockwork Swarm: `At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.`
  - [all 1996] Kjeldoran Home Guard: `At end of combat, if this creature attacked or blocked this combat, put a -0/-1 counter on this creature and create a 0/`
All top-level units starting `At end of combat`: 13
