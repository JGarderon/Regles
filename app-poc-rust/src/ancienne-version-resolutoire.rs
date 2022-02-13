#![allow(warnings, unused)] 

use std::iter::Enumerate; 
// use std::collections::HashMap; 

type IdRegle = usize;
type IdCondition = usize;
type IdClause = usize;
type IdPoids = usize;

type AppelableSimple = fn( contexte: &mut Contexte ) -> bool; 
type AppelableComplexe = fn( contexte: &mut Contexte, declenchable: bool ) -> bool; 

trait ArbreResolvable<T> {
  fn resoudre( self, contexte: &mut Contexte, appelables: &T ) -> bool {
    objet.
  }
}

// #[derive(Debug)]
enum ArbreClauses {
  Ou( Box<ArbreClauses>, Box<ArbreClauses> ),
  Et( Box<ArbreClauses>, Box<ArbreClauses> ),
  Seul( IdClause ) 
} 

// #[derive(Debug)]
struct Contexte { 
  regles: Vec<Regle>, 
  conditions: Vec<Condition>, 
  clauses: Vec<Clause>, 
  tours: Vec<Vec<IdRegle>>, // vecteur groupé de vecteurs triés de règles 
  tour_conditions: Vec<IdCondition>, 
  tour_clauses: Vec<IdClause> 
} 

impl Contexte { 
  fn creer( regles: Vec<Regle>, conditions: Vec<Condition>, clauses: Vec<Clause> ) -> Self {
    Contexte { 
      regles: regles, 
      conditions: conditions, 
      clauses: clauses, 
      tours: vec!(), 
      tour_conditions: vec!(), 
      tour_clauses: vec!() 
    } 
  }
  fn compiler( mut self ) { 
    let mut regroupement = self.regles.iter().map( 
        |r| r.groupe 
    ).collect::<Vec<usize>>(); 
    regroupement.sort();
    regroupement.dedup(); 
    self.tours = regroupement.iter().fold( 
      vec!(), 
      |mut v_n1, valeur_regroupement| {
        let groupe = self.regles.iter().enumerate().fold( 
          vec!(),  
          |mut v_n2, (i, r)| {
            if r.groupe == *valeur_regroupement { 
              v_n2.push( i ) 
            } 
            v_n2 
          }
        ); 
        v_n1.push( groupe ); 
        v_n1 
      }
    ); 
  } 
  fn jouer( mut self ) -> bool { 
    if let Some( tour ) = self.tours.pop() { 

      true 
    } else {
      false 
    }
  }
} 

// #[derive(Debug)]
struct Condition { 
  etat: bool, 
  arbre: ArbreClauses  
}

// #[derive(Debug)]
struct Clause { 
  etat: bool, 
  fct: AppelableSimple 
}

// #[derive(Debug)]
struct Regle { 
  nom: String, 
  groupe: usize, 
  conditions: Vec<IdCondition>, 
  si: Vec<AppelableSimple>, 
  sinon: Vec<AppelableSimple>, 
  finalement: Vec<AppelableComplexe> 
} 

impl Regle {
  fn declencher( self, contexte: &mut Contexte ) -> bool { 
    let declenchable: bool = self.conditions.iter().fold(
      true,  
      |acc, id_condition| { 
        let id_condition_regle = contexte.tour_conditions[*id_condition]; 
        acc && contexte.conditions[id_condition_regle].etat 
      } 
    ); 
    if declenchable { 
      self.si.iter().for_each( 
          |fct| fct( contexte ) 
      ); 
    } else { 
      self.sinon.iter().for_each( 
          |fct| fct( contexte ) 
      ); 
    }; 
    self.finalement.iter().for_each( 
      |fct| fct( contexte, declenchable ) 
    ); 
    declenchable 
  } 
}


fn main() {
  println!("Hello, world!");
}
