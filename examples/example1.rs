
use d3::AxesDescriptor;
use topohedral_viewer::{d3::State3D, *};  



fn main()
{
    let mut state = d3::State::new();

    state.add_axes(&AxesDescriptor {
        origin: Vec3::new(0.0, 0.0, 0.0),
        x_axis: Vec3::new(1.0, 0.0, 0.0),
        y_axis: Vec3::new(0.0, 1.0, 0.0),
        z_axis: Vec3::new(0.0, 0.0,1.0), 
        neg_len: 10.0,
        pos_len: 10.0,
    });


    state.add_cylinder(&d3::CylinderDescriptor {
        origin: Vec3::new(-1.0, -1.0, 0.0),
        axis: Vec3::z(),
        radius: 1.0,
        height: 2.0,
        num_sides: 10,
        line_color: Color::White,
        tri_color: Color::Red,
        open: true,
    });
    state.add_cuboid(&d3::CuboidDescriptor {
        origin: Vec3::new(0.1, 0.1, 0.0), 
        x: Vec3::new(1.0, 1.0, 0.0), 
        y: Vec3::y(), 
        z: Vec3::z(),
        lenx: 1.0, 
        leny: 1.0,
        lenz: 1.0, 
        line_color: Color::White,
        tri_color: Color::Green,
    });

    state.add_plane(&d3::PlaneDescriptor { origin: Vec3::new(0.0, 0.0, 5.0), 
                                       x_axis: Vec3::x(), 
                                       y_axis: Vec3::y(), 
                                       x_min: -1.0, 
                                       x_max: 1.0, 
                                       y_min: -1.0, 
                                       y_max: 1.0, 
                                       line_color: Color::Black, 
                                       tri_color: Color::Blue });


    // pollster::block_on(run_minimal_3d(state));
}