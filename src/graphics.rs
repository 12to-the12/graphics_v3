fn render() {
    process_geometry();
    rasterize();
}


fn process_geometry() {
    project();

    clip();
    project_to_2d();
    convert_to_screen_coordinates();
}





fn project() {
    //
    convert_to_camera_coordinates();
    homogenize_coordinates();
    



}

fn convert_to_camera_coordinates() {
    // turns world coordinates into camera coordinates
}

fn homogenize_coordinates() {
    // this function takes camera coordinates and divides by Z to get a normalized result
    // returns a vector of points in 3d space within a unit cube
}

fn project_to_2d() {
    // simple, takes unit cube as input and returns a 2d projection obtained by discarding the Z coordinate
    // returns 2d coordinates in a unit square
}

fn convert_to_screen_coordinates(coordinates) {
    // turns ephemeral normalized coordinates into something that can be projected on the screen or saved to an image file
    // receives and returns 2d coordinates

}

fn clip() {
    // culling: discards polygons with all vertexes outside the unit cube
    // clipping: clips polygons with any but not all vertexes outide the unit cube
}


fn rasterize() {
    // converts vector primitves to a bitmap
    rasterize_wireframe();

}

fn rasterize_wireframe() {
    // rasterizes a wireframe with 2d coordinates and references to them
    // takes 2d coordinates and returns a raster
    for line_segment in line_segments {

    }
}