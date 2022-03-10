//! # '_Règle_'. Organiser son traitement. Sans se désorganiser.
//!
//! Le programme _Règle_ est un moteur de règles, c'est-à-dire qu'il exécute des règles déclarées, suivant leur état individuel (schéma habituel _Si... Alors... Sinon... Finalement..._). Le moteur cherchera dans son fonctionnement, à optimiser fortement la gestion des états, notamment si une règle n'a pas été déclenchée afin d'éviter de refaire des opérations. 
//!
//! Le programme fonctionne à partir des E/S standard, et repose sur des lignes en simili-Bash pour communiquer avec les autres processus. A ce jour, seulement un mode dit "asservi" est supporté : le programme est supervisé par un exécuteur, qui jouera les demandes (appelable de contexte ou d'actions). C'est le programme superviseur qui gère les variables ; le moteur de règle n'a notion que d'états booléens pour savoir si une règle pourrait être déclenchée ou non. 
//! 
//! ## Principales explications et exemples 
//! 
//! D'abord quelques points importants : 
//!   - l'_exécuteur_, qui est ici un processus qui supervise le programme _Règle_ (c-à-d qui l'exécute et garde le contrôle de ses E/S), est le seul qui a accès aux variables et qui a la charge d'exécuter les opérations ; 
//!   - ... 
//! 
//! _nb : Pour des fins de lisibilités, il est préférable de fortement séparer les différents éléments de rédaction de votre jeux de règles : cela facilite la relecture et le parseur n'a pas de restriction._ 
//! 
//! ### Types de données disponibles 
//! 
//! Le programme gère dans ses déclarations de règles seulement trois types : les 'Textes', les 'Nombres' et les 'Variables'. Les 'Appelables' (qui peuvent servir à définir un état ou appeler une action) comme les renvois ou les conditionnels, sont des types dérivés des types fondamentaux (sous le format d'une symbole suivit d'un Texte). 
//! 
//! ```
//! "Ceci est texte" # Texte, toujours entre guillemets 
//! ```
//! 
//! ```
//! 20    # Nombre 
//! 20.5  # Nombre 
//! -20.5 # Nombre 
//! ```
//! 
//! ```
//! ma_variable # Variable  
//! ma_fonction() # Variable + arguments* -> Appelable (clause ou action) 
//! ?"ma condition" # symbole + Texte -> Conditionnel 
//! !"mon renvoi vers une règle" # symbole + Texte => Renvoi 
//! ```
//! 
//! ### Définir une 'Variable' 
//!
//! Une Variable est la déclaration de la liaison entre une Variable et une valeur d'un type des deux types fondamentaux (Texte ou Nombre). 
//!
//! ```
//! Variable message_bonjour: 
//!   "bonjour" 
//! ```
//!
//! Notez que la définition des variables n'est pas dédié directement au moteur : ce dernier doit simplement signifier les valeurs de contexte initial à l'exécuteur (le programme distant). __Si plusieurs définitions sont trouvées, la plus récente sera gardée.__ 
//!
//! ### Définir une 'Condition' 
//! 
//! Une Condition est le regroupement de plusieurs 'Clauses'. Une clause est un état, fruit de l'exécution d'un appelable passé à l'exécuteur : une clause _ne peut pas être une valeur_. Plusieurs clauses peuvent être liées ensembles au travers deux opérateurs logiques : `et` et `ou`. La condition peut aussi n'avoir qu'une seule clause, comme suit : 
//! 
//! ```
//! Condition "est membre": 
//!   client.est_membre( 50, "jours" ) 
//! ```
//! 
//! Si aucune parenthèse ou regroupement n'est admis dans les conditions, vous pouvez appeler d'autres conditions (comme étant des clauses) au sein d'une parente. Cela revient à "regrouper" et le résultat se comportera comme si vous aviez eu recourt à des parenthèses. En plus efficace : pas besoin d'avoir à retaper les mêmes conditions et les mêmes clauses à chaque nouvelle règle... 
//! 
//! ```
//! Condition "offre spéciale": 
//!   ?"est membre"
//!   et 
//!   client.total_historique( ceci_est_une_variable_locale_a_l_executeur, ">", 1000 ) 
//! ```
//! 
//! Une clause définie de la même façon à plusieurs endroits (y compris dans l'ordre des arguments), aura un seul objet en mémoire la représentant au sein d'un contexte de résolution. Ainsi il n'est pas dommageable dans la création des conditions ou dans un jeu de conditions embriquées, d'avoir plusieurs fois le même appels à une même clause : il n'y aura qu'une seule appelée / exécutée à la fin. 
//! 
//! Il est à noter que si l'exécuteur exécute un appelable qui a des effets de bord, le résultat pourrait être inattendu (et catastrophique). Sachant qu'une clause peut être seulement exécutée une fois au travers de plusieurs conditions voire de plusieurs règles, la fonction distante doit être la plus "pure" (au sens où elle marginalise le risque d'avoir un retour différent dans un contexte identique, ou de provoquer un changement d'état de l'exécuteur lui-même). 
//! 
//! ### Définir une 'Règle' 
//! 
//! La Règle est le plus haut niveau et regroupe à la fois des conditions (donc les clauses qui y sont contenues), ainsi que des appelables, c'est-à-des fonctions demandées à l'exécuteur (le processus distant). Une règle se compose de plusieurs parties de corps, et doit forcément avoir avoir son premier bloc : 'Si'. Ce bloc déclare la ou les conditions de la règles. 
//! 
//! Les autres corps sont optionnels, mais doivent seulement respecter un ordre d'apparition : 
//!   - 'Alors' : sera exécuté seulement si la ou les conditions retournent un état négatif ; 
//!   - 'Sinon' : sera exécuté seulement si la ou les conditions retourne un état négatif ; 
//!   - 'Finalement' : sera exécuté si le corps 'Alors' ou 'Sinon' est exécuté. 
//! 
//! Attention, le moteur ne contrôle pas la logique d'ensemble : un corps 'Finalement', sans un corps 'Alors' ou 'Sinon', ne sera jamais exécuté. 
//! 
//! Les règles sont triées entre elles par un poids, qui est un élément indispensable de la déclaration d'une règle. Ce poids est un nombre (de type [`f64`]) et peut être donc être négatif. Le tri se fait de la valeur la plus faible (qui est tentée en premier) à la plus élevée (qui est tenté en dernier). 
//! 
//! En terme du programme, le passage d'une règle à l'autre se fait en incrément de 1 sur la position au sein d'un vecteur de Règles. Pour permettre des embranchements ou des boucles, il est possible de d'indiquer comme appelable, la formule spéciale du Renvoi. C'est-à-dire la déclaration d'un nom d'une autre règle, qui sera donc la prochaine position qui sera jouée. Un renvoi dont le nom nul (c'est-à-dire une absence de texte entre les guillements), équivaut à avoir la position la plus élevée et donc arrêter le moteur de règles pour ce contexte. 
//! 
//! Voici un exemple de règle classique : 
//! 
//! ```
//! Règle "réduction" (10): 
//!   Si 
//!     ?"offre spéciale" et ?"date offre spéciale" 
//!   Alors 
//!     panier.reduction( 10.5, "%" ), 
//!     panier.notification( message_bonjour, "bravo, vous êtes un client fidèle" ) 
//!   Sinon 
//!     panier.reduction( -5, "%" ) 
//!   Finalement 
//!     panier.mettre_a_jour() 
//! ```
//! 
//! Vous pouvez aussi sortir rapidement de votre parcours des règles (du contexte) de cette façon, en fonction d'une condition réussie : 
//! 
//! ```
//! Règle "réduction applicable" (0): 
//!   Si 
//!     ?"réduction maximale atteinte" 
//!   Alors 
//!     !"" 
//! ```
//! 
//! ## Envie de tester ? 
//! 
//! Une fois le projet Git cloné localement, Python et Rust installés sur votre ordinateur, vous pouvez jouer la commande suivante depuis le répertoire dans votre consoole : 
//! 
//! ```bash
//! cargo build --release && RESOLUTION_TYPE=asservi REGLES_SOURCE=./regles.txt ./superviseur.py 
//! ``` 
//! 
//! ... Vous verrez alors apparaître une sortie similaire (plus ou moins fournie, les états du simulateur sont aléatoires) à ceci au bout d'une seconde ou deux : 
//! 
//! ```
//!    Compiling Regle v1.0.0 (/home/julien/Developpement/graphe-inference/regle)
//!     Finished release [optimized] target(s) in 1.26s
//! 140133526019376 exécuteur - initier ---> ! 0 sur 1 essai(s)
//! 140133526019376 exécuteur - définir ---> definir "message_bonjour" "bonjour"
//! 140133526019376 exécuteur - définir ---> definir "taux_max" 25
//! 140133526019376 exécuteur - faire ---> executer panier.total_reduction_verifier ">" $taux_max "%"
//! 140133526019376 exécuteur - faire ---> executer client.est_membre 50 "jours"
//! 140133526019376 exécuteur - faire ---> executer client.total_historique $ceci_est_une_variable_locale_a_l_executeur ">" 1000
//! 140133526019376 exécuteur - faire ---> executer date.aujourdhui "<" "2022-02-16"
//! 140133526019376 exécuteur - faire ---> executer panier.reduction 10.5 "%"
//! 140133526019376 exécuteur - faire ---> executer panier.notification $message_bonjour "bravo, vous êtes un client fidèle"
//! 140133526019376 exécuteur - faire ---> executer panier.mettre_a_jour
//! 140133526019376 exécuteur - initier ---> ! 1 sur 1 essai(s)
//! ``` 
//! 
//! Bravo, vous avez joué un jeu de règles de test ! 
//! 
//! ## Licences et support 
//!
//! Fruit d'un travail de recherche et de réflexion, ce programme est mis à disposition sans aucune garantie de fonctionnement, et pour celle restriction, le respect [de la licence MIT](https://fr.wikipedia.org/wiki/Licence_MIT). Il est pensé prioritaire pour des systèmes UNIX/Linux. 
//!
//! Pour toute garantie ou développement particulier, vous pouvez vous rapprocher de l'auteur Julien Garderon ([Linkedin](https://www.linkedin.com/in/julien-garderon-13a3a920), [Twitter](https://twitter.com/intelligencepol), [Github](https://github.com/JGarderon)). 
//!
#![allow(warnings, unused)] 

