
#[derive(Clone,Debug)] 
pub enum Jeton<T> { 
	Appelable(T), 
	LiaisonOu, 
	LiaisonEt,
	GroupeOuvrant, 
	GroupeFermant 
} 
