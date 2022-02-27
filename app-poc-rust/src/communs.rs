
#[derive(Debug,PartialEq,PartialOrd,Clone)] 
pub enum Types {
	Nombre(f64), 
	Texte(String), 
	Variable(String), 
	Conditionnel(String), 
	Appelable(String, Option<bool>, Vec<Types>), 
	Et, 
	Ou, 
}



