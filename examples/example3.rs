
use topohedral_tracing::{topo_log, info, init};
use topohedral_viewer::{Vec2, Color, CellType};
use topohedral_viewer::d2::{self, Mesh2D};



fn main()
{
    init().unwrap();

    let mut client = d2::Client2D::new(50051).unwrap();

    let axes_id = client.add_axes(d2::AxesDescriptor{
        origin: Vec2::new(0.0, 0.0),
        x_axis: Vec2::new(1.0, 0.0),
        y_axis: Vec2::new(0.0, 1.0),
        neg_len: 0.5,
        pos_len: 0.5,
    }).unwrap();

    let square_id = client.add_square(d2::SquareDescriptor{
        origin: Vec2::new(0.1, 0.1),
        x_axis: Vec2::new(1.0, 0.0),
        y_axis: Vec2::new(0.0, 1.0),
        lenx: 0.3,
        leny: 0.3,
        line_color: Color::Red,
        tri_color: Color::Green,
        cell_type: CellType::Triangle,
    }).unwrap();

    let mut mesh = d2::Mesh::from_num_triangles(1);

    let v1 = Vec2::new(-6.0, 0.0);
    let v2 = Vec2::new(-5.0, 0.0);
    let v3 = Vec2::new(-6.0, 1.0);
    mesh.add_triangle(&v1, &v2, &v3, &Color::Black, &Color::Cyan);
    let mesh_id = client.add_mesh(mesh).unwrap();

    //{{{ trace
    info!(target: "example3", "axes_id = {}", axes_id);
    info!(target: "example3", "square_id = {}", square_id);
    info!(target: "example3", "mesh_id = {}", mesh_id);
    //}}}
}