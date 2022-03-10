module ModGrammaire

using ..ModJetons 
using ..ModAnalyseur 

RetourRegle = Union{ Nothing, ModJetons.Jetons } 

RetourTerminal = Union{ Nothing, ModJetons.Jeton } 

struct Variable 
  cle::String 
  valeur::String 
end 
struct Clause 
  valeur::String
end 
struct Condition 
  nom::String 
  poids::Int32 
  clauses::Vector{Clause}
end 
struct Règle 
  nom::String 
  poids::Int32 
  conditions::Vector{Condition}
end 

AlphaMinus = map( only, string.(Char.('a':'z')) |> collect ) 
AlphaMajus = map( only, string.(Char.('A':'Z')) |> collect ) 
Num = map( only, string.(Char.('0':'9')) |> collect ) 
AlphaNum = cat( 
  AlphaMinus, 
  AlphaMajus, 
  Num, 
  dims=1 
)
AlphaNumVar = cat( 
  AlphaNum, 
  ['_','.'], 
  dims=1 
)


function terminal_variable( position_relative::Int, parseur::ModAnalyseur.Parseur ) 
  p = 0
  while only( parseur.source.obtenir( position_relative+p, 0 ) ) in AlphaNumVar 
    p += 1
  end 
  if p == 0 
    return false 
  else 
    return ModJetons.TerminalVariable( 
      parseur.source.obtenir( 
        position_relative, 
        p-1 
      ) 
    ) 
  end 
end 

function expression_variable( parseur::ModAnalyseur.Parseur )
  position_depart = parseur.position 
  parseur.espaces() 
  if ! parseur.comparer( "Variable", true ) 
    return false 
  end 
  parseur.espaces() 
  println( terminal_variable( parseur.position, parseur ) ) 
  return parseur.position 
end 

function avancer( parseur::ModAnalyseur.Parseur )
  return expression_variable( parseur )   
end 



end 