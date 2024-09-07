use topohedral_tracing::{info, init, topo_log};
use topohedral_viewer::app::locate_executable;
use topohedral_viewer::d3::PlaneDescriptor;
use topohedral_viewer::{d2, d2::Mesh2D, d3, d3::Mesh3D};
use topohedral_viewer::{CellType, Color, Vec2, Vec3};

use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn d2_rpc_test() {
    init().unwrap();

    let topoviewer_exec_result = locate_executable();
    if let Ok(topoviewer_exec) = topoviewer_exec_result {
        //{{{ com: launch server
        let _server_process = Command::new(topoviewer_exec)
            .arg("d2")
            .arg("with-port")
            .arg("50051")
            .spawn()
            .expect("Failed to start topoviewer");

        //{{{ trace
        info!("Sleeping for 2 seconds");
        //}}}
        sleep(Duration::from_millis(2000));
        //}}}
        //{{{ com: launch client
        //{{{ trace
        info!("Launching client");
        //}}}
        let mut client = d2::Client2D::new(50051).unwrap();
        //}}}
        //{{{ com: add axes
        //{{{ trace
        info!("Addding axes");
        //}}}
        let axes_id = client
            .add_axes(d2::AxesDescriptor {
                origin: Vec2::new(0.0, 0.0),
                x_axis: Vec2::new(1.0, 0.0),
                y_axis: Vec2::new(0.0, 1.0),
                neg_len: 0.5,
                pos_len: 0.5,
            })
            .unwrap();
        assert_eq!(axes_id, 5);
        //{{{ trace
        info!("axes_id: {}", axes_id);
        //}}}
        //}}}
        //{{{ com: add squares
        let square_id1 = client
            .add_square(d2::SquareDescriptor {
                origin: Vec2::new(0.1, 0.1),
                x_axis: Vec2::new(1.0, 0.0),
                y_axis: Vec2::new(0.0, 1.0),
                lenx: 0.1,
                leny: 0.1,
                line_color: Color::Red,
                tri_color: Color::Green,
                cell_type: CellType::Triangle,
            })
            .unwrap();
        assert_eq!(square_id1, 6);

        let square_id2 = client
            .add_square(d2::SquareDescriptor {
                origin: Vec2::new(0.3, 0.1),
                x_axis: Vec2::new(1.0, 0.0),
                y_axis: Vec2::new(0.0, 1.0),
                lenx: 0.1,
                leny: 0.1,
                line_color: Color::Red,
                tri_color: Color::Green,
                cell_type: CellType::Line,
            })
            .unwrap();
        assert_eq!(square_id2, 7);
        //{{{ trace
        info!("square_id1: {} square_id2: {}", square_id1, square_id2);
        //}}}
        //}}}
        //{{{ com: add circles
        let circle_id1 = client
            .add_circle(d2::CircleDescriptor {
                center: Vec2::new(-0.2, 0.2),
                radius: 0.1,
                num_sides: 30,
                line_color: Color::Red,
                tri_color: Color::Green,
                cell_type: CellType::Triangle,
            })
            .unwrap();

        assert_eq!(circle_id1, 8);

        let circle_id2 = client
            .add_circle(d2::CircleDescriptor {
                center: Vec2::new(-0.4, 0.2),
                radius: 0.1,
                num_sides: 30,
                line_color: Color::Red,
                tri_color: Color::Green,
                cell_type: CellType::Line,
            })
            .unwrap();

        assert_eq!(circle_id2, 9);

        //{{{ trace
        info!("circle_id1: {} circle_id2: {}", circle_id1, circle_id2);
        //}}}
        //}}}
        //{{{ com: add mesh
        let mut mesh = d2::Mesh::from_num_triangles(1);
        let v1 = Vec2::new(-0.1, -0.1);
        let v2 = Vec2::new(-0.2, -0.1);
        let v3 = Vec2::new(-0.1, -0.2);
        mesh.add_triangle(&v1, &v2, &v3, &Color::Black, &Color::Cyan);
        let mesh_id = client.add_mesh(mesh).unwrap();

        assert_eq!(mesh_id, 10);
        //{{{ trace
        info!("mesh_id: {}", mesh_id);
        //}}}
        //}}}
        //{{{ com: clear and kill server
        sleep(Duration::from_millis(5000));
        //{{{ trace
        info!("Clearing scene");
        //}}}
        client.clear().expect("Failed to clear");
        //{{{ trace
        info!("Killing server");
        //}}}
        client.kill_server().expect("Failed to kill server");
        //}}}
    }
}

