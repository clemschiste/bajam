# A faire
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
