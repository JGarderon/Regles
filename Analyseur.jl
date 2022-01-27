module ModAnalyseur

using ..ModJetons

mutable struct Source 
  texte::String  
  obtenir::Function 
  function Source( texte::String ) 
    objet = new() 
    objet.texte = texte 
    objet.obtenir = function ( position::Int, taille::Int )
      return SubString( 
        objet.texte, 
        position,  
        position+taille 
      )  
    end 
    return objet 
  end 
end

mutable struct Parseur 
  source::Source 
  position::Int 
  espaces_liste::NTuple{4,Char} 
  jetons::ModJetons.Jetons 
  espaces::Function 
  comparer::Function 
  avancer::Function 
  function Parseur( source::Source ) 
    objet = new()
    objet.source = source 
    objet.position = 1 
    objet.espaces_liste = ( ' ', '\t', '\n', '\r' ) 
    objet.jetons = ModJetons.Jetons()
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
    return objet 
  end 
end 

end