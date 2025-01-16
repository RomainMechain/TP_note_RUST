# TP de Programmation Avancée : Ditherpunk: retour au monochrome

## Présentation du projet : 

L'objectif de ce tp est de s'entrainer à manipuler des images en utilisant la bibliothèque image en Rust. 

## Membre du groupe : 

- Liam Sottier 
- Romain Mechain 

## Questions 2 : 

Le type DynamicImage représente une image sous forme de matrice de pixels, chacun sous la forme RGBA. 

Pour obtenir une image sous forme RGB8 il nous suffit d'utiliser la fonction to_rgb8() sur notre image.

## Questions 3 :

Si nous utilisons cette méthode sur une image avec un canal alpha, puis que nous l'exportons en PNG, le canal alpha sera perdu.

