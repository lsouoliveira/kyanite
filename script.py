# import socket
#
#
# # connect to localhost 8080
# def connect_to_server(host="localhost", port=8080):
#     try:
#         # Create a socket object
#         sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
#
#         # Connect to the server
#         sock.connect((host, port))
#         print(f"Connected to {host}:{port}")
#
#         return sock
#     except socket.error as e:
#         print(f"Error connecting to server: {e}")
#         return None
#
#
# connection = connect_to_server()
#
# if connection:
#     connection.send(b"Hello, Server!")

a
