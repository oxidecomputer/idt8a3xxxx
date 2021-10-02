# idt8a3xxxx

idt8a3xxxx: A crate for drivers for the Renesas 8A3XXXX series

The Renesas (nee IDT) 8A3XXXX series (branded as ClockMatrix) is a family
of clock generator parts.  These parts are sophisticated and capable,
offering a great degree of programmability.  This crate making available a
static definition of the familiy's modules and registers; the "8A3xxxx
Family Programming Guide" has details as to their meaning.  The
definitions themselves are contained in a RON file that, at build time
via `build.rs`, is turned into the static definition.


License: MPL-2.0
