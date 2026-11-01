
Theme: Alchemy

Planning on using a grid-based crafting table similar to this concept:![[Pasted image 20261101113117.png]]

We are planning on making currently a tower defense game, where each "tower" can be constructed via an unknown resource, where each is occupied by a person/monkey that when given a "potion" they then use to 'throw' or use against incoming opponents.

Additionally, there should be a delay in handing a person a potion, and also the crafting table will be to the right of the battle to create "stress" or "suspension", causing a delay between a created potion and it's effect.

Additionally, the potions can be crafted, and then mixed. Potions can have various different types (fire, poison, electricity, slowness/stick, etc.) and if you grab 2 fire potions and mix them, then you get a stronger fire potion, or if you have a fire and poison potion, then you can have a mixed effect.

Theme of enemies or people is unchosen. Creation of towers is yet to be decided on. Creation of additional features/traps undecided. Creation of source is undecided.

Basic assumptions: When killing a creature, we "harvest" their guts for alchemical supplies.

We could go as simple as a few towers exist and we cannot create any, but when enough creatures die, we can use their 'remains' in the crafting table with crafting recipes?

Or it could be basic, drag their corpse's, turn it into corpse starch, and then use that to make a potion. Or it could be as simple as "enemy die? get mana, use mana, big boom!"

Different types of potions we can make:

1) Fire Potion: Can either be represented by either DoT, especially if we end up going "resistances" on this tower defense game and allow enemies to have resistances to different types of elemental/alchemical damage), or or AoE
   
2) Lightning Potion: Can either be represented by either Chain damage (arcs to 2 additional enemies with 50% original damage) and again resistance to electric/shock/lightning
   
3) Poison Potion: The OG DoT Potion, can also be AoE but would rather this one be single target
   
4) Acid Potion: Can deal some DoT but more importantly, I think this should "rust" enemy armor and decrease the enemies defensive stat.
   
5) Frost Potion: Slows enemy, deals less damage, can be coded to do AoE
   
6) Gunpowder/Explosive: Big AoE Flat Damage
   
7) Gold Potion: Deals minor damage (80-90% of base) but amplifies mana or 'gold' gained from enemy corpses
   
8) Elixir/Panacea Potion: Buffs nearby towers (adds a flat 20% bonus in a set range to nearby towers)
   
9) Void Potion: Expensive but negates resistances of enemies

10) Mercury Potion: Needs to be balanced VERY carefully, makes enemies slip and fallback a percentage of their movement, or can be placed as a trap permanently slowing enemies that traverse the ground (we can add air units if wanted to avoid this). Cannot be upgraded.


## Damage Calculation:

I think a simple Base 6-10 damage range for level 1 potions is enough. Then for each additional level, increase damage of previous base by 50% (so scales, but also easy to code in)

Base 


Creation/Mixing Potions:

We could do a range system where depending on what percentage of each potion is "mixed" we can have different effects.

Each effect can be categorized, so for example: If you mix a fire potion and a poison potion, you get 50% of each effect.

However if you mix a Level 2 Fire Potion with a level 2 fire and poison potion, you get 75% Fire and 25% poison. This will make the game interesting by making AoE poison possible.

We should never allow any number go below 25%, so if you mix a level 3 Fire/Poison/Frost/Gold potion with a level 3 Fire/Poison/Acid/Lightning potion, we should either:
	a) Reject the mix
	b) Mix: find the common ones and increase those and decrease the weaker dilutions
	c) Mix, but just keep the effects of the first potion chosen (i.e. override the second)

I do not believe we should allow potions to be mixed to an 1/8th strength (a Fire/Lightning/Poison/Acid/Frost/Gunpower/Gold/Void potion sounds broken)


## Creation of Towers 

Towers can be created on any 'neutral' ground that is in each map. If it is a snowy biome terrain, it will be the default "snowy" terrain that can be placed anywhere (And should cover the majority of the map), if it is dungeon, the basic stone terrain will do.

