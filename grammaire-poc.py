#!/usr/bin/env python3


if __name__ != "__main__": 
  raise Exception( 
    "Ce script ne peux pas être inséré comme module" 
  ) 

################################ 

class Traducteur: 

	jetons = [] 

	def __espaces__( self, position ): 
		for i, signe in enumerate( self.source[position:] ): 
			if signe in ( ' ', '\t', '\n' ): 
				continue 
			else: 
				return i 
		return i 

	def __tester__( self, position, taille ): 
		return self.source[position:position+taille] 

	def __capture__( self, position, debut, fin=None ): 
		if fin is None: 
			fin = debut 
		if self.source[position] != debut: 
			return False 
		echappement = False 
		for i, signe in enumerate( self.source[position+1:] ): 
			if signe == fin and echappement is False: 
				return i+2 
			elif signe == '\\': 
				echappement = True 
			elif echappement is True: 
				echappement = False 
		raise Exception( f"une capture a échoué ('{debut}')" ) 

	def __entier__( self, position ): 
		entiers = tuple( map( 
			lambda i: str( i ), 
			tuple( range( 0, 10 ) ) 
		) ) 
		if self.source[position] not in entiers: 
			return False 
		for i, signe in enumerate( self.source[position:] ): 
			if signe not in entiers: 
				return i 

	def __init__( self, source ): 
		self.source = source 
		self.pointer = 0 
		print( "condition = ", self.__condition__( 0 ) ) 

	def __condition__( self, position ): 
		jetons = []
		position += self.__espaces__( position ) 
		if "condition" != self.__tester__( position, 9 ).lower(): 
			return False 
		jetons.append( "condition:debut" ) 
		position += self.__espaces__( position+9 ) + 9
		r = self.__capture__( position, '"' ) 
		if r is False: 
			return False 
		jetons.append( "nom: " + self.source[position+1:position+r-1] ) 
		position += r 
		position += self.__espaces__( position ) 
		if "(" == self.__tester__( position, 1 ): 
			p_relatif = position + self.__entier__( position+1 ) + 1
			if p_relatif is False: 
				raise Exception( "un entier est obligatoire dans un poids de condition" ) 
			if ")" != self.__tester__( p_relatif, 1 ): 
				raise Exception( "seul un entier est autorisé dans un poids de condition" ) 
			jetons.append( "entier: " + self.source[position+1:p_relatif]  ) 
			position = p_relatif + 1 
		position += self.__espaces__( position ) 
		if ":" != self.__tester__( position, 1 ): 
			raise Exception( "erreur car la liste des clauses ne commence pas dans la condition" ) 
		position += self.__espaces__( position ) 
		jetons.append( "condition:liste" ) 
		self.jetons += jetons 




################################ 

source = '''
	Condition "offre spéciale" (10) : 
	  $date.aujourdhui < s2022-02-16 
	  & @client.est_membre()

	Règle "réduction" (90) : 
	  Si 
	    ?"offre spéciale" 
	  Alors 
	    @panier.reduction( 10, "%" ) 
''' 

print( 
	Traducteur( source ).jetons 
)

