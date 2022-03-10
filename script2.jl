
mutable struct Toto
  a::Int
  additionner::Function # champ pour porter la méthode 
  function Toto( a::Int ) # constructeur 
    this = new() # pour l'encapsulation 
    this.a = a 
    this.additionner = function (b::Any) # liaison via une fct anonyme...
      TotoAdditionner( this, b ) # ... on garde ainsi le composition 
    end 
    return this 
  end 
  function Toto( a::String ) # constructeur (autre composition)
    this = new() 
    this.a = 99 
    this.additionner = function (b::Any) 
      TotoAdditionner( this, b ) 
    end 
    return this 
  end 
end 

function TotoAdditionner( this::Any, b::Int )
  this.a = this.a + b 
end 

function TotoAdditionner( this::Any, b::String )
  this.a = 88 
end 

t1 = Toto( 0 ) # "instanciation" de l'objet 

t1.additionner( 1 ) 

println( t1 ) # Toto(1, var"#1#2"{Toto}(Toto(#= circular reference @-2 =#)))

println( t1.a ) # 1 

t2 = t1 

t2.additionner( "ok" ) 

println( t2 ) # Toto(88, var"#1#2"{Toto}(Toto(#= circular reference @-2 =#)))

println( t2.a ) # 88 

t3 = Toto( "texte" ) 

t3.additionner( 1 )  

println( t3.a ) # 1 

t3.additionner( "1" ) 

println( t3.a ) # 88 



