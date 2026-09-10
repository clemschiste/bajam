# A faire
## Corriger upload
- Ai-je bien reçu une image ? Est ce que cette image à bien un id unique de heartbeat (inverser par rapport à ce qui est fait actuellement)
Actuellement :
1) Upload image -> Intégration au dossier uploads avec un UUid.jpg
2) Upload heartbeat -> Ajout du Uuid (récupéré lors du post de l'upload) pour lier les deux

A faire :
1) Upload heartbeat -> Création et récupération d'un id_image ?
2) Upload image -> Refusé si :
 - Pas auth (mdp dans un premier temps)
 - Aucun heartbeat ne dispose de cette foreign_key.
 - Un fichier existe déjà avec ce nom.
  -> Maintenant que se passe-t-il si le fichier du server est supprimé. Ca laisse le champ libre à une insertion.
    -> Soit je créé une table qui stocke l'historique des uuid id_upload. Si je veux "remplacer" l'image et que j'ai les droits je change la fk du post.
    -> Soit je met un délai temporel de type : pas possible d'inserer sur un post après tant de tant après le timstamp... pas très agréable.

*Question que je me pose.*
Est ce courant de chain des routes comme cela /heartbeat puis /upload ? Je pense que oui car upload je peux le réutiliser en chain avec un heartbeat/edit.

- Compresser les images par défault à la réception des bytes (600 width)

## Add auth
Pour post et upload

## Autre
- Fleches pour passer aux heartbeats précédent ou directement en cliquant sur le marqueur.
- Check validity of data (long, lat)
- Faire un truc joli

# Problèmes
- Actuellement **n'importe qui** peut publier une image via upload/ -> auth nécéssaire
- Actuellement upload/ accepte les image, n'importe lesquels sans conditions -> auth nécéssaire + lien avec heartbeat obligatoire
il est plus logique de "d'abord" avoir un post et le lier ensuite à une image pour que le minimum soit le pin (visible) et pas l'image
possiblement invisible et s'accumulant coté serveur.
- Actuellement n'importe quel type de fichier peut etre push ave curl -F "image=@....md" par exemple
