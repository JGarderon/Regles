#![allow(warnings, unused)] 

#[derive(Clone,Debug)] 
struct Clause { 
	appelable: String, 
	etat: bool 
} 

impl Clause { 
	fn creer( appelable: String ) -> Self { 
		Clause { 
			appelable: appelable, 
			etat: false 
		} 
	} 
} 

#[derive(Clone,Debug)] 
enum Jeton { 
	Clause(Clause), 
	LiaisonOu, 
	LiaisonEt,
	GroupeOuvrant, 
	GroupeFermant 
} 

#[derive(Debug)] 
struct Condition { 
	etat: Option<bool>, 
	clauses: Vec<Jeton>, 
	declenchable: bool, 
	declenchee: bool 
} 

impl Condition { 
	fn creer( clauses: Vec<Jeton> ) -> Self { 
		Condition { 
			etat: None, 
			clauses: clauses, 
			declenchable: false, 
			declenchee: false 
		} 
	} 
	fn preparer( &mut self ) -> Vec<String> { 
		self.clauses.iter().filter_map( 
			|item| match item { 
				Jeton::Clause( c ) => Some( c.appelable.clone() ), 
				_ => None 
			}
		).collect::<Vec<String>>() 
	} 
	fn resoudre( &mut self ) { 
		let mut condition = vec!( Feuille::creer() ); 
		for element in &self.clauses { 
			match element { 
				Jeton::Clause( c ) => { 
					&condition[..].last_mut().unwrap().pile.push( c.etat ); 
				} 
				Jeton::LiaisonOu | Jeton::LiaisonEt => { 
					&condition[..].last_mut().unwrap().action.push( element.clone() ); 
				}, 
				Jeton::GroupeOuvrant => {
					condition.push( Feuille::creer() ); 
				}, 
				Jeton::GroupeFermant => { 
					let e = condition.pop().unwrap().resoudre(); 
					&condition[..].last_mut().unwrap().pile.push( e ); 
				} 
			} 
		} 
		let feuille = condition.pop().unwrap(); 
		self.etat = Some( feuille.resoudre() ); 
	} 
} 

#[derive(Debug)]
struct Feuille {
	pile: Vec<bool>, 
	action: Vec<Jeton> 
} 

impl Feuille {
	fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	fn resoudre( self ) -> bool { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire est incorrect" ) 
				} 
			) 
	} 
}

fn main() {

	let liste = vec!( 
		Jeton::GroupeOuvrant, 
			Jeton::Clause( Clause::creer( "a".to_string() ) ), 
			Jeton::LiaisonOu, 
			Jeton::Clause( Clause::creer( "b".to_string() ) ), 
			Jeton::LiaisonEt, 
			Jeton::Clause( Clause::creer( "c".to_string() ) ), 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::GroupeOuvrant, 
			Jeton::Clause( Clause::creer( "d".to_string() ) ), 
			Jeton::LiaisonEt, 
			Jeton::GroupeOuvrant, 
				Jeton::Clause( Clause::creer( "e".to_string() ) ), 
				Jeton::LiaisonOu, 
				Jeton::Clause( Clause::creer( "f".to_string() ) ), 
			Jeton::GroupeFermant, 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::Clause( Clause::creer( "g".to_string() ) ) 
	); 

	let mut condition = Condition::creer( liste ); 
	condition.resoudre(); 

	println!( 
		"résultat final = {:?}",  
		condition.etat 
	); 

} 




