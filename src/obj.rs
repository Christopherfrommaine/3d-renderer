use tobj;

pub fn get_model(filename: &str) -> Vec<[[f64; 3]; 3]> {
    let (models, _) = tobj::load_obj(String::from("obj/") + filename, &tobj::LoadOptions::default()).unwrap();
    let mesh = &models[0].mesh;
    let positions = &mesh.positions;
    let indices = &mesh.indices;

    let triangles: Vec<[[f64; 3]; 3]> = indices
        .chunks(3)
        .map(|i| [
            [
                positions[3 * i[0] as usize] as f64,
                positions[3 * i[0] as usize + 1] as f64,
                positions[3 * i[0] as usize + 2] as f64,
            ],
            [
                positions[3 * i[1] as usize] as f64,
                positions[3 * i[1] as usize + 1] as f64,
                positions[3 * i[1] as usize + 2] as f64,
            ],
            [
                positions[3 * i[2] as usize] as f64,
                positions[3 * i[2] as usize + 1] as f64,
                positions[3 * i[2] as usize + 2] as f64,
            ],
        ])
        .collect();

    triangles
}
