%token ADD SUB MUL DIV

%token FLOAT HEX OCT BIN

%token LPAREN RPAREN

%left ADD SUB
%left MUL DIV

%start expr

%%

expr -> Result<Expr>:
	binary_expr { $1.map(From::from) }
	| number { $1.map(From::from) }
	| paren_expr { $1.map(From::from) }
	| unary_expr { $1.map(From::from) }
	;

paren_expr -> Result<ParenExpr>:
	LPAREN expr RPAREN
	{
		Ok(ParenExpr::new($2?))
	}
	;

binary_expr -> Result<BinaryExpr>:
	expr ADD expr
	{
		let op = span_str($lexer, $2)?.parse()?;
		Ok(BinaryExpr::new($1?, op, $3?))
	}
	| expr SUB expr
	{
		let op = span_str($lexer, $2)?.parse()?;
		Ok(BinaryExpr::new($1?, op, $3?))
	}
	| expr MUL expr
	{
		let op = span_str($lexer, $2)?.parse()?;
		Ok(BinaryExpr::new($1?, op, $3?))
	}
	| expr DIV expr
	{
		let op = span_str($lexer, $2)?.parse()?;
		Ok(BinaryExpr::new($1?, op, $3?))
	}
	;

unary_expr -> Result<UnaryExpr>:
	unary_op expr %prec MUL
	{
		Ok(UnaryExpr::new($1?, $2?))
	}
	;

unary_op -> Result<UnaryOp>:
	ADD { span_str($lexer, $1)?.parse() }
	| SUB { span_str($lexer, $1)?.parse() }
	;

number -> Result<Number>:
	HEX { i64_from_str_radix(span_str($lexer, $1)?, 16).map(From::from) }
	| OCT { i64_from_str_radix(span_str($lexer, $1)?, 8).map(From::from) }
	| BIN { i64_from_str_radix(span_str($lexer, $1)?, 2).map(From::from) }
	| FLOAT { span_str($lexer, $1).and_then(any_from_str::<f64>).map(From::from) }
	;

%%

use anyhow::Result;
use crate::ast::util::*;
use crate::ast::*;
