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
enum Jeton<T> { 
	Appelable(T), 
	LiaisonOu, 
	LiaisonEt,
	GroupeOuvrant, 
	GroupeFermant 
} 

#[derive(Debug)]
struct Feuille<T> {
	pile: Vec<bool>, 
	action: Vec<Jeton<T>> 
} 

impl Feuille<Clause> { 
	fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	fn resoudre( self )-> bool where Self: Sized { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire des clauses est incorrect" ) 
				} 
			) 
	} 
} 

impl Feuille<Condition> { 
	fn creer() -> Self {
		Feuille { 
			pile: vec!(), 
			action: vec!() 
		} 
	} 
	fn resoudre( self )-> bool where Self: Sized { 
		*&self.pile[1..] 
			.iter()
			.zip( &self.action[..] )
			.fold( 
				self.pile[0], 
				|acc,(etat,liaison)| match liaison {
					Jeton::LiaisonOu => acc | *etat,
					Jeton::LiaisonEt => acc & *etat, 
					_ => panic!( "l'arbre résolutoire des conditions est incorrect" ) 
				} 
			) 
	} 
} 

#[derive(Debug)] 
struct Regle { 
	etat: Option<bool>, 
	conditions: Vec<Jeton<Condition>>, 
	declenchable: bool, 
	declenchee: bool 
}

impl Regle { 
	fn creer( conditions: Vec<Jeton<Condition>> ) -> Self { 
		Regle { 
			etat: None, 
			conditions: conditions, 
			declenchable: false, 
			declenchee: false 
		} 
	} 
	fn preparer( &mut self ) -> Vec<String> { 
		// self.conditions.iter().filter_map( 
		// 	|item| match item { 
		// 		Jeton::Appelable( c ) => Some( c.appelable.clone() ), 
		// 		_ => None 
		// 	}
		// ).collect::<Vec<String>>() 
		vec!() 
	} 
	fn resoudre( &mut self ) { 
		// let mut contexte = vec!( Feuille::<Condition>::creer() ); 
		// for element in &self.conditions { 
		// 	match element { 
		// 		Jeton::Appelable( c ) => { 
		// 			&contexte[..].last_mut().unwrap().pile.push( c.etat ); 
		// 		} 
		// 		Jeton::LiaisonOu | Jeton::LiaisonEt => { 
		// 			&contexte[..].last_mut().unwrap().action.push( element.clone() ); 
		// 		}, 
		// 		Jeton::GroupeOuvrant => { 
		// 			contexte.push( Feuille::creer() ); 
		// 		}, 
		// 		Jeton::GroupeFermant => { 
		// 			let e = contexte.pop().unwrap().resoudre(); 
		// 			&contexte[..].last_mut().unwrap().pile.push( e ); 
		// 		} 
		// 	} 
		// } 
		// let feuille = contexte.pop().unwrap(); 
		// self.etat = Some( feuille.resoudre() ); 
	} 
} 

#[derive(Debug)] 
struct Condition { 
	etat: Option<bool>, 
	clauses: Vec<Jeton<Clause>>, 
	declenchable: bool, 
	declenchee: bool 
} 

impl Condition { 
	fn creer( clauses: Vec<Jeton<Clause>> ) -> Self { 
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
				Jeton::Appelable( c ) => Some( c.appelable.clone() ), 
				_ => None 
			}
		).collect::<Vec<String>>() 
	} 
	fn resoudre( &mut self ) { 
		let mut condition = vec!( Feuille::<Clause>::creer() ); 
		for element in &self.clauses { 
			match element { 
				Jeton::Appelable( c ) => { 
					&condition[..].last_mut().unwrap().pile.push( c.etat ); 
				} 
				Jeton::LiaisonOu | Jeton::LiaisonEt => { 
					&condition[..].last_mut().unwrap().action.push( element.clone() ); 
				}, 
				Jeton::GroupeOuvrant => { 
					condition.push( Feuille::<Clause>::creer() ); 
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


fn main() {

	let liste = vec!( 
		Jeton::GroupeOuvrant, 
			Jeton::Appelable( Clause::creer( "a".to_string() ) ), 
			Jeton::LiaisonOu, 
			Jeton::Appelable( Clause::creer( "b".to_string() ) ), 
			Jeton::LiaisonEt, 
			Jeton::Appelable( Clause::creer( "c".to_string() ) ), 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::GroupeOuvrant, 
			Jeton::Appelable( Clause::creer( "d".to_string() ) ), 
			Jeton::LiaisonEt, 
			Jeton::GroupeOuvrant, 
				Jeton::Appelable( Clause::creer( "e".to_string() ) ), 
				Jeton::LiaisonOu, 
				Jeton::Appelable( Clause::creer( "f".to_string() ) ), 
			Jeton::GroupeFermant, 
		Jeton::GroupeFermant, 
		Jeton::LiaisonEt, 
		Jeton::Appelable( Clause::creer( "g".to_string() ) ) 
	); 

	let mut condition = Condition::creer( liste ); 
	condition.resoudre(); 

	println!( 
		"résultat final = {:?}",  
		condition.etat 
	); 

} 




