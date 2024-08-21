import os
import sys

# Add the parent directory to the system path so that the
# topohedral_viewer_py/client.py module can be imported
parent_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.append(parent_dir)

# Import the Client class from client.py
from .client import Client2D, launch_server