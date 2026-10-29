# Security policy

EH3D-MOTIONCNC is so far a Rust library: it reads no files from other people and opens no network connection or serial port. What it holds is the view of a job (the scene, the themes and the words of the interface), shared through EH3D-COMMON; a theme file written by a user is the only outside text it parses, and it is checked rather than trusted.

What this repository does **not** do: it does not drive a real machine, it is not a safety system and it does not handle limit switches or an emergency stop. Do not connect a machine to it.

## Reporting a problem

Please report a vulnerability privately to **electrohobby3d@gmail.com**, with the steps that show it. Do not open a public issue for it. You will get an answer as soon as possible; this is a one-person project.
