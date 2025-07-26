lock = Lock()

lock.acquire()
lock.release()

socket = socket()
socket.bind("localhost", 8080)
print("Server is listening on port 8080")
connection = socket.accept()

msg = connection.recv(1024)

print("Received message: ", msg)
