import subprocess
import time
import os
import grpc
import logging as log

from .d2_pb2 import Vec2, AxesDescriptor
from . import d2_pb2_grpc


class Client2D:
    def __init__(self, port: int = 50051):
        self.server_executable_path = self._find_executable()
        self.server_process = None
        self.channel = None
        self.stub = None
        self.port = port

    def _find_executable(self):
        # Locate the Rust executable within the package
        base_path = os.path.dirname(__file__)
        is_dev = os.getenv("TOPOHEDRAL_VIEWER_DEV")

        if is_dev:
            return os.path.join(base_path, '..', '..', '..', 'target', 'debug', 'topohedral-viewer-rpc')

        return os.path.join(base_path, '..', 'target', 'release', 'topohedral-viewer-rpc')

    def start_server(self, port: int = 50051):

        exec = [self._find_executable()]
        exec.append("d2")
        exec.append("with-port")
        exec.append(str(port))
        self.server_process = subprocess.Popen([self.server_executable_path])

    def stop_server(self):
        if self.server_process:
            self.server_process.terminate()
            self.server_process.wait()

    def connect(self):
        port = self.port
        self.channel = grpc.insecure_channel(f'localhost:{port}')
        self.stub = d2_pb2_grpc.StateServiceStub(self.channel)

    def add_axes(self, axes_descriptor: AxesDescriptor):    
        return self.stub.AddAxes(axes_descriptor)

    def add_square(self, square_descriptor):
        return self.stub.AddSquare(square_descriptor)

    def add_circle(self, circle_descriptor):
        return self.stub.AddCircle(circle_descriptor)
    
    def add_global_axes(self):
        origin = Vec2(x = 0, y = 0)
        x_axis = Vec2(x = 1, y = 0)
        y_axis = Vec2(x = 0, y = 1)
        request = AxesDescriptor(
            origin = origin, 
            x_axis = x_axis, 
            y_axis = y_axis, 
            pos_len = 100, 
            neg_len = 100
        )  
        return self.add_axes(request)



def launch_server(dim: int = 2, port: int = 50051):
    log.info('Launching server...')

    client = None
    if dim == 2:
        client = Client2D(dim, port)
        client.start_server()
        client.connect()    
    elif dim == 3:
        pass
        # client = Client3D(dim, port)
    return client