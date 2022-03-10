#!/usr/bin/env python3


if __name__ != "__main__": 
  raise Exception( 
    "Ce script ne peux pas être inséré comme module" 
  ) 

################################ 

class Traducteur: 

  jetons = [] 

  def __init__( self, source ): 
    self.source = source 
    self.position = 0 
    self.jetons = [] 

  def __iter__( self ): 
    return self 

  def __next__( self ): 
    exp = self.__expression_variable__() 
    if exp is not False: 
      self.position, jeton = exp 
      self.jetons.append( 
        ( "variable", jeton ) 
      ) 
      return ( "variable", exp ) 
    exp = self.__expression_condition__() 
    if exp is not False: 
      self.position, jeton = exp 
      self.jetons.append( 
        ( "condition", jeton ) 
      ) 
      return ( "condition", exp ) 
    raise StopIteration 

  def __expression_variable__( self ): 
    jetons = []
    position = self.position
    position += self.__espaces__( position ) 
    if "variable" != self.__tester__( position, 8 ).lower(): 
      return False 
    jetons.append( 
      ( 
        "variable", 
        "declaration" 
      ) 
    ) 
    position += 8 
    position += self.__espaces__( position ) 
    r = self.__variable__( position ) 
    if r is False: 
      raise Exception( 
        "une déclaration de variable n'est pas conforme (variable attendue)" 
      ) 
    jetons.append( 
      ( 
        "partie_gauche", 
        self.source[position:position+r] 
      ) 
    ) 
    position += r 
    position += self.__espaces__( position ) 
    if ":" != self.__tester__( position, 1 ): 
      raise Exception( 
        "un séparateur n'est pas trouvé lors de la déclaration d'une variable" 
      ) 
    position +=1 
    position += self.__espaces__( position ) 
    r = self.__clause_operande__( position ) 
    if r is False: 
      raise Exception( 
        "une opérande n'est pas trouvée lors de la déclaration d'une variable, après le séparateur" 
      ) 
    p_relative, type_jeton = r 
    jetons.append( 
      ( 
        "partie_droite", 
        ( 
          type_jeton, 
          self.source[position:position+p_relative] 
        ) 
      ) 
    ) 
    position += p_relative 
    return position, jetons 

  def __expression_condition__( self ): 
    jetons = []
    position = self.position
    position += self.__espaces__( position ) 
    if "condition" != self.__tester__( position, 9 ).lower(): 
      return False 
    jetons.append( ( "condition", "debut" ) ) 
    position += self.__espaces__( position+9 ) + 9
    r = self.__capture__( position, '"' ) 
    if r is False: 
      return False 
    jetons.append( 
      ( "condition", ( "nom", self.source[position+1:position+r-1] ) ) 
    ) 
    position += r 
    position += self.__espaces__( position ) 
    if "(" == self.__tester__( position, 1 ): 
      p_relative = position + self.__entier__( position+1 ) + 1
      if p_relative is False: 
        raise Exception( "un entier est obligatoire dans un poids de condition" ) 
      if ")" != self.__tester__( p_relative, 1 ): 
        raise Exception( "seul un entier est autorisé dans un poids de condition" ) 
      jetons.append( ( "condition", ( "poids", self.source[position+1:p_relative] ) ) )  
      position = p_relative + 1 
    position += self.__espaces__( position ) 
    if ":" != self.__tester__( position, 1 ): 
      raise Exception( "erreur car la liste des clauses ne commence pas dans la condition" ) 
    position += self.__espaces__( position+1 ) 
    jetons.append( ( "condition", "liste" ) ) 
    r = self.__clause__( position ) 
    if r is False: 
      return False 
    position, jeton = r 
    jetons += jeton 
    while True: 
      position += self.__espaces__( position ) 
      jeton = self.__condition_liaison__( position ) 
      if jeton is False: 
        jetons.append( ( "condition", "fin" ) ) 
        return position, jetons 
      p_relative, jeton = jeton 
      position += p_relative 
      jetons.append( jeton ) 
      position += self.__espaces__( position ) 
      r = self.__clause__( position ) 
      if r is False: 
        raise Exception( "un opérateur de liaison pour condition a été trouvé sans clause en suivant" ) 
      jetons += r 

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

  def __condition_liaison__( self, position ): 
    if "&" == self.__tester__( position, 1 ): 
      return ( 1, ( "condition:liaison", "et" ) ) 
    if "|" == self.__tester__( position, 1 ): 
      return ( 2, ( "condition:liaison", "ou" ) ) 
    return False 

  def __clause__( self, position ): 
    jetons = []
    position += self.__espaces__( position ) 
    jeton = self.__clause_operande__( position ) 
    if jeton is False: 
      return False 
    p_relative, jeton = jeton 
    jetons.append( 
      ( 
        "clause:operande", 
        ( 
          jeton, 
          self.source[position:position+p_relative] 
        ) 
      ) 
    ) 
    position += p_relative 
    position += self.__espaces__( position ) 
    jeton = self.__clause_liaison__( position ) 
    if jeton is False: 
      return jetons 
    else: 
      p_relative, jeton = jeton 
      position += p_relative 
      jetons.append( jeton ) 
    position += self.__espaces__( position ) 
    jeton = self.__clause_operande__( position ) 
    if jeton is False: 
      raise Exception( "une liaison est ouvert sans opérande dans une clause" ) 
    p_relative, jeton = jeton 
    jetons.append( 
      ( 
        "clause:operande", 
        ( 
          jeton, 
          self.source[position:position+p_relative] 
        ) 
      ) 
    ) 
    position += p_relative 
    return ( position, jetons ) 

  def __clause_operande__( self, position ): 
    p_relative = self.__appelable__( position ) 
    if p_relative is not False: 
      return ( p_relative, "appelable" ) 
    p_relative = self.__variable__( position ) 
    if p_relative is not False: 
      return ( p_relative, "variable" ) 
    p_relative = self.__capture__( position, '"' ) 
    if p_relative is not False: 
      return ( p_relative, "capture" ) 
    return False 

  def __clause_liaison__( self, position ): 
    if "<" == self.__tester__( position, 1 ): 
      return ( 1, ( "clause:liaison", "inferieur à" ) ) 
    if "<=" == self.__tester__( position, 2 ): 
      return ( 2, ( "clause:liaison", "inferieur ou egal à" ) ) 
    if ">" == self.__tester__( position, 1 ): 
      return ( 1, ( "clause:liaison", "superieur à" ) ) 
    if ">=" == self.__tester__( position, 2 ): 
      return ( 2, ( "clause:liaison", "superieur ou egal à" ) )
    if "==" == self.__tester__( position, 2 ): 
      return ( 2, ( "clause:liaison", "egal à" ) ) 
    if "!=" == self.__tester__( position, 2 ): 
      return ( 2, ( "clause:liaison", "different de" )  )
    if "~" == self.__tester__( position, 1 ): 
      return ( 1, ( "clause:liaison", "proche de" ) ) 
    return False 

  def __appelable__( self, position ): 
    r = self.__variable__( position ) 
    if r is False: 
      return False  
    if "()" != self.__tester__( position+r, 2 ): 
      return False 
    return r+2 

  def __variable__( self, position ): 
    if "$" != self.__tester__( position, 1 ): 
      return False 
    position += 1 
    for i, signe in enumerate( self.source[position:] ): 
      if not signe.isalnum() and signe not in ( '.', '_' ): 
        if i == 0: 
          return False 
        return i+1 
    raise Exception( 
      "une variable a été commencée mais reste sans corps (fin de source)" 
    ) 

################################ 

class Variable: 
  def __init__( self, partie_gauche, partie_droite ): 
    self.partie_gauche = partie_gauche 
    self.partie_droite = partie_droite 

################################ 

source = ''' 

  Variable $message : "bonjour" 

  Condition "offre spéciale" (10) : 
    $date.aujourdhui < "2022-02-16" 
    & $client.est_membre() 

  Condition "réduction maximale" (9) : 
    $panier.total_reduction( 25, "%" ) 

  Règle "réduction" (90) : 
    Si 
      ?"offre spéciale" 
    Donc 
      ?"réduction maximale"
    Alors 
      $panier.reduction( 10, "%" ) 
    Sinon 
      $panier.reduction( 5, "%" ) 
    Finalement 
      $panier.mettre_à_jour() 

''' 

t = Traducteur( source ) 
for item in t:
  print( 
    item  
  ) 

variables = [ 
  Variable( variable[1], variable[2] ) 
  for variable in [ 
    jeton[1] 
    for jeton in t.jetons 
    if jeton[0] == "variable" 
  ] 
]

print( 
  "---\n", 
  variables 
) 

