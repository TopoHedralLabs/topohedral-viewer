

cargo run --features enable_trace -- d2 with-port 50051 &
sleep 5
grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1", "axes_descriptor": {"origin": {"x": 0, "y": 0}, "x_axis": {"x": 1, "y": 0}, "y_axis": {"x": 0, "y": 1}, "neg_len": 10, "pos_len": 10}}' localhost:50051 d2rpc.StateService/AddAxes
grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1", "square_descriptor": {"origin": {"x": 0, "y": 0}, "x_axis": {"x": -0.7071, "y": 0.7071}, "y_axis": {"x": 0, "y": 1}, "lenx": 1, "leny": 1, "line_color": {"r": 0, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d2rpc.StateService/AddSquare
grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1", "square_descriptor": {"origin": {"x": 0, "y": 0}, "x_axis": {"x": 1, "y": 0}, "y_axis": {"x": 0, "y": 1}, "lenx": 0.5, "leny": 0.5, "line_color": {"r": 1, "g": 0, "b": 0}, "tri_color": {"r": 0, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d2rpc.StateService/AddSquare
grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1", "circle_descriptor": {"center": {"x": 0, "y": 0}, "radius": 1, "num_sides": 100, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "LINE"}}' localhost:50051 d2rpc.StateService/AddCircle
grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1", "circle_descriptor": {"center": {"x": 4, "y": 4}, "radius": 1, "num_sides": 100, "line_color": {"r": 0, "g": 0, "b": 1}, "tri_color": {"r": 1, "g": 1, "b": 0}, "cell_type": "TRIANGLE"}}' localhost:50051 d2rpc.StateService/AddCircle
# grpcurl -proto protos/d2.proto -plaintext -d '{"client_name": "client1"}' localhost:50051 d2rpc.StateService/KillServer
