import subprocess
import time
import os
import grpc
import logging as log

from .d2_pb2 import Vec2, AxesDescriptor
from . import d2_pb2_grpc


class Client:
    def __init__(self, dim: int = 2, port: int = 50051):
        self.server_executable_path = self._find_executable()
        self.server_process = None
        self.channel = None
        self.stub = None
        self.dim = dim
        self.port = port

    def _find_executable(self):
        # Locate the Rust executable within the package
        base_path = os.path.dirname(__file__)
        is_dev = os.getenv("TOPOHEDRAL_VIEWER_DEV")

        if is_dev:
            return os.path.join(base_path, '..', '..', '..', 'target', 'debug', 'topohedral-viewer-rpc')

        return os.path.join(base_path, '..', 'target', 'release', 'topohedral-viewer-rpc')

    def start_server(self, dim: int = 2, port: int = 50051):

        exec = [self._find_executable()]

        if dim == 2:
            exec.append("d2")
        elif dim == 3:
            exec.append("d3")
            
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



def launch_server(dim: int = 2, port: int = 50051):
    log.info('Launching server...')
    client = Client(dim, port)
    client.start_server()
    client.connect()    
    return client