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
use image::{DynamicImage, GenericImageView, ImageError};

// Exercice 7
fn calcule_luminosité(pixel: image::Rgb<u8>) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn main() -> Result<(), ImageError> {
    let img_path = "iut.jpg";
    let img = image::open(img_path)?;
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
    let luminosité = calcule_luminosité(*pixel32_52);
    println!("Luminosité du pixel (32, 52) : {}", luminosité);

    rgb_img.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
        let luminosité = calcule_luminosité(*pixel);
        if luminosité > 128.0 {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    });
    rgb_img.save("../images/Question7.png")?;

    Ok(())
}
    
