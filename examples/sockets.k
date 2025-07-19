class Server
    def constructor(host, port)
        self.host = host
        self.port = port
        self.socket = socket()
    end

    def start
        self.socket.bind(self.host, self.port)

        print("Server started on ", self.host, ":", self.port)
        
        self.accept_connection()
    end

    def accept_connection
        print("Waiting for a connection...")

        connection = self.socket.accept()

        self.process_connection(connection)
    end

    def process_connection(connection)
        while true
            msg = connection.recv(1024)

            if msg.length() == 0
                break
            end

            print("Received message: ", msg)
        end
    end
end

server = Server("localhost", 8080)
server.start()