#[test]
fn d3_rpc_test() {
    init().unwrap();

    let topoviewer_exec_result = locate_executable();
    if let Ok(topoviewer_exec) = topoviewer_exec_result {
        //{{{ com: launch server
        let _server_process = Command::new(topoviewer_exec)
            .arg("d3")
            .arg("with-port")
            .arg("50051")
            .spawn()
            .expect("Failed to start topoviewer");

        //{{{ trace
        info!("Sleeping for 2 seconds");
        //}}}
        sleep(Duration::from_millis(2000));
        //}}}
        //{{{ com: launch client
        //{{{ trace
        info!("Launching client");
        //}}}
        let mut client = d3::Client3D::new(50051).unwrap();
        //}}}
        //{{{ com: add axes
        //{{{ trace
        info!("Adding axes");
        //}}}
        let axes_id = client
            .add_axes(d3::AxesDescriptor {
                origin: Vec3::new(0.0, 0.0, 0.0),
                x_axis: Vec3::new(1.0, 0.0, 0.0),
                y_axis: Vec3::new(0.0, 1.0, 0.0),
                z_axis: Vec3::new(0.0, 0.0, 1.0),
                neg_len: 1000.0,
                pos_len: 1000.0,
            })
            .unwrap();

        assert_eq!(axes_id, 5);
        //}}}
        //{{{ com: add triangles
        //{{{ trace
        info!("Adding triangles");
        //}}}
        let tri_id1 = client.add_triangle(d3::TriangleDescriptor {
            v1: Vec3::new(0.0, 0.0, 0.0),
            v2: Vec3::new(1.0, 0.0, 0.0),   
            v3: Vec3::new(0.00, 1.0, 0.0),
            line_color: Color::Black,
            tri_color: Color::Red,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(tri_id1, 6);

        let tri_id2 = client.add_triangle(d3::TriangleDescriptor {
            v1: Vec3::new(0.0, 0.0, 1.0),
            v2: Vec3::new(1.0, 0.0, 1.0),   
            v3: Vec3::new(0.00, 1.0, 1.0),
            line_color: Color::Pink,
            tri_color: Color::Red,
            cell_type: CellType::Line,
        }).unwrap();
        assert_eq!(tri_id2, 7);
        //}}}
        //{{{ com: add planes
        //{{{ trace
        info!("Adding planes");
        //}}}
        let plane_id1 = client.add_plane(d3::PlaneDescriptor{
            origin: Vec3::new(0.0, 0.0, 2.0),
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            x_min: -0.5, 
            x_max: 0.5,
            y_min: -0.5,
            y_max: 0.5,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(plane_id1, 8);
        let plane_id2 = client.add_plane(d3::PlaneDescriptor{
            origin: Vec3::new(0.0, 0.0, 3.0),
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            x_min: -0.5, 
            x_max: 0.5,
            y_min: -0.5,
            y_max: 0.5,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Line,
        }).unwrap();
        assert_eq!(plane_id2, 9);
        //}}}
        //{{{ com: add cuboids
        let cuboid_id1 = client.add_cuboid(d3::CuboidDescriptor{
            origin: Vec3::new(2.0, 2.0, 2.0),
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            z_axis: Vec3::new(0.0, 0.0, 1.0),
            lenx: 1.0,
            leny: 1.0,
            lenz: 1.0,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(cuboid_id1, 10);

        let cuboid_id2 = client.add_cuboid(d3::CuboidDescriptor{
            origin: Vec3::new(5.0, 2.0, 2.0),
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            z_axis: Vec3::new(0.0, 0.0, 1.0),
            lenx: 1.0,
            leny: 1.0,
            lenz: 1.0,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Line,
        }).unwrap();
        assert_eq!(cuboid_id2, 11);
        //}}}
        //{{{ com: add cylinders
        let cyl_id1 = client.add_cylinder(d3::CylinderDescriptor{
            origin: Vec3::new(-2.0, -2.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            height: 3.0,
            num_sides: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            open: false,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(cyl_id1, 12);
        let cyl_id2 = client.add_cylinder(d3::CylinderDescriptor{
            origin: Vec3::new(-5.0, -2.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            height: 3.0,
            num_sides: 20,
            line_color: Color::Purple,
            tri_color: Color::Gray,
            open: false,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(cyl_id2, 13);
        let cyl_id3 = client.add_cylinder(d3::CylinderDescriptor{
            origin: Vec3::new(-2.0, -5.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            height: 3.0,
            num_sides: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            open: true,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(cyl_id3, 14);
        let cyl_id4 = client.add_cylinder(d3::CylinderDescriptor{
            origin: Vec3::new(-5.0, -5.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            height: 3.0,
            num_sides: 20,
            line_color: Color::Purple,
            tri_color: Color::Gray,
            open: true,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(cyl_id4, 15);
        //}}}
        //{{{ com: add discs
        let disc_id1 = client.add_disc(d3::DiscDescriptor{
            origin: Vec3::new(5.0, 0.0, 0.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            num_sides: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        }).unwrap();
        assert_eq!(disc_id1, 16);
        let disc_id2 = client.add_disc(d3::DiscDescriptor{
            origin: Vec3::new(6.0, 0.0, 0.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            num_sides: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Line,
        }).unwrap();
        assert_eq!(disc_id2, 17);
        //}}}
        //{{{ com: add spheres
        let sphere_id1 = client.add_sphere(d3::SphereDescriptor{
            origin: Vec3::new(0.0, -6.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0), 
            radius: 1.0,
            n_lat: 20, 
            n_long: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        }).unwrap();    
        assert_eq!(sphere_id1, 18);
        let sphere_id2 = client.add_sphere(d3::SphereDescriptor{
            origin: Vec3::new(0.0, -3.0, 0.0),
            axis: Vec3::new(0.0, 1.0, 0.0), 
            radius: 1.0,
            n_lat: 20, 
            n_long: 20,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Line,
        }).unwrap();    
        assert_eq!(sphere_id2, 19);
        //}}}
        //{{{ com: clear and kill server
        sleep(Duration::from_millis(10000));
        //{{{ trace
        info!("Clearing scene");
        //}}}
        client.clear().expect("Failed to clear");
        //{{{ trace
        info!("Killing server");
        //}}}
        client.kill_server().expect("Failed to kill server");
        //}}}
    }
}
