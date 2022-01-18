# module MoteurRegles 
################# Début du module 

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

# RetourRegle = Union{ Nothing, Jetons } 

# RetourTerminal = Union{ Nothing, Jeton } 

mutable struct MotVariable 
  __debut__::Int
  __taille__::Int 
  cle::TerminalVariable 
  valeur::Union{ TerminalVariable, TerminalTexte } 
end 
struct MotClause 
  valeur::String
end 
struct MotCondition 
  nom::String 
  poids::Int32 
  clauses::Vector{MotClause}
end 
struct MotRègle 
  nom::String 
  poids::Int32 
  conditions::Vector{Condition}
end 

Jeton = Union{ 
  MotVariable, 
  MotClause,
  MotCondition,
  MotRègle
} 

mutable struct Jetons 
  liste::Vector{Jeton} 
  taille::Int 
  function Jetons() 
    objet = new() 
    objet.liste = []
    objet.taille = 0 
    return objet
  end 
end 

mutable struct Source 
  texte::String  
  obtenir::Function 
  obtenir_character::Function
  function Source( texte::String ) 
    objet = new() 
    objet.texte = texte 
    objet.obtenir = function ( position::Int, taille::Int )
      return SubString( 
        objet.texte, 
        nextind( objet.texte, position ),  
        nextind( objet.texte, position+taille ) 
      )  
    end 
    objet.obtenir_character = function ( position::Int ) 
      return only( 
        SubString( 
          objet.texte, 
          nextind( objet.texte, position )  
        ) 
      ) 
    end 
    return objet 
  end 
end

mutable struct Parseur 
  source::Source 
  position::Int 
  espaces_liste::NTuple{4,Char} 
  jetons::Jetons 
  espaces::Function 
  comparer::Function 
  avancer::Function 
  ajouter_jetons::Function 
  function Parseur( source::Source ) 
    objet = new()
    objet.source = source 
    objet.position = 1 
    objet.espaces_liste = ( ' ', '\t', '\n', '\r' ) 
    objet.jetons = Jetons()
    objet.espaces = function ( avancement::Bool = true ) 
      p = objet.position
      taille = length( objet.source.texte ) 
      while p < taille
        if !( 
          objet.source.obtenir( p, 1 )[1] in objet.espaces_liste 
        )  
          break 
        end 
        p += 1 
      end 
      r = p - objet.position 
      if avancement 
        objet.position = p 
      end 
      return r 
    end 
    objet.comparer = function ( texte::String, avancement::Bool=false ) 
      taille = length( texte )  
      if objet.source.obtenir( 
        objet.position, 
        taille-1
      ) == texte 
        if avancement 
          objet.position += taille
        end 
        return true 
      else 
        return false 
      end 
    end 
    objet.ajouter_jetons = function ( jeton::Jeton ) 
      push!( objet.jetons.liste, jeton ) 
    end 
    return objet 
  end 
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

function terminal_variable( position_relative::Int, parseur::Parseur ) 
  p = 0
  while only( parseur.source.obtenir( position_relative+p, 0 ) ) in AlphaNumVar 
    p += 1 
  end 
  if p == 0 
    return false 
  else 
    return TerminalVariable( 
      parseur.source.obtenir( 
        position_relative, 
        p-1 
      ) 
    ) 
  end 
end 

function terminal_texte( position_relative::Int, parseur::Parseur ) 
  p = 0
  if ! parseur.comparer( "\"", false ) 
    return false 
  end 
  echappement = false 
  while true 
    p += 1
    c = only( parseur.source.obtenir( position_relative+p, 0 ) ) 
    if c == '\\' 
      echappement = true 
    elseif c =='"'
      if echappement == false 
        break
      else 
        echappement = false 
      end 
    end 
  end 
  return TerminalTexte( 
    parseur.source.obtenir( 
      position_relative+1, 
      p-2 
    ) 
  ) 
end 

function expression_variable( parseur::Parseur )
  position_depart = parseur.position 
  if ! parseur.comparer( "Variable", false ) 
    return false 
  end 
  parseur.position += 8 
  parseur.espaces() 
  cle = terminal_variable( parseur.position, parseur ) 
  if cle == false 
    error( "Une variable a été commencée sans partie 'clé' valide" ) 
  end 
  parseur.position += length( cle.valeur )
  parseur.espaces() 
  if ! parseur.comparer( ":", true ) 
    error( "Une variable a été commencée sans séparateur valide" ) 
  end 
  parseur.espaces() 
  valeur = terminal_variable( parseur.position, parseur ) 
  if valeur == false 
    valeur = terminal_texte( parseur.position, parseur ) 
  end 
  if valeur == false 
    error( "Une variable a été commencée sans partie 'valeur' valide" ) 
  end 
  parseur.position += length( valeur.valeur ) 
  variable = MotVariable( 
    position_depart, 
    ( parseur.position - position_depart ), 
    cle, 
    valeur 
  ) 
  parseur.ajouter_jetons( variable ) 
  return variable 
end 



function expression_condition( parseur::Parseur )
  parseur.espaces() 
  position_depart = parseur.position 
  print( parseur.source.obtenir( position_depart, 9 ) ) 
  if ! parseur.comparer( "Condition", false ) 
    return false 
  end 
  parseur.position += 9 
  parseur.espaces() 
  nom = terminal_texte( parseur.position, parseur ) 
  if nom == false 
    error( "Une condition a été commencée sans partie 'nom' valide" ) 
  end 
  parseur.position += length( nom.valeur ) 
  parseur.espaces() 
  # if parseur.comparer( "(", true ) 


  #   error( "Une variable a été commencée sans séparateur valide" ) 
  # end 
  parseur.espaces() 
  return nom 
end 





function avancer( parseur::Parseur ) 
  # parseur.espaces() 
  # println( expression_variable( parseur ) ) 
  parseur.espaces() 
  println( expression_condition( parseur ) ) 
  parseur.espaces() 
  return nothing 
end 

################# Fin du module 
# end

  # Variable message : "bonjour"
source = Source( """

  Condition "offre spéciale" (10) : 
    date.aujourdhui < "2022-02-16" 
    & client.est_membre() 

  Règle "réduction" (90) : 
    Si 
      ?"offre spéciale" 
    Alors 
      panier.reduction( 10, "%" ) 
""" ) 

parseur = Parseur( source ) 

avancer( parseur ) 


