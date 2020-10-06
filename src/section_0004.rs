//! The program begins with a normal \PASCAL\ program heading, whose
//! components will be filled in later, using the conventions of \.{WEB}.
//! @.WEB@>
//! For example, the portion of the program called `\X\glob:Global
//! variables\X' below will be replaced by a sequence of variable declarations
//! that starts in $\section\glob$ of this documentation. In this way, we are able
//! to define each individual global variable when we are prepared to
//! understand what it means; we do not have to define all of the globals at
//! once.  Cross references in $\section\glob$, where it says ``See also
//! sections \gglob, \dots,'' also make it possible to look at the set of
//! all global variables, if desired.  Similar remarks apply to the other
//! portions of the program heading.
//!
//! Actually the heading shown here is not quite normal: The |program| line
//! does not mention any |output| file, because \ph\ would ask the \TeX\ user
//! to specify a file name if |output| were specified here.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//!
//! @d mtype==t@&y@&p@&e {this is a \.{WEB} coding trick:}
//! @f mtype==type {`\&{mtype}' will be equivalent to `\&{type}'}
//! @f type==true {but `|type|' will not be treated as a reserved word}
//!
//! @p @t\4@>@<Compiler directives@>@/
//! program TEX; {all file names are defined dynamically}
//! label @<Labels in the outer block@>@/
//! const @<Constants in the outer block@>@/
//! mtype @<Types in the outer block@>@/
//! var @<Global variables@>@/
//! @#
//! procedure initialize; {this procedure gets things started properly}
//!   var @<Local variables for initialization@>@/
//!   begin @<Initialize whatever \TeX\ might access@>@;
//!   end;@#
//! @t\4@>@<Basic printing procedures@>@/
//! @t\4@>@<Error handling procedures@>@/
