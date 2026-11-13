# LASERLIGHT

## Overview

Laserlight is a first-person-shooter in the vein of counter-strike, set within the realm of cyberspace. Cyberspace in this context shapes the various stages and themes for the game, each one revolving around some computing concept such as penetration testing, cybersecurity, etc. To illustrate an example, the starting stage for Laserlight will be called *Transistor*, and be a simple 1 vs 1 map layout in the shape of, well, a transistor. Other maps could include stages such as network routers, personal computers, smart devices, and more.

The basic gameplay loop is such. Two teams, the Tigers and the Hounds, will face off in various gameplay scenarios similar to that of the Terrorists and Counter-Terrorists in Counter-Strike. Tiger team takes the role of the attacker in most gameplay scenarios, while Hound team takes the role of the defender. Currently, the following gameplay scenarios are envisioned:

- Deletion, in which teams are tasked with eliminating all other opposing members to secure a round win. 
- Payload, in which Tiger team is tasked with placing a data breach at one of two breach points. 
  - This data breach will count down from a set value and detonate when the timer hits zero. Tiger team secures a round win by eliminating all Hound team members or setting off the payload. 
  - Hound team secures a round win by eliminating all Tiger team members, defusing the payload, or letting time run out.
- Clawback, in which Hound team is tasked with securing a hostage program from the Tiger team.
  - Hound team secures a round win by eliminating all Tiger Team members or escorting the hostage to safety.
  - Tiger team secures a round win by eliminating all Hound Team members or when time expires.
- Uplink, where Tiger team has to rotate around the map and upload data to terminals.
- Singleton, where each team must escort a designated VIP, the singleton, to a data extraction point. Death of a singleton results in a round loss for that team.
- Lockout, where the map has areas that are cut off or restricted based on the results of previous rounds, opening up advantages for Tiger or Hound team.
