#!/usr/bin/env python3

from julia import Main as jl_Main 

jl_Main.include( "MoteurRegles.jl" ) 

source = jl_Main.MoteurRegles.Source( """
  Variable message : "bonjour" 

  Condition "offre spéciale" (10) : 
    date.aujourdhui < "2022-02-16" 
    & client.est_membre() 

  Règle "réduction" (90) : 
    Si 
      ?"offre spéciale" 
    Alors 
      panier.reduction( 10, "%" ) 
""" ) 

parseur = jl_Main.MoteurRegles.Parseur( source ) 

try:
  print( jl_Main.MoteurRegles.avancer( parseur ) ) 
except Exception as e:
  print( 
    "Erreur dans Julia :", 
    str( e ).split( '\n', 2 )[1].split( ':', 1 )[1].strip()  
  ) 
  # raise e 
