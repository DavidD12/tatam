- vérifier l'absence de cycle dans les définitions.
- duplicate parameters

# Search
- check that bound objective is a value

# Propagate
- doit etre récussif: si propagation rapeller propagate

# Init/Inv/Trans
- rendre le nommage optionnel

# Time
- add LTL restrictions in check_time

# revoir les niveaux de verbosité (éviter le debug en 3)

# TODO function:
- check if return type is interval then function must be bounded !

# propagate:
- revoir et eventuellement supprimer les simplification et ne faire que la propagation des definitions !

# Check
- parameter size

# Remove le "=" dans prop = F true

# Remove propagate ???

# Unresolved
- changer le Apply en FunCall(fun_id, params)
- ajouter un UnresolvedFunCall
- idem avec les scope: ajouter un UnresolvedScopeVar et un ScopeVar

# Timeout
- ajouter un timeout global
- ajouter le timeout dans le solveur smt
- prise en compte dans l'optimize et dans la solution : currest best solution (good solution)

# smt thread
- ajouter une option nb threads pour la partie smt.

# documentation
- minimale disponible avant la présentation de Jeudi
- traduction minimale du problème de planif avec horloges à plat
- finaliser les exemples "robots"