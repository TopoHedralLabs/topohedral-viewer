cargo run --features enable_trace -- d3 with-port 50051 &
sleep 5

# Add axes
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "axes_descriptor": {"origin": {"x": 0, "y": 0, "z": 0}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "z_axis": {"x": 0, "y": 0, "z": 1}, "neg_len": 1000, "pos_len": 1000}}' localhost:50051 d3rpc.StateService/AddAxes

# Add a triangles
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "triangle_descriptor": {"v1": {"x": 0, "y": 0, "z": 0}, "v2": {"x": 1, "y": 0, "z": 0}, "v3": {"x": 0, "y": 1, "z": 0}, "line_color": {"r": 0, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddTriangle
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "triangle_descriptor": {"v1": {"x": 0, "y": 0, "z": 1}, "v2": {"x": 1, "y": 0, "z": 1}, "v3": {"x": 0, "y": 1, "z": 1}, "line_color": {"r": 0, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddTriangle

# Add a plane
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "plane_descriptor": {"origin": {"x": 0, "y": 0, "z": 2}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "x_min": -0.5, "x_max": 0.5, "y_min": -0.5, "y_max": 0.5, "line_color": {"r": 1, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddPlane
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "plane_descriptor": {"origin": {"x": 0, "y": 0, "z": 3}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "x_min": -0.5, "x_max": 0.5, "y_min": -0.5, "y_max": 0.5, "line_color": {"r": 1, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddPlane

# Add a cuboid
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cuboid_descriptor": {"origin": {"x": 2, "y": 2, "z": 2}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "z_axis": {"x": 0, "y": 0, "z": 1}, "lenx": 1, "leny": 1, "lenz": 1, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddCuboid
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cuboid_descriptor": {"origin": {"x": 5, "y": 2, "z": 2}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "z_axis": {"x": 0, "y": 0, "z": 1}, "lenx": 1, "leny": 1, "lenz": 1, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddCuboid

# Add a cylinder
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cylinder_descriptor": {"origin": {"x": -2, "y": -2, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 3, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE", "open": true}}' localhost:50051 d3rpc.StateService/AddCylinder
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cylinder_descriptor": {"origin": {"x": -5, "y": -2, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 3, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE", "open": true}}' localhost:50051 d3rpc.StateService/AddCylinder
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cylinder_descriptor": {"origin": {"x": -2, "y": -5, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 3, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE", "open": false}}' localhost:50051 d3rpc.StateService/AddCylinder
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cylinder_descriptor": {"origin": {"x": -5, "y": -5, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 3, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE", "open": false}}' localhost:50051 d3rpc.StateService/AddCylinder

# add a disc
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "disc_descriptor": {"origin": {"x": 5, "y": 0, "z": 0}, "axis": {"x": 1, "y": 0, "z": 0}, "radius": 1, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddDisc    
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "disc_descriptor": {"origin": {"x": 6, "y": 0, "z": 0}, "axis": {"x": 1, "y": 0, "z": 0}, "radius": 1, "num_sides": 20, "line_color": {"r": 1, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddDisc    

# Add a sphere 
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "sphere_descriptor": {"origin": {"x": 0, "y": -6, "z": 0}, "axis": {"x": 0, "y": 1, "z": 0}, "radius": 1, "n_lat": 50, "n_long": 50, "line_color": {"r": 1, "g": 1, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddSphere
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "sphere_descriptor": {"origin": {"x": 0, "y": -3, "z": 0}, "axis": {"x": 0, "y": 1, "z": 0}, "radius": 1, "n_lat": 10, "n_long": 50, "line_color": {"r": 1, "g": 1, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddSphere

# Clear the scene
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1"}' localhost:50051 d3rpc.StateService/clear

# Kill the server
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1"}' localhost:50051 d3rpc.StateService/KillServer
