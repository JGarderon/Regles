module ModJetons

struct TerminalTexte 
  valeur::String 
end 

struct TerminalEntier 
  valeur::Int 
end 

struct TerminalDate 
  valeur::Int32 
  annee::Int16 
  mois::Int 
  jour::Int 
end 

struct TerminalVariable 
  valeur::String
end 

struct TerminalAppelable 
  valeur::String
end 

Jeton = Union{ 
  TerminalTexte, 
  TerminalEntier, 
  TerminalDate, 
  TerminalVariable, 
  TerminalAppelable 
} 

mutable struct Jetons 
  l::Vector{Jeton} 
  taille::Int 
  function Jetons() 
    objet = new() 
    objet.l = []
    objet.taille = 0 
    return objet
  end 
end 

end 