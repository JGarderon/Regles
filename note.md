
Condition "offre spéciale" (10) : 
	$date.aujourdhui < 2022-02-16 
	& @client.est_membre()

Règle "réduction" (90) : 
	Si 
		?"offre spéciale" 
	Alors 
		@panier.reduction( 10, "%" ) 

--------------

Condition "..." (poids) : (...)

Règle "..." (poids) : Si ?... & !?... ; Alors @... | @... . 

--------------

Configuration : var = 1
C : var = 1 

Bloc { 
	(...) 
}

Règle (nom complexe) : Si (conditions) ; Alors (actions) ; Sinon (actions) ; Finalement (actions) .
R :

Action (nom complexe) : (actions) . 

--------------

Objet 			[\w] '{' ( [\w] '=' Arg )* '}' 

Commentaire		'/*' ( [^/][^*] )*  '*/'

Variable		Champ | Texte | Heure | Date | Nombre 

Conditions		Condition ( ( "&" Action ) | ( "|" Action ) )* 
Condition 		Actions | Variable Operateur Variable 

Actions 		Action ( ";" Action )* 
Action 			'#' Champ | '@' Champ '(' Args? ')' | Variable '=' ( Variable | Action )

Args 			Arg ( "," Arg )*
Arg 			Variable | Action

Operateur		"<" | ">" | "==" | "!=" | "<=" | ">=" | "~" | "="

Texte			'"' [\w\s]* '"'

Nombre 			( '-' )? [0-9]+ ( '.' [0-9]+ )? 

Champ			\w | \w '.' \w

Date 			[0-9]{4} '-' [0-9]{2} '-' [0-9]{2} ( ' ' Heure )? 

Heure 			[0-9]{2} ':' [0-9]{2} ( ':' [0-9]{2} )? 

--------------




