use image::{ImageBuffer, Pixel, Rgb};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn calculate_distance(p_1: &Rgb<u8>, p_2: &Rgb<u8>) -> u16 {
    let colors_1 = Rgb::channels(&p_1);
    let colors_2 = Rgb::channels(&p_2);
    let diff_r = (colors_1[0] as i16 - colors_2[0] as i16) as f64;
    let diff_g = (colors_1[1] as i16 - colors_2[1] as i16) as f64;
    let diff_b = (colors_1[2] as i16 - colors_2[2] as i16) as f64;
    f64::sqrt(diff_r * diff_r + diff_g * diff_g + diff_b * diff_b) as u16
}

fn calculate_centroid(cluster: &Vec<&Rgb<u8>>) -> Rgb<u8> {
    let mut centroid: [u32; 3] = [0, 0, 0];
    let mut counter = 1;
    for point in cluster.iter() {
        centroid[0] += point[0] as u32;
        centroid[1] += point[1] as u32;
        centroid[2] += point[2] as u32;
        counter += 1;
    }
    let mut final_centroid: [u8; 3] = [0, 0, 0];
    final_centroid[0] = (centroid[0] / counter) as u8;
    final_centroid[1] = (centroid[1] / counter) as u8;
    final_centroid[2] = (centroid[2] / counter) as u8;
    Rgb::from_slice(&final_centroid).to_owned()
}

fn calculate_centroids(clusters: &Vec<Vec<&Rgb<u8>>>) -> Vec<Rgb<u8>> {
    let mut new_centroids = Vec::new();
    for cluster in clusters.iter() {
        new_centroids.push(calculate_centroid(&cluster));
    }
    new_centroids
}

fn find_closest_index(centroids: &Vec<Rgb<u8>>, pixel: &Rgb<u8>) -> usize {
    let mut distance = u16::MAX;
    let mut minimum = 0;
    for (index, centroid) in centroids.iter().enumerate() {
        let current_distance = calculate_distance(pixel, centroid);
        if current_distance < distance {
            minimum = index;
            distance = current_distance;
        }
    }
    minimum
}

fn clustering(
    k: usize,
    initial_centroids: Vec<Rgb<u8>>,
    pixels: image::buffer::Pixels<Rgb<u8>>,
) -> Vec<Rgb<u8>> {
    let mut clusters: HashMap<&usize, Vec<&Rgb<u8>>> = HashMap::new();
    let indexes: Vec<usize> = (0..k).collect();
    let mut converged = false;
    let mut centroids: Vec<Rgb<u8>> = initial_centroids.clone();
    while !converged {
        for index in indexes.iter() {
            clusters.insert(index, Vec::new());
        }
        for pixel in pixels.clone() {
            let minimum = find_closest_index(&centroids, &pixel);
            clusters.get_mut(&minimum).unwrap().push(pixel);
        }
        let new_centroids =
            calculate_centroids(&clusters.values().cloned().collect::<Vec<Vec<&Rgb<u8>>>>());
        let set_new_centroids: HashSet<Rgb<u8>> = HashSet::from_iter(new_centroids.clone());
        let set_centroids: HashSet<Rgb<u8>> = HashSet::from_iter(centroids);
        converged = set_new_centroids == set_centroids;
        centroids = new_centroids;
    }

    centroids
}

fn lbg(codebook_size: usize, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut initial_centroids = vec![Rgb([0, 0, 0]); codebook_size];
    for (index, pixel) in img.pixels().take(codebook_size).enumerate() {
        initial_centroids[index] = pixel.clone();
    }
    let centroids = clustering(codebook_size, initial_centroids, img.pixels());

    let (width, height) = img.dimensions();
    for y in 0..width {
        for x in 0..height {
            let pixel = img.get_pixel(y, x);
            let index = find_closest_index(&centroids, pixel);
            img.put_pixel(y, x, centroids[index])
        }
    }
}

pub fn quantize_image(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    n_colors: usize,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img_rgb = img.clone();
    lbg(n_colors, &mut img_rgb);
    img_rgb
}
