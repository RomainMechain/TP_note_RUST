# TP de Programmation Avancée : Ditherpunk: retour au monochrome

## Présentation du projet : 

L'objectif de ce tp est de s'entrainer à manipuler des images en utilisant la bibliothèque image en Rust. 

## Membre du groupe : 

- Liam Sottier 
- Romain Mechain 

## Question 1 :

Pour créer un nouveau projet cargo il suffit de faire la commande suivante `cargo init tp_note_rust` dans le terminal.

Ensuite il suffit de se rendre dans le dossier créé et d'ajouter la dépendance suivante dans le fichier `Cargo.toml` : 

```toml
[dependencies]
argh = "0.1.13"
image = "0.24.9"
```

## Questions 2 : 

Le type DynamicImage représente une image sous forme de matrice de pixels, chacun sous la forme RGBA. 

Pour obtenir une image sous forme RGB8 il nous suffit d'utiliser la fonction to_rgb8() sur notre image. Nous avons donc le code suivant : 

```rust
let img_path = "iut.jpg";
let img = image::open(img_path)?;
let rgb_img = img.to_rgb8();
rgb_img.save("output_rgb8.png")?;
```

Nous avons l'image suivante :

![alt text](images/output_rgb8.png)

## Questions 3 :

Si nous utilisons cette méthode sur une image avec un canal alpha, puis que nous l'exportons en PNG, le canal alpha sera perdu.

## Question 4 : 

IL est possible de récupérer un pixel avec la commande `img.get_pixel(x, y)`. Nous avons le code suivant pour l'afficher 

```rust
let pixel32_52 = rgb_img.get_pixel(32, 52);
println!("Pixel (32, 52) : {:?}", pixel32_52);
```

## Question 5 : 

Pour changer la couleur d'un pixel sur deux, il suffit de les parcourir et de modifier ceux dont la ligne plus la colonne sont paire. Nous avons donc le code et le résultat suivant : 

```rust
rgb_img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
    if (x + y) % 2 == 0 {
        *pixel = image::Rgb([255, 255, 255]);
    }
});
rgb_img.save("../images/Question5.png")?;
```
![alt text](images/Question5.png)

## Question 6 :

Il est possible de définir la luminosité d'un pixel à partire de son code rgb, on utilise pour cela la formule `0.2126 * rouge + 0.7152 * vert + 0.0722 * bleu`. 

## Question 7 :

Pour implémenter cette méthode, nous avons le code suivant : 

```rust
fn calcule_luminosité(pixel: image::Rgb<u8>) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
```

Nous pouvons donc le tester sur le pixel (32, 52) : 

```rust
let luminosité = calcule_luminosité(*pixel32_52);
println!("Luminosité du pixel (32, 52) : {}", luminosité);
```

Enfin, pour implémenter le traitement sur l'image, on peut utiliser le code suivant : 

```rust
rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
    let luminosité = calcule_luminosité(*pixel);
    if luminosité > 128.0 {
        *pixel = image::Rgb([255, 255, 255]);
    } else {
        *pixel = image::Rgb([0, 0, 0]);
    }
});
rgb_img.save("../images/Question7.png")?;
```

Nous obtenons donc l'image suivante :

![alt text](images/Question7.png)

## Question 8 :
