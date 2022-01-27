# println(PROGRAM_FILE); for x in ARGS; println(x); end 

include( "Jetons.jl" ) 
using .ModJetons 

include( "Analyseur.jl" ) 
using .ModAnalyseur 

include( "Grammaire.jl" ) 
using .ModGrammaire 

source = ModAnalyseur.Source( """
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

parseur = ModAnalyseur.Parseur( source ) 

print( ModGrammaire.avancer( parseur ) ) 











# function mycompare(a, b)::Cint 
#     println("mycompare($a, $b)") 
#     return (a < b) ? -1 : ((a > b) ? +1 : 0) 
#   end 

# mycompare_c = @cfunction(mycompare, Cint, (Ref{Cdouble}, Ref{Cdouble})) 
# A = [12, 1.3, 4.4, 3.1, -2.7] 
# ccall(:qsort, Cvoid, (Ptr{Cdouble}, Csize_t, Csize_t, Ptr{Cvoid}), 
#              A, length(A), sizeof(eltype(A)), mycompare_c) 
# println(A)  

# function double!(a::Int64) 
#     a *= 2 
# end 

# function test!(a::Vector{Int8}) 
#   """coucou""" 
#     a[2] += 5 
# end 

# i = 2 

# y = double!( i ) 

# println( y ) 
# println( i ) 

# z = Int8[1, 2, 3] 

# println( test!( z ) ) 
# println( z ) 

