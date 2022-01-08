#!/usr/bin/env python3

from functools import reduce 

if __name__ != "__main__": 
  raise Exception( 
    "Ce script ne peux pas être inséré comme module" 
  ) 

################################ 

REGLES = {} 
CONDITIONS = {} 
CLAUSES = {} 
CONTEXTES = {} 

################################ 

class Contexte: 
  relance_tour = False 
  autoriser_sinon = True 
  regles = {} 
  conditions = {} 
  clauses = {} 
  nom = None 
  def __init__( self, nom ): 
    self.nom = nom 
    CONTEXTES[self.nom] = self 
    self.regles = { cle: REGLES[cle] for cle in REGLES } 
    self.conditions = { cle: CONDITIONS[cle] for cle in CONDITIONS } 
    self.clauses = { cle: CLAUSES[cle] for cle in CLAUSES } 
  def resoudre( self, priorite ): 
    if priorite == "regle": 
      return self.__resoudre_regles__() 
    else: 
      raise Exception( f"Cette priorité '{priorite}' n'est pas supportée" ) 
  def __resoudre_regles__( self ): 
    '''Priorité ici à la règle la plus forte''' 
    poids_tour = None
    poids_tous = set( 
      ( self.regles[cle].poids for cle in self.regles ) 
    ) 
    regles_tour = () 
    while len( poids_tous ) > 0: 
      if not self.relance_tour and len( regles_tour ) < 1: 
        poids_tour = max( poids_tous ) 
        poids_tous.remove( poids_tour ) 
        regles_tour = set( filter( 
          lambda cle: self.regles[cle].poids == poids_tour, 
          self.regles 
        ) ) 
      else: 
        self.relance_tour = False 
      yield regles_tour 
      conditions_tour = set( reduce( 
        lambda l, r: r.conditions + l,  
        ( self.regles[cle] for cle in regles_tour ), 
        tuple() 
      ) ) 
      yield conditions_tour 
      clauses_tour = set( reduce( 
        lambda l, c: c.clauses + l,  
        ( self.conditions[cle] for cle in conditions_tour ), 
        tuple() 
      ) ) 
      yield clauses_tour 
      clauses_resultat = { 
        nom: clause.executer( self ) 
        for ( nom, clause ) in [ 
          ( nom, self.clauses[nom] ) 
          for nom in clauses_tour 
        ] 
      } 
      conditions_resultat = {
        condition_nom: reduce( 
          lambda a, b: a and b, 
          ( clauses_resultat[clause_nom] for clause_nom in self.conditions[condition_nom].clauses ), 
          True 
        ) 
        for condition_nom in conditions_tour 
      } 
      regles_resultat = { 
        regle_nom: reduce( 
          lambda a, b: a and b, 
          ( conditions_resultat[condition_nom] for condition_nom in self.regles[regle_nom].conditions ), 
          True 
        ) 
        for regle_nom in regles_tour 
      } 
      regles_jouees = { 
        regle_nom: self.regles[regle_nom].executer( 
          self, 
          regles_resultat[regle_nom] 
        ) 
        for regle_nom in regles_resultat 
        if regles_resultat[regle_nom] or self.autoriser_sinon 
      } 
      yield regles_jouees 
      if self.relance_tour: 
        regles_tour = set( 
          ( regle_nom for regle_nom in regles_resultat if regle_nom not in regles_jouees ) 
        ) 

class Regle: 
  etat = None 
  poids = None 
  nom = None 
  conditions = () 
  def __init__( self, nom, poids = None, conditions = () ): 
    global REGLES 
    self.nom = nom 
    self.poids = poids 
    self.conditions = conditions 
    REGLES[self.nom] = self 
  def executer( self, contexte: Contexte, conditions_resultat: bool ): 
    return self.__Finalement__( 
      contexte, 
      conditions_resultat, 
      self.__Si__( contexte ) 
        if conditions_resultat 
        else self.__Sinon__( contexte ) 
    )
  def __Si__( self, contexte: Contexte ): 
    pass 
  def __Sinon__( self, contexte: Contexte ): 
    pass 
  def __Finalement__( self, contexte: Contexte, conditions_resultat: bool, action_resultat ): 
    return action_resultat  
  def retirer( self, contexte: Contexte ): 
    if self.nom in CLAUSES: 
      del REGLES[self.nom] 

class Condition: 
  etat = None 
  poids = None 
  nom = None 
  clauses = () 
  def __init__( self, nom, poids = None, clauses = () ): 
    global CONDITIONS 
    self.nom = nom 
    self.poids = poids 
    self.clauses = clauses 
    CONDITIONS[self.nom] = self 
  def executer( self, contexte: Contexte ): 
    pass 
  def __execution__( self, contexte: Contexte ): 
    raise Exception( f"La condition '{self.nom}' n'a pas d'exécution" ) 
  def retirer( self, contexte: Contexte ): 
    if self.nom in CLAUSES: 
      del CONDITIONS[self.nom]  

class Clause: 
  etat = None 
  nom = None 
  def __init__( self, nom ): 
    global CONDITIONS 
    self.nom = nom 
    CLAUSES[self.nom] = self 
  def executer( self, contexte: Contexte ): 
    return True if self.__execution__( contexte ) else False 
  def __execution__( self, contexte: Contexte ): 
    raise Exception( f"La clause '{self.nom}' n'a pas d'exécution" ) 
  def retirer( self, contexte: Contexte ): 
    if self.nom in CLAUSES: 
      del CLAUSES[self.nom] 

################################ 

# Condition "offre spéciale" (10) : 
#   $date.aujourdhui < 2022-02-16 
#   & @client.est_membre()
# 
# Règle "réduction" (90) : 
#   Si 
#     ?"offre spéciale" 
#   Alors 
#     @panier.reduction( 10, "%" ) 

cl1 = Clause( "$date.aujourdhui < 2022-02-16" )
cl1.__execution__ = lambda c: False 

cl2 = Clause( "@client.est_membre()" )
cl2.__execution__ = lambda c: True 

co = Condition( "offre spéciale", poids = 90, clauses = ( cl1.nom, cl2.nom ) ) 
co.__execution__ = lambda _, clauses: reduce( 
  lambda a, b: a and b, 
  clauses, 
  True 
) 

re = Regle( "réduction", poids = 10, conditions = ( co.nom, ) )
re.__Si__ = lambda c: True  
re.__Sinon__ = lambda c: False 

contexte = Contexte( "test" ) 

print( "DEBUT" ) 

for m in contexte.resoudre( "regle" ): 
  print( m )

print( "FIN" ) 


