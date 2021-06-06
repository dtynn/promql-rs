%token ADD SUB MUL DIV

%token FLOAT HEX OCT BIN

%token LPAREN RPAREN

%left ADD SUB
%left MUL DIV

%start expr

%%

expr: binary_expr
	| number
	| paren_expr
	;


number: HEX
	  | OCT
	  | BIN
	  | FLOAT
	  ;

paren_expr: LPAREN expr RPAREN
		  ;

binary_expr: expr ADD expr
		   | expr SUB expr
		   | expr MUL expr
		   | expr DIV expr
		   ;
