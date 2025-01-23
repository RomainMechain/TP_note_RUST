use argh::FromArgs;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// la premiere couleur
    #[argh(option, default = "String::from(\"FFFFFF\")")]
    color1: String,

    /// la seconde couleur
    #[argh(option, default = "String::from(\"000000\")")]
    color2: String,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode

    
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Tramage(OptsTramage),
    Bayer(OptsBayer),
    //DiffusionMono(OptsDiffusionMono),
}

// #[derive(Debug, Clone, PartialEq, FromArgs)]
// #[argh(subcommand, name="diffusion_mono")]
// /// Rendu de l’image par diffusion d’erreur en monochrome
// struct OptsDiffusionMono {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="bayer")]
/// Rendu de l’image par tramage d’ordre n
struct OptsBayer {
    /// l’ordre de la matrice de Bayer
    #[argh(option)]
    ordre: usize
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="tramage")]
/// Rendu de l’image par tramage
struct OptsTramage {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
use image::{DynamicImage, GenericImageView, ImageError,Rgb};

// Exercice 7
fn calcule_luminosité(pixel: image::Rgb<u8>) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, String> {
    
    if hex.len() != 6 {
        return Err(format!("La couleur '{}' n'est pas valide. Utilisez le format #RRGGBB.", hex));
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Composante rouge invalide")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Composante verte invalide")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Composante bleue invalide")?;
    Ok(Rgb([r, g, b]))
}

fn calcule_distance_couleur(pixel1: Rgb<u8>, pixel2: Rgb<u8>) -> f32 {
    let r_diff = pixel1[0] as f32 - pixel2[0] as f32;
    let g_diff = pixel1[1] as f32 - pixel2[1] as f32;
    let b_diff = pixel1[2] as f32 - pixel2[2] as f32;
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).sqrt()
}

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    if !std::path::Path::new(&args.input).exists() {
        eprintln!("Erreur : Le fichier d'entrée '{}' n'existe pas.", args.input);
        std::process::exit(1);
    }

    let img = image::open(&args.input)?;
    let mut rgb_img = img.to_rgb8();
    //rgb_img.save("output_rgb8.png")?;

    // Exercice 4
    let pixel32_52 = rgb_img.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel32_52);

    // Exercice 5 
    // rgb_img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
    //     if (x + y) % 2 == 0 {
    //         *pixel = image::Rgb([255, 255, 255]);
    //     }
    // });
    // rgb_img.save("../images/Question5.png")?;

    //exercice 7
    // let luminosité = calcule_luminosité(*pixel32_52);
    // println!("Luminosité du pixel (32, 52) : {}", luminosité);

    // rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
    //     let luminosité = calcule_luminosité(*pixel);
    //     if luminosité > 128.0 {
    //         *pixel = image::Rgb([255, 255, 255]);
    //     } else {
    //         *pixel = image::Rgb([0, 0, 0]);
    //     }
    // });
    // rgb_img.save("../images/Question7.png")?;

    // exercice 8
    // let color1 = hex_to_rgb(&args.color1).expect("Première couleur invalide");
    // let color2 = hex_to_rgb(&args.color2).expect("Deuxième couleur invalide");

    // let img = image::open(&args.input)?;
    // let mut rgb_img = img.to_rgb8();

    // rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
    //     let luminosité = calcule_luminosité(*pixel);
    //     if luminosité > 128.0 {
    //         *pixel = color1;
    //     } else {
    //         *pixel = color2;
    //     }
    // });

    fn generation_matrice_bayer(ordre: usize) -> Vec<Vec<u32>> {
        if ordre == 0 {
            return vec![vec![0]];
        }

        let matrice_ordre_precedent = generation_matrice_bayer(ordre - 1);
        let taille = matrice_ordre_precedent.len();
        let mut matrice = vec![vec![0; taille * 2]; taille * 2];

        for i in 0..taille {
            for j in 0..taille {
                let valeur = matrice_ordre_precedent[i][j];
                matrice[i][j] = 4 * valeur;
                matrice[i + taille][j] = 4 * valeur + 2;
                matrice[i][j + taille] = 4 * valeur + 3;
                matrice[i + taille][j + taille] = 4 * valeur + 1;
            }
        }

        return matrice;
    }

    
    // rgb_img.save("../images/Question8.png")?;

    match args.mode {
        Mode::Seuil(_) => {
            let color1 = hex_to_rgb(&args.color1).expect("Première couleur invalide");
            let color2 = hex_to_rgb(&args.color2).expect("Deuxième couleur invalide");

            rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
                let luminosité = calcule_luminosité(*pixel);
                if luminosité > 128.0 {
                    *pixel = color1;
                } else {
                    *pixel = color2;
                }
            });
        }
        Mode::Palette(opts) => {
            let couleurs = vec![
                Rgb([0, 0, 0]),       // Noir
                Rgb([255, 255, 255]), // Blanc
                Rgb([255, 0, 0]),     // Rouge
                Rgb([0, 255, 0]),     // Vert
                Rgb([0, 0, 255]),     // Bleu
                Rgb([255, 255, 0]),   // Jaune
                Rgb([0, 255, 255]),   // Cyan
                Rgb([255, 0, 255]),   // Magenta
            ];

            if opts.n_couleurs == 0 || opts.n_couleurs > couleurs.len() {
                eprintln!(
                    "Erreur : Le nombre de couleurs doit être entre 1 et {}.",
                    couleurs.len()
                );
                std::process::exit(1);
            }

            let palette = &couleurs[..opts.n_couleurs];

            rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
                let mut meilleure_distance = f32::MAX;
                let mut meilleure_couleur = palette[0];

                for &couleur in palette {
                    let distance = calcule_distance_couleur(*pixel, couleur);
                    if distance < meilleure_distance {
                        meilleure_distance = distance;
                        meilleure_couleur = couleur;
                    }
                }

                *pixel = meilleure_couleur;
            });
        }
        Mode::Tramage(opts) => {
            let mut rng = rand::thread_rng();
            let mut seuil = 128.0;
            rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
                let luminosité = calcule_luminosité(*pixel);
                seuil = rng.gen_range(0.0..255.0);
                if luminosité > seuil {
                    *pixel = image::Rgb([255, 255, 255]);
                } else {
                    *pixel = image::Rgb([0, 0, 0]);
                }
            });
        }
        Mode::Bayer(opts) => {
            let matrice_bayer = generation_matrice_bayer(opts.ordre);
            let taille = matrice_bayer.len();
            rgb_img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
                let luminosité = calcule_luminosité(*pixel);
                let valeur = matrice_bayer[(x % taille as u32) as usize][(y % taille as u32) as usize] as f32;
                if luminosité > valeur {
                    *pixel = image::Rgb([255, 255, 255]);
                } else {
                    *pixel = image::Rgb([0, 0, 0]);
                }
            });
        }
        // Mode::DiffusionMono(opts) => {
        //     let largeur = rgb_img.width();
        //     let hauteur = rgb_img.height();
        //     rgb_img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        //         let luminosité = calcule_luminosité(*pixel);
        //         let mut erreur = 0.0;
        //         let mut blanc = true;
        //         if luminosité > 128.0 {
        //             *pixel = image::Rgb([255, 255, 255]);
        //             erreur = 255.0 - luminosité;
        //         } else {
        //             *pixel = image::Rgb([0, 0, 0]);
        //             erreur = luminosité;
        //             blanc = false;
        //         }
        //         // Ajout de l'erreur sur le pixel de droite 
        //         if x + 1 < largeur {
        //             let pixel_droite = rgb_img.get_pixel(x + 1, y);
        //             if blanc {
        //                 let r = pixel_droite[0] as f32 + erreur * 0.5;
        //                 let g = pixel_droite[1] as f32 + erreur * 0.5;
        //                 let b = pixel_droite[2] as f32 + erreur * 0.5;
        //                 rgb_img.put_pixel(x + 1, y, image::Rgb([r as u8, g as u8, b as u8]));
        //             } else {
        //                 let r = pixel_droite[0] as f32 - erreur * 0.5;
        //                 let g = pixel_droite[1] as f32 - erreur * 0.5;
        //                 let b = pixel_droite[2] as f32 - erreur * 0.5;
        //                 rgb_img.put_pixel(x + 1, y, image::Rgb([r as u8, g as u8, b as u8]));
        //             }
        //         }
        //         // Ajout de l'erreur sur le pixel en bas
        //         if y + 1 < hauteur {
        //             let pixel_bas = rgb_img.get_pixel(x, y + 1);
        //             if blanc {
        //                 let r = pixel_bas[0] as f32 + erreur * 0.5;
        //                 let g = pixel_bas[1] as f32 + erreur * 0.5;
        //                 let b = pixel_bas[2] as f32 + erreur * 0.5;
        //                 rgb_img.put_pixel(x, y + 1, image::Rgb([r as u8, g as u8, b as u8]));
        //             } else {
        //                 let r = pixel_bas[0] as f32 - erreur * 0.5;
        //                 let g = pixel_bas[1] as f32 - erreur * 0.5;
        //                 let b = pixel_bas[2] as f32 - erreur * 0.5;
        //                 rgb_img.put_pixel(x, y + 1, image::Rgb([r as u8, g as u8, b as u8]));
        //             }
        //         }
        //     });
        // }
    }
    rgb_img.save("../images/Question15.png")?;


    Ok(())
}
    
