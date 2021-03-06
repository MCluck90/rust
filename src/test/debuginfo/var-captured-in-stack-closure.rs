// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// min-lldb-version: 310

// compile-flags:-g

// === GDB TESTS ===================================================================================

// gdb-command:run

// gdb-command:print variable
// gdb-check:$1 = 1
// gdb-command:print constant
// gdb-check:$2 = 2
// gdb-command:print a_struct
// gdbg-check:$3 = {a = -3, b = 4.5, c = 5}
// gdbr-check:$3 = var_captured_in_stack_closure::Struct {a: -3, b: 4.5, c: 5}
// gdb-command:print *struct_ref
// gdbg-check:$4 = {a = -3, b = 4.5, c = 5}
// gdbr-check:$4 = var_captured_in_stack_closure::Struct {a: -3, b: 4.5, c: 5}
// gdb-command:print *owned
// gdb-check:$5 = 6

// gdb-command:continue

// gdb-command:print variable
// gdb-check:$6 = 2
// gdb-command:print constant
// gdb-check:$7 = 2
// gdb-command:print a_struct
// gdbg-check:$8 = {a = -3, b = 4.5, c = 5}
// gdbr-check:$8 = var_captured_in_stack_closure::Struct {a: -3, b: 4.5, c: 5}
// gdb-command:print *struct_ref
// gdbg-check:$9 = {a = -3, b = 4.5, c = 5}
// gdbr-check:$9 = var_captured_in_stack_closure::Struct {a: -3, b: 4.5, c: 5}
// gdb-command:print *owned
// gdb-check:$10 = 6


// === LLDB TESTS ==================================================================================

// lldb-command:run

// lldb-command:print variable
// lldb-check:[...]$0 = 1
// lldb-command:print constant
// lldb-check:[...]$1 = 2
// lldb-command:print a_struct
// lldb-check:[...]$2 = Struct { a: -3, b: 4.5, c: 5 }
// lldb-command:print *struct_ref
// lldb-check:[...]$3 = Struct { a: -3, b: 4.5, c: 5 }
// lldb-command:print *owned
// lldb-check:[...]$4 = 6

// lldb-command:continue

// lldb-command:print variable
// lldb-check:[...]$5 = 2
// lldb-command:print constant
// lldb-check:[...]$6 = 2
// lldb-command:print a_struct
// lldb-check:[...]$7 = Struct { a: -3, b: 4.5, c: 5 }
// lldb-command:print *struct_ref
// lldb-check:[...]$8 = Struct { a: -3, b: 4.5, c: 5 }
// lldb-command:print *owned
// lldb-check:[...]$9 = 6

#![feature(box_syntax)]
#![allow(unused_variables)]
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

struct Struct {
    a: isize,
    b: f64,
    c: usize
}

fn main() {
    let mut variable = 1;
    let constant = 2;

    let a_struct = Struct {
        a: -3,
        b: 4.5,
        c: 5
    };

    let struct_ref = &a_struct;
    let owned: Box<_> = box 6;

    {
        let mut first_closure = || {
            zzz(); // #break
            variable = constant + a_struct.a + struct_ref.a + *owned;
        };

        first_closure();
    }

    {
        let mut second_closure = || {
            zzz(); // #break
            variable = constant + a_struct.a + struct_ref.a + *owned;
        };
        second_closure();
    }
}

fn zzz() {()}