When placing a tower, we should have a red overlay apply to any tiles that you CANNOT place a tower on, to help guide the player to easily and quickly judge for themselves the best location to place a tower for their specific potion/idea.

## Creation of Maps

We should make 8-10 maps to function as our story that gets progressively more difficult.

The first one should act as the tutorial where the player is introduced to the 3 basic potions: Fire, Lightning and Poison. It should have 6 waves of enemies with 1 mini-boss wave.

Map 2 should introduce Acid and Frost and have 8 waves with 2 mini-boss waves.

Map 3 should introduce Gunpowder and Mercury. and have 9 waves with 1 mini-boss and 1 boss wave.

Map 4 should introduce Gold and Elixir. 11 waves, 2 mini boss and 1 boss wave.

Map 5 should introduce Void. 11 waves, 2 mini boss and 1 boss wave.

Map 6 will be the first "true test of the player", while difficulty will be ramping up, we jump up to 15 waves, 3 mini bosses and 2 bosses.

Map 7 will be a breather, 12 waves, 2 mini bosses and 1 boss.

Map 8 will be the first (or last) "Epic" mission with 20 waves, 4 mini boss waves, 2 boss waves and 1 epic boss.


## Troops

Troops will operate off of a "base" scale, where each stat follows a base value, and then scales a percentage based off of the wave counter.

Example: 

The Base Orc has 25 hp, if he spawns on wave 1 that is. On wave 2 he gets a 1.1* (10% increase) multiplier. All multipliers can be rounded up or down (doesn't matter, for now let's round down) so he goes up to 27 hp. 

His base speed is 1 unit of movement. The orc has a 10% increase of speed every 3 waves, so on Wave 2 he will still be 1.0 speed, but on Wave 3 he will have 1.1 speed. 

