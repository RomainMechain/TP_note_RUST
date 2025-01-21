use argh::FromArgs;

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
    #[argh(option)]
    color1: String,

    /// la seconde couleur
    #[argh(option)]
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
}

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
    }

    rgb_img.save("../images/Question8.png")?;

    Ok(())
}
    
