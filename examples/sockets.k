socket = socket()

socket.bind("127.0.0.1", 8080)

print("Server started on localhost:8080")

connection = socket.accept()

while true
    msg = connection.recv(1024)

    print(msg)

    if msg.length() == 0
        print("Connection closed by client")
        break
    end
end
