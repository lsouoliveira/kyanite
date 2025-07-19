import socket


# connect to localhost 8080
def connect_to_server(host="localhost", port=8080):
    try:
        # Create a socket object
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        # Connect to the server
        sock.connect((host, port))
        print(f"Connected to {host}:{port}")

        return sock
    except socket.error as e:
        print(f"Error connecting to server: {e}")
        return None


connection = connect_to_server()

if connection:
    while True:
        try:
            msg = input("Enter message to send (or 'exit' to quit): ")

            if msg.lower() == "exit":
                print("Exiting...")
                connection.close()
                break

            connection.sendall(msg.encode("utf-8"))
        except socket.error as e:
            print(f"Error receiving data: {e}")
            break
        except KeyboardInterrupt:
            print("Interrupted by user, closing connection.")
            break
