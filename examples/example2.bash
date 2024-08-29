cargo run --features enable_trace -- d3 with-port 50051 &
sleep 5

# Add axes
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "axes_descriptor": {"origin": {"x": 0, "y": 0, "z": 0}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "z_axis": {"x": 0, "y": 0, "z": 1}, "neg_len": 1000, "pos_len": 1000}}' localhost:50051 d3rpc.StateService/AddAxes

# Add a triangle
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "triangle_descriptor": {"v1": {"x": 0, "y": 0, "z": 0}, "v2": {"x": 1, "y": 0, "z": 0}, "v3": {"x": 0, "y": 1, "z": 0}, "line_color": {"r": 0, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddTriangle

# Add a plane
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "plane_descriptor": {"origin": {"x": 1, "y": 1, "z": 1}, "x_axis": {"x": 0, "y": 0, "z": 1}, "y_axis": {"x": 0, "y": 1, "z": 0}, "x_min": -5, "x_max": 5, "y_min": -5, "y_max": 5, "line_color": {"r": 1, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d3rpc.StateService/AddPlane

# Add a cuboid
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cuboid_descriptor": {"origin": {"x": 5, "y": 5, "z": 5}, "x_axis": {"x": 1, "y": 0, "z": 0}, "y_axis": {"x": 0, "y": 1, "z": 0}, "z_axis": {"x": 0, "y": 0, "z": 1}, "lenx": 2, "leny": 3, "lenz": 4, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d3rpc.StateService/AddCuboid

# Add a cylinder
grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "cylinder_descriptor": {"origin": {"x": -5, "y": -5, "z": -5}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 3, "num_sides": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "open": false}}' localhost:50051 d3rpc.StateService/AddCylinder

# Add a sphere 
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1", "sphere_descriptor": {"origin": {"x": 0, "y": 0, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 2, "n_lat": 20, "n_long": 20, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}}}' localhost:50051 d3rpc.StateService/AddSphere

# Clear the scene
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1"}' localhost:50051 d3rpc.StateService/clear

# Kill the server
# grpcurl -proto protos/d3.proto -plaintext -d '{"client_name": "client1"}' localhost:50051 d3rpc.StateService/KillServer