/// Le module [`resolution`] s'occupe de gérer les contextes, c'est-à-dire l'objet en mémoire qui porte l'état des clauses et des conditions et règles compilées, en gardant le lien avec l'environnement de règles initial. 
mod resolution; 
use crate::resolution::contexte::Contexte;
use crate::resolution::contexte::construire as contexte_resolution; 
use crate::resolution::executer as resolution_executer; 

/// Le module [`communs`] porte les types généraux utilisés dans les règles. A l'avenir, il sera étendu pour augmenter les fonctionnalités courantes du programme. 
mod communs; 
use crate::communs::Types; 

/// Second module utilisés après [`resolution`], [`grammaire`] porte les fonctions, macros et structures diverses qui permettent de partir d'un fichier de règles, vers un environnement complet en mémoire. 
mod grammaire; 
use crate::grammaire::constructeur::construire as environnement_construire; 

/// La fonction `main` n'a pour seule tâche que d'appeler la fonction de résolution globale [`resolution::executer`] et de gérer l'affichage sur le `stderr` d'une erreur rencontrée par le programme. 
///
/// 
///
fn main() { 
	std::process::exit( match resolution_executer() { 
		Ok( _ ) => 0, 
		Err( erreur ) => { 
			eprintln!( "# erreur : {}", erreur ); 
			1 
		} 
	} ); 
} 



