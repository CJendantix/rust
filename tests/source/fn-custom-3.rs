// rustfmt-fn_arg_indent: Tabbed
// rustfmt-generics_indent: Inherit
// Test different indents.

fn foo(a: Aaaaaaaaaaaaaaa, b: Bbbbbbbbbbbbbbbb, c: Ccccccccccccccccc, d: Ddddddddddddddddddddddddd, e: Eeeeeeeeeeeeeeeeeee) {
    foo();
}

fn bar<'a: 'bbbbbbbbbbbbbbbbbbbbbbbbbbb, TTTTTTTTTTTTT, UUUUUUUUUUUUUUUUUUUU: WWWWWWWWWWWWWWWWWWWWWWWW>(a: Aaaaaaaaaaaaaaa) {
    bar();
}

impl Foo {
    fn foo(self, a: Aaaaaaaaaaaaaaa, b: Bbbbbbbbbbbbbbbb, c: Ccccccccccccccccc, d: Ddddddddddddddddddddddddd, e: Eeeeeeeeeeeeeeeeeee) {
        foo();
    }    

    fn bar<'a: 'bbbbbbbbbbbbbbbbbbbbbbbbbbb, TTTTTTTTTTTTT, UUUUUUUUUUUUUUUUUUUU: WWWWWWWWWWWWWWWWWWWWWWWW>(a: Aaaaaaaaaaaaaaa) {
        bar();
    }
}

struct Foo<TTTTTTTTTTTTTTTTTTTTTTTTTTTT, UUUUUUUUUUUUUUUUUUUUUU, VVVVVVVVVVVVVVVVVVVVVVVVVVV, WWWWWWWWWWWWWWWWWWWWWWWW> {
    foo: Foo,
}
