use topohedral_tracing::{info, init, topo_log};
use topohedral_viewer::app::locate_executable;
use topohedral_viewer::d2::{self, Mesh2D};
use topohedral_viewer::{CellType, Color, Vec2};

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
        //{{{ trace
        info!("square_id1: {} square_id2: {}", square_id1,  square_id2);
        //}}}
        //}}}
        //{{{ com: add circles
        let circle_id1 = client.add_circle(d2::CircleDescriptor{
            center: Vec2::new(-0.2, 0.2),
            radius: 0.1,
            num_sides: 30,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        }).unwrap();

        let circle_id2 = client.add_circle(d2::CircleDescriptor{
            center: Vec2::new(-0.4, 0.2),
            radius: 0.1,
            num_sides: 30,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Line,
        }).unwrap();

        //{{{ trace
        info!("circle_id1: {} circle_id2: {}", circle_id1,  circle_id2);
        //}}}
        //}}}
        //{{{ com: add mesh
        let mut mesh = d2::Mesh::from_num_triangles(1);
        let v1 = Vec2::new(-0.1, -0.1);
        let v2 = Vec2::new(-0.2, -0.1);
        let v3 = Vec2::new(-0.1, -0.2);
        mesh.add_triangle(&v1, &v2, &v3, &Color::Black, &Color::Cyan);
        let mesh_id = client.add_mesh(mesh).unwrap();
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
