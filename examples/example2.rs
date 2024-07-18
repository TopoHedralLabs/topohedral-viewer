
use topohedral_viewer::{d2::{AxesDescriptor, SquareDescriptor, State2D}, *};


fn main()
{

    let mut state = d2::State::new();

    state.add_axes(&AxesDescriptor{
        origin: Vec2::new(0.0, 0.0),
        x_axis: Vec2::new(1.0, 0.0),
        y_axis: Vec2::new(0.0, 1.0),
        neg_len: 0.5,
        pos_len: 0.5,
    });

    state.add_square(&SquareDescriptor{
        origin: Vec2::new(0.1, 0.1),
        x_axis: Vec2::new(1.0, 0.0),
        y_axis: Vec2::new(0.0, 1.0),
        lenx: 0.3,
        leny: 0.3,
        line_color: Color::Red,
        tri_color: Color::Green,
        cell_type: CellType::Triangle,
    });

    // pollster::block_on(run_minimal_2d(state));

    
}