
use crate::Arena;
use super::*;

#[test]
fn hello_world() {
    let arena = Arena::new();
    let src = "Hello, World! {}()";
    let mut errors = Errors::new(); 
    let tokens = lex(&arena, src, &mut errors);
    assert_eq!(tokens.as_slice(), &[
        Segment { ty: SegmentType::Text, span: SrcSpan::new(0, 18) }
    ]);
    assert!(errors.is_empty());
}

#[test]
fn empty_expr_and_stmt() {
    let arena = Arena::new();
    let src = "#{}!{}";
    let mut errors = Errors::new(); 
    let tokens = lex(&arena, src, &mut errors);
    assert_eq!(tokens.as_slice(), &[
        Segment { ty: SegmentType::Expr(ArenaVec::new()), span: SrcSpan::new(0, 3) },
        Segment { ty: SegmentType::Stmt(ArenaVec::new()), span: SrcSpan::new(3, 6) }
    ]);
    assert!(errors.is_empty());
}

#[test]
fn unclosed_expr() {
    let arena = Arena::new();
    let src = "#{";
    let mut errors = Errors::new(); 
    lex(&arena, src, &mut errors);
    assert!(!errors.is_empty());
}

#[test]
fn whitespace() {
    let arena = Arena::new();
    let src = "#{ \r\n\t}";
    let mut errors = Errors::new(); 
    let tokens = lex(&arena, src, &mut errors);
    assert_eq!(tokens.as_slice(), &[
        Segment { ty: SegmentType::Expr(ArenaVec::new()), span: SrcSpan::new(0, 7) },
    ]);
    assert!(errors.is_empty());
}

#[test]
fn paren_brack_brace() {
    let arena = Arena::new();
    let src = "#{()[]{}}";
    let mut errors = Errors::new(); 
    let expr = &lex(&arena, src, &mut errors)[0];
    let SegmentType::Expr(tokens) = &expr.ty else { panic!() };
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0].ty, TokenType::Paren(_)));
    assert!(matches!(tokens[1].ty, TokenType::Brack(_)));
    assert!(matches!(tokens[2].ty, TokenType::Brace(_)));

    assert!(errors.is_empty());
}

#[test]
fn nested_brackets() {
    let arena = Arena::new();
    let src = "#{([])}";
    let mut errors = Errors::new(); 
    let expr = &lex(&arena, src, &mut errors)[0];
    let SegmentType::Expr(tokens) = &expr.ty else { panic!() };
    let TokenType::Paren(tokens) = &tokens[0].ty else { panic!() };
    assert!(matches!(tokens[0].ty, TokenType::Brack(_)));

    assert!(errors.is_empty());
}

#[test]
fn unclosed_paren() {
    let arena = Arena::new();
    let src = "#{(}";
    let mut errors = Errors::new(); 
    lex(&arena, src, &mut errors);
    assert!(!errors.is_empty());
}

#[test]
fn paranthetical_nonsense() {
    let arena = Arena::new();
    let src = "#{([)]}";
    let mut errors = Errors::new(); 
    lex(&arena, src, &mut errors);
    assert!(!errors.is_empty());
}
