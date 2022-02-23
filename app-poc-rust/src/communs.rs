
#[derive(Debug)]
pub enum Types {
	Nombre(f64), 
	Texte(String), 
	Conditionnel(String), 
	Appelable(String, Vec<Types>), 
	Et, 
	Ou, 
}


