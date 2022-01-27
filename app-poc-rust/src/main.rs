
use std::collections::HashMap; 

// #[derive(Debug)]
struct Contexte<'t_cd,'t_cl> { 
    regles: HashMap<String, Regle>, 
    conditions: HashMap<String, Condition>, 
    clauses: HashMap<String, Clause>, 
    tours: Vec<(usize, Vec<Regle>)>, // vecteur groupé (poids du groupe + vecteur trié des règles) 
    tour_conditions: HashMap<&'t_cd str, &'t_cd Condition>, 
    tour_clauses: HashMap<&'t_cl str, &'t_cl Clause>, 
} 

impl Contexte { 
    fn deduire( self ) {
        
    }
}

// #[derive(Debug)]
struct Condition { 
    etat: bool 
}

// #[derive(Debug)]
struct Clause { 
    etat: bool 
}

// #[derive(Debug)]
struct Regle { 
    nom: String, 
    poids: (usize,usize), // groupe, trie 
    conditions: Vec<String>, 
    si: Option<fn( contexte: &mut Contexte ) -> bool>, 
    sinon: Option<fn( contexte: &mut Contexte ) -> bool>, 
    finalement: Option<fn( contexte: &mut Contexte, declenchable: bool ) -> bool> 
} 

impl Regle {
    fn declencher( self, contexte: &mut Contexte ) -> bool {
        let declenchable: bool = self.conditions.iter().fold(
            true,  
            |acc, c_nom| acc && contexte.tour_conditions.get( &c_nom[..] ).unwrap().etat 
        ); 
        let fin = if declenchable { 
            if let Some( f ) = self.si {
                f( contexte ); 
                true
            } else {
                false 
            }
        } else {
            if let Some( f ) = self.sinon {
                f( contexte ); 
                true 
            } else { 
                false
            } 
        }; 
        if fin {
            if let Some( f ) = self.finalement {
                f( contexte, declenchable ); 
            } 
        } 
        fin 
    }
}


fn main() {
    println!("Hello, world!");
}
