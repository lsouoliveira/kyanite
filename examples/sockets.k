socket = socket()

socket.bind("127.0.0.1", 8080)

print("Server started on localhost:8080")

connection = socket.accept()
