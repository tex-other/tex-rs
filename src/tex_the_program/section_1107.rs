//! ` `

// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1107($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("unpenalty",remove_item,penalty_node);@/
    // @!@:un_penalty_}{\.{\\unpenalty} primitive@>
    // primitive("unkern",remove_item,kern_node);@/
    // @!@:un_kern_}{\.{\\unkern} primitive@>
    // primitive("unskip",remove_item,glue_node);@/
    primitive(
        globals,
        crate::strpool_str!("unskip"),
        remove_item,
        glue_node as _,
    );
    // @!@:un_skip_}{\.{\\unskip} primitive@>
    // primitive("unhbox",un_hbox,box_code);@/
    primitive(
        globals,
        crate::strpool_str!("unhbox"),
        un_hbox,
        box_code as _,
    );
    // @!@:un_hbox_}{\.{\\unhbox} primitive@>
    // primitive("unhcopy",un_hbox,copy_code);@/
    // @!@:un_hcopy_}{\.{\\unhcopy} primitive@>
    // primitive("unvbox",un_vbox,box_code);@/
    primitive(
        globals,
        crate::strpool_str!("unvbox"),
        un_vbox,
        box_code as _,
    );
    // @!@:un_vbox_}{\.{\\unvbox} primitive@>
    // primitive("unvcopy",un_vbox,copy_code);@/
    // @!@:un_vcopy_}{\.{\\unvcopy} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0149::glue_node;
use crate::section_0208::*;
use crate::section_0264::primitive;
use crate::section_1071::box_code;
