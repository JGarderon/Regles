

function action_executer () { 
	echo "$@"
	read reponse 
	case $reponse in 
		"v" | "f")
			echo "# action '$@': retour '$reponse'" 
			;; 
		*) 
			echo "# action '$@': valeur de réponse incorrecte"
			exit 1 
			;; 
	esac 
}

function regle_executer() { 
	eval "for item in \"\${$1_alors[@]}\"; do action_executer \$item ; done"
	eval "for item in \"\${$1_sinon[@]}\"; do action_executer \$item ; done"
	eval "for item in \"\${$1_finalement[@]}\"; do action_executer \$item ; done"

	# eval "conditions=(\${$1_conditions[@]})" 
	# eval "alors=(\${$1_alors[@]})" 
	# eval "sinon=(\${$1_sinon[@]})" 
	# eval "finalement=(\${$1_finalement[@]})" 
	# for item in "${regle_1_alors[@]}"
	# do
	# 	echo "--- $item"
	# done 
} 

### -------------------------------- 

declare -a condition_1=( 
	"clause_virt_1"
	"&&"
	"clause_virt_2"
)
declare -a condition_2=( 
	"clause_virt_1"
	"&&"
	"clause_virt_3"
)

### -------------------------------- 

declare -a regles=( "regle_1" ) # "regle_2" ) 

declare -a regle_1_conditions=( "(" "condition_1" ")" ) 
declare -a regle_1_alors
regle_1_alors[0]="act_virtuel_1 \$arg1" 
regle_1_alors[1]="act_virtuel_2 \$arg1" 
declare -a regle_1_sinon=( "act_virtuel_2 \$arg1" ) 
declare -a regle_1_finalement=( "act_virtuel_3 \$arg1" ) 

declare -a regle_2_conditions=( "(" "condition_1" "&&" "condition_2" ")" ) 
declare -a regle_2_alors=( "act_virtuel_1 \$arg1" ) 
declare -a regle_2_sinon=( "act_virtuel_2 \$arg1" ) 
declare -a regle_2_finalement=( "act_virtuel_3 \$arg1" ) 

### -------------------------------- 

for regle_nom in "${regles[@]}"
do 
	regle_executer $regle_nom 
done 












# function virtuel_vrai() { 
# 	return 1; 
# }
# function virtuel_faux() { 
# 	return 1; 
# }

# declare -A clauses_appelable=( 
# 	["C1"]="virtuel_vrai" 
# 	["C2"]="virtuel_faux" 
# 	["C3"]="virtuel_vrai" 
# ) 

# declare -A clauses_etat=( 
# 	["C1"]="" 
# 	["C2"]=""
# 	["C3"]="" 
# ) 

# # echo "-- ${!clauses_etat[@]}" 

# declare -a regles_liste=("r1" "r2") 

# declare -a r1=( "C1"  )

# # echo "-- ${truc[@]}" 


