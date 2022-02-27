.R1
accumulate

database bib.ref

move-punctuation

.R2

.ps 20
.vs 24
.ds FAM Monospace 

.TL
A test Document

.AU
Erarnitox - David Partsch
.AI
DropSoft
.DA

.AB
.PP
This Documentööäää is an example Document on how to use drop-markdown.Which is a wrapper around groff for easy writing of papers to be published on 
.B dropsoft.org


.AE

.bp

.NH 1
Heading
.XS
Heading
.XE
.PP
This was and example heading.  

a new line.The same line
.NH 2
A sub heading
.XS
A sub heading
.XE
.NH 3
And so on
.XS
And so on
.XE
.PP
You get the point.

.EQ
.PP
s = sqrt { { sum from i=1 to inf ( x sub i - x bar ) sup 2 } over { N - 1 } }
.EN

.NH 4
down deeep
.XS
down deeep
.XE
.PP
Goes down to 6 sub levels
.B1
.QP
This is a small quoted paragraph. I really hope this will work!
.B2
.NH 2
Testing bold text
.XS
Testing bold text
.XE
.PP
This is a small 
.I test
for 
.B bold
text.
.NH 2
Testing italic text
.XS
Testing italic text
.XE
.PP
This is a small test for 
.I italic
text.
.NH 2
Pictures
.XS
Pictures
.XE

.PSPIC -C "./pic/unknown.eps" 5i 5i 
.ce
description 


.PSPIC -C "./pic/36thchamberofshaolin2.eps" 5i 5i 
.ce
Welcome to the 36th chamber! 

.NH 2
Code
.XS
Code
.XE
.PP
some source highlighting
.[
source-highlight
.]
, which can be very useful to have tbh. no one will write or read code without some cool colors!
.B1

.DS I
.fam C
\f[CB]\m[darkblue]#include\m[]\fP \fC\m[red]<iostream>\m[]\fP

\m[darkgreen]int\m[] \f[CB]\m[black]main\m[]\fP\m[darkred]()\m[]\m[red]{\m[]
    std\m[darkred]::\m[]cout \m[darkred]<<\m[] \fC\m[red]"Hello World!"\m[]\fP \m[darkred]<<\m[] std\m[darkred]::\m[]endl\m[darkred];\m[]
\m[red]}\m[]

.fam
.DE
\m[]
.B2\m[]

.TC
