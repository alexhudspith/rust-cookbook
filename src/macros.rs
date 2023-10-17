/*
 * Compile-time exercising of macro fragment specifiers.
 *
 * https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
 *
 * Also see https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
 */
#![allow(unused_macros)]

macro_rules! ty {
    ($x:ty) => { () };
}

macro_rules! path {
    ($x:path) => { () };
}

macro_rules! ident {
    ($x:ident) => { () };
}

macro_rules! expr {
    ($x:expr) => { () };
}

macro_rules! item {
    ($x:item) => { () };
}

macro_rules! stmt {
    ($x:stmt) => { () };
}

macro_rules! block {
    ($x:block) => { () };
}

macro_rules! literal {
    ($x:literal) => { () };
}

macro_rules! vis {
    ($x:vis) => { () };
}

macro_rules! pat {
    ($x:pat) => { () };
}

macro_rules! lifetime {
    ($x:lifetime) => { () };
}

macro_rules! tt {
    ($x:tt) => { () };
}

#[cfg(test)]
mod tests {
    // These are not really tests since they are checked at compile time
    #[test]
    fn ty() {
        // Type: https://doc.rust-lang.org/reference/types.html#type-expressions
        // ParenthesizedType
        ty!((foo));
        // TypePath
        ty!(foo);
        ty!(foo::bar);
        ty!(foo<T>);
        ty!(foo::<T>);
        ty!(::foo);
        ty!(::foo::bar);
        ty!(::foo<T>);
        ty!(::foo::<T>);
        // TupleType
        ty!((foo, bar));
        // NeverType
        ty!(!);
        // RawPointerType
        ty!(*const foo);
        ty!(*mut foo);
        // ReferenceType
        ty!(&foo);
        ty!(&'a foo);
        ty!(&mut foo);
        ty!(&'a mut foo);
        // ArrayType
        ty!([foo; 1]);
        // SliceType
        ty!([foo]);
        // InferredType
        ty!(_);
        // QualifiedPathInType
        ty!(<T>::Foo::Bar);
        ty!(<T as U>::Foo::Bar);
        // BareFunctionType
        ty!(fn() -> ());
        ty!(fn(f: Foo) -> Foo);
        // ImplTraitTypeOneBound
        ty!(impl Foo);
        // ImplTraitType
        ty!(impl Foo + Send);
        ty!(impl Foo + 'a);
        ty!(impl for<'a> Foo + 'a);
        ty!(impl 'a);
        // TraitObjectTypeOneBound
        ty!(dyn Foo);
        // TraitObjectType
        ty!(dyn Foo + Send);
        ty!(dyn Foo + 'a);
        ty!(dyn for<'a> Foo + 'a);
        ty!(dyn 'a);
        // MacroInvocation
        ty!(println!());
    }

    #[test]
    fn path() {
        // https://doc.rust-lang.org/reference/paths.html#paths-in-types
        // TypePath
        path!(foo);
        path!(foo::bar);
        path!(foo<T>);
        path!(foo::<T>);
        path!(::foo);
        path!(::foo::bar);
        path!(::foo<T>);
        path!(::foo::<T>);
    }

    #[test]
    fn ident() {
        // https://doc.rust-lang.org/reference/identifiers.html
        // IDENTIFIER_OR_KEYWORD
        ident!(foo);
        ident!(impl);
        // RAW_IDENTIFIER
        ident!(r#impl);
        ident!(Self);

        // Disallowed raw identifiers
        // ident!(r#Self);
        // ident!(r#crate);
        // ident!(r#self);
        // ident!(r#super);
    }

    #[test]
    fn expr() {
        // Expression: https://doc.rust-lang.org/reference/expressions.html
        // LiteralExpression
        expr!(#[attr] ());
        expr!(#[attr] 0);
        // PathExpression
        expr!(#[attr] r#impl);
        expr!(#[attr] ::foo::bar);
        // OperatorExpression
        expr!(#[attr] 1 + 1);
        // GroupedExpression
        expr!(#[attr] (1 + 1));
        // ArrayExpression
        expr!(#[attr] [foo; bar]);
        // AwaitExpression
        expr!(#[attr] foo.await);
        // IndexExpression
        expr!(#[attr] foo[bar]);
        // TupleExpression
        expr!((#[attr] foo, bar));
        // TupleIndexingExpression
        expr!(#[attr] (foo, bar).1);
        // StructExpression
        expr!(#[attr] foo { });
        expr!(#[attr] foo { bar });
        expr!(#[attr] foo { bar: baz });
        // CallExpression
        expr!(#[attr] foo(x, y));
        // MethodCallExpression
        expr!(#[attr] foo.bar(x, y));
        // FieldExpression
        expr!(#[attr] foo.bar);
        // ClosureExpression
        expr!(#[attr] || 0);
        // AsyncBlockExpression
        expr!(#[attr] async { 0 });
        // ContinueExpression
        expr!(#[attr] continue);
        // BreakExpression
        expr!(#[attr] break foo);
        expr!(#[attr] break);
        // RangeExpression
        expr!(#[attr] 0..10);
        // ReturnExpression
        expr!(#[attr] return);
        expr!(#[attr] return 0);
        // UnderscoreExpression
        expr!(#[attr] (_,) = foo);
        // MacroInvocation
        expr!(#[attr] foo![]);

        // BlockExpression
        expr!(#[attr] { 0 });
        // UnsafeBlockExpression
        expr!(#[attr] unsafe { 0 });
        // LoopExpression
        expr!(#[attr] loop { break; });
        // IfExpression
        expr!(#[attr] if foo { 0 } else { 1 });
        // IfLetExpression
        expr!(#[attr] if let foo(bar) = baz { 0 } else { 1 });
        // MatchExpression
        expr!(#[attr] match m { foo(bar) => baz });
    }

    #[test]
    fn item() {
        // Item: https://doc.rust-lang.org/reference/items.html
        // Module
        item!(pub mod foo;);
        item!(pub mod foo { });
        // ExternCrate
        item!(pub extern crate foo as bar;);
        // UseDeclaration
        item!(pub use foo as bar;);
        // Function
        item!(pub fn foo<T>() -> bar { });
        // TypeAlias
        item!(pub type foo<T> = bar<T>;);
        // Struct
        item!(pub struct Foo<T: U> { bar: baz });
        // Enumeration
        item!(pub enum Foo<T: U> { Bar(baz) });
        // Union
        item!(pub union Foo<T: U> { bar: Bar, baz: Baz });
        // ConstantItem
        item!(pub const FOO: Bar = baz;);
        // StaticItem
        item!(pub static mut FOO: Bar = baz;);
        // Trait
        item!(pub trait Foo<T: U> { });
        // Implementation
        item!(pub impl<T> Foo<T> { });
        // ExternBlock
        item!(pub extern "C" { });
        // MacroInvocationSemi
        item!(println!(););
        // MacroRulesDefinition
        item!(macro_rules! foo { });
    }

    #[test]
    fn stmt() {
        // Statement: https://doc.rust-lang.org/reference/statements.html
        // Item
        stmt!(#[attr] pub mod S { });
        stmt!(#[attr] const S: Foo = bar;);
        // LetStatement (no semicolon)
        stmt!(#[attr] let foo = bar);
        // ExpressionStatement
        stmt!(#[attr] ());
        stmt!(#[attr] 1 + 1);
        stmt!(#[attr] if let foo(bar) = baz { 0 } else { 1 });
        // MacroInvocationSemi (no semicolon)
        stmt!(#[attr] println!());
    }

    #[test]
    fn block() {
        // https://doc.rust-lang.org/reference/expressions/block-expr.html
        // BlockExpression
        block!({});
        // BlockExpression > Statement+ ExpressionWithoutBlock
        block!({ let foo = bar; });
        // BlockExpression > ExpressionWithoutBlock
        block!({ 0 });
        // BlockExpression > Statement
        block!({ let foo = bar; });
    }

    #[test]
    fn literal() {
        // https://doc.rust-lang.org/reference/expressions/literal-expr.html
        // -?LiteralExpression
        literal!(0);
        literal!(-0);
        literal!(0x7f);
        literal!(-0x7f);
        literal!(1e-45);
        literal!(-1e-45);
        literal!('A');
        literal!(-'A');
        literal!("Hello");
        literal!(-"Hello");
        literal!(r#"Hello"#);
        literal!(-r#"Hello"#);
        literal!(b"Hello");
        literal!(-b"Hello");
        literal!(br#"Hello"#);
        literal!(-br#"Hello"#);
        literal!(true);
        literal!(-true);
    }

    #[test]
    fn vis() {
        // https://doc.rust-lang.org/reference/visibility-and-privacy.html
        // Visibility
        vis!(pub);
        vis!(pub(crate));
        vis!(pub(self));
        vis!(pub(super));
        vis!(pub(in foo));
        vis!(pub(in foo::bar));
        vis!(pub(in ::foo));
        vis!(pub(in ::foo::bar));
    }

    #[test]
    fn pat() {
        // Pattern: https://doc.rust-lang.org/reference/patterns.html
        // LiteralPattern
        pat!(-1);
        // IdentifierPattern
        pat!(foo);
        pat!(ref mut foo @ Foo(bar));
        // WildcardPattern
        pat!(_);
        // RestPattern
        pat!(..);
        pat!((foo .. bar));
        // ReferencePattern
        pat!(&foo);
        // StructPattern
        pat!(::Foo { bar: ref baz });
        // TupleStructPattern
        pat!(::Foo(bar, baz));
        // TuplePattern
        pat!((bar, baz));
        // GroupedPattern
        pat!((-1));
        // SlicePattern
        pat!([foo, bar]);
        // PathPattern
        // ...
        // MacroInvocation
        pat!(println!());
        // RangePattern
        pat!(foo @ 0..10);

        // Pattern
        pat!(Foo(bar) | Baz);
        pat!(| Foo(bar) | Baz);
        pat!(Foo(bar | baz));
        pat!(Foo(| bar | baz));
    }

    #[test]
    fn lifetime() {
        // LIFETIME_TOKEN: https://doc.rust-lang.org/reference/tokens.html#lifetimes-and-loop-labels
        // ' IDENTIFIER_OR_KEYWORD
        lifetime!('a);
        lifetime!('static);
        lifetime!('loop);
        // '_
        lifetime!('_);
        // Not raw identifier
        // lifetime!('r#impl);
    }

    #[test]
    fn tt() {
        // TokenTree: https://doc.rust-lang.org/reference/macros.html#macro-invocation

        // Keywords
        tt!(loop);
        // Identifiers
        tt!(foo);
        // Literals
        tt!(0);
        tt!(r#"hello"#);
        // Lifetimes
        tt!('a);
        // Punctuation
        tt!(!);
        tt!(&);
        tt!(-);
        tt!(::);
        tt!(<<=);
        tt!(~);
        tt!(.);
        tt!(..);
        tt!(...);

        // DelimTokenTree
        tt!(());
        tt!([]);
        tt!({});
        tt!([0 loop break (foo if) r#hello { ! - 'a }]);
    }
}
