from grpc_tools import protoc

def generate_protos():
    proto_path = "../protos"
    proto_file = f"{proto_path}/d2.proto"
    python_out = "./topohedral_viewer_py"
    
    protoc.main((
        '',
        f'-I{proto_path}',
        f'--python_out={python_out}',
        f'--grpc_python_out={python_out}',
        proto_file,
    ))

if __name__ == "__main__":
    generate_protos()