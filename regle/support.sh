#!/usr/bin/env bash

declare -A client

client[total]=890
client[membre]=

function client.total_historique { 
	case "$1" in 
		">") ;; 
		"<") ;; 
		"==") ;; 
		"!=") ;; 
		*) 
			echo 'erreur' 
			return 
	esac 
	if [[ ! "$2" =~ ^[+-]?[0-9]+([.][0-9]+)?$ ]]
	then 
		echo "erreur l'argument n'est pas un nombre" 
		return 
	fi 
	eval "if ((${client[total]} $@)) ; then echo 'true' ; else echo 'false' ; fi" \
		|| echo "erreur ceci est un message d'explication" 
}

while read -r toto
do
	eval $toto 
done 

# echo 'client.total_historique "<" 100' | ./support.sh 