Finally, each Orc has armor. He starts with 2 armor. Every 4 waves, his armour value is increased by 2 (to incentivize higher level potions rather than mass producing 100 turrets with 100 potions). So by wave 12, he should have 8 armor (and since base potions deal 6-10 damage, they'll be effectively rendered useless)


## Rats: ![[Pasted image 20261101130446.png]] 

Fast, Agile, low HP and in high numbers

Base Hp: 10. Increase of 5% for every wave.

Base Speed: 2, Increase of 10% for every wave

Base Armour: 0, Increase of 1 every 3 waves.

Base Attack: 4, hits 1x every 0.5 seconds. Increase damage (never attack speed) every 2 waves by 1.

Fire Resistance: -25%

Poison Resistance: 25%

Orcs: 

## Tanky, decent, all-arounders: ![[Pasted image 20261101131114.png]]

Base Hp: 25. Increase of 10% for every wave.

Base Speed: 1.25, increase of 5% every 2 waves

Base Armour: 2, Increase of 2 every 4 waves.

Base Attack: 8, hits 1x every 0.75 seconds. Increase damage (never attack speed) by 2 every wave.

Universal Resistance: 25%

## Knight ![[Pasted image 20261101131245.png]]

Dedicated Tank, high armour and resistance 

Base Hp: 30. Increase of 8% for every wave.

Base Speed: 1, increase of 5% every 2 waves.

Base Armour: 4, Increase of 1.5 every 2 waves.

Base Attack: 6, hits 1x every 1 second. Increase damage (never attack speed) by 3 every 2 waves.

Fire, Lightning, Frost Resistance: 50%

Poison Resistance: -25%

## Archer ![[Pasted image 20261101131454.png]]

Ranged, Introduces potentially evasion mechanic, minimal resistances, fast

Base Hp: 20, Increase of 6% for every wave.

Base Speed: 1.5, increase of 8% every wave.

Base Armour: 1, Increase of 1 every 2 waves

Base Attack: 10, hits 1x every 1 seconds. Increase damage (never attack speed) by 4 every 2 waves.

Fire Resistance: 25% 

Lightning Resistance: 50%

Frost Resistance: -25% (weakness/vulnerable)

## Mage/Wizard![[Pasted image 20261101132423.png]]

The damage dealer, slow and glascannon-y with magic resistances. Spawned in few numbers.

Base Hp: 18, Increase of 4% for every wave.

Base Speed: 1.0, increase of  4% every wave.

Base Armour: 0, Increase of 1 every 4 waves.

Universal Resistance: "Resistant" tier (50%)

Base Attack: 16, hits 1x every 1.5 seconds. Increase damage (never attack speed) by 8 every 2 waves (to be a real threat in the late game)

## Mini-Bosses

Should use the existing framework, but maybe change the color of art (blue armour rather than silver, purple ork rather than green) and increase base stats by 3x and numbers limited. 

So instead of facing a wave of 10 knights at wave 5, get 3 Knights at wave 5 with 3x the stats of a wave 5 knight. This concentrates the force, making AoE less effective and poison/armour reduction EXTREMELY valuable.

the only stat we should be wary of doing 3x on is the armour stat, but if we leave it be, it incentivizes the acid tower immensely as the game goes on (could be neat)

## Boss Monsters

Should use the existing Framework but be tailored to user experience. IF we want, can follow a basic formula: Roughly half as many untis as mini-boss (rounding up) with 8x the stats. Should feel unfair.



## Formula's / Damage Calculation

Enemy Health - Damage

Well what is Damage?

Enemy Health - (Potion damage RNG (6-10 base) - Enemy Armour ) - ((Resistance Damage x Percent of Potion) - Resistance)

Ok well what is Resistance?

Resistance is per damage type. So: Poison DoT deals let's say 4 damage, ignores armour and is affected only by resistance type so if enemy has 2 armour and 25% resistance, then `(4*0.75)*1.0 - 0 armour = 3 damage per second for 3 seconds`

What happens if there is mixed damage? Each resistance type is calculated independantly:

Assume enemy has 4 armour and 25% poison resistance and 50% fire resistance.

Level 3 Poison Potion deals 12 damage after RNG generation. Sure. Well, that means it deals 12 additional poison damage per second for 3 seconds, total 48 damage (great single target! Notice: Poison affects do not stack! If a higher damaging poison is in affect, keep stronger version)

So we get: `Damage dealt == - ((12 damage - 4 armour) - ((12 dps * 100% of Potion) * (75% Poison Resistance)) * 3 Seconds == 8 up front damage and 9 dps for 3 turns for a total of 27 DoT or 35 total damage`

What if Level 3 Poison/Fire? Well, same 12 base damage after RNG, but then Fire AoE deals 6 AoE fire damage normally and 12 poison normally so we get:

`((12 damage - 4 armour) - ((12 poison dps * (50% of Potion) * (75% Poison Resistance) * 3 seconds) - (6 Fire Dps * (50% of Potion) * 50% Fire resistance)`

== `8 base damage - 4.5 dps poison per second for 3 seconds - 1.5 Fire Damage from AoE`

Master Damage Formula should be:

`(Base Damage - Armour) 
+ Fire Damage * 50% of Base Damage * Enemy Resistance
+ Lightning Damage * 50% of Base Damage * Enemy Resistance
+ Poison Damage * 100% of Base Damage * Enemy Resistance
+ Frost Damage * 40% of Base Damage * Enemy Resistance
`

Notice: 
Acid Damage (no Bonus damage)
Gunpowder Explosive (merely does Base Damage in AoE or could do 125% base damage as AoE)
Gold Potion Reduces Base Damage by 25% (immensely important for high level gold potions affecting other formulas)
Elixir does no Damage
Void Poison does just base damage


## Views

We are doing isometric top-down, not true top down, too difficult to do.

Ideas for the crafting table: 
![[Pasted image 20261101114852.png|376]]![[Pasted image 20261101114859.png|413]]