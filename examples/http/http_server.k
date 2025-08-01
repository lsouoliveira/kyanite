INDEX_PAGE = "
<!DOCTYPE html>
<html lang='en'>
<head>
  <meta charset='UTF-8'>
  <meta name='viewport' content='width=device-width, initial-scale=1'>
  <title>Hello from Kyanite</title>
  <link href='https://fonts.googleapis.com/css2?family=Fira+Code&family=Inter:wght@400;600&display=swap' rel='stylesheet'>
  <style>
    body {
      margin: 0;
      padding: 0;
      background: linear-gradient(to right, #1f2937, #111827);
      color: #e5e7eb;
      font-family: 'Inter', sans-serif;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      height: 100vh;
      text-align: center;
    }

    h1 {
      font-size: 3em;
      font-weight: 600;
      margin-bottom: 0.5em;
      color: #93c5fd;
    }

    p {
      font-size: 1.2em;
      color: #d1d5db;
      max-width: 500px;
    }

    .tagline {
      font-family: 'Fira Code', monospace;
      font-size: 1em;
      margin-top: 1em;
      background: rgba(255, 255, 255, 0.05);
      padding: 0.5em 1em;
      border-radius: 8px;
      color: #60a5fa;
    }

    .logo {
      width: 150px;
    }

    .footer-description {
      font-size: 0.8em;
      color: #6b7280;
      margin-top: 2em;
    }

    .footer-description a {
      color: #60a5fa;
      text-decoration: none;
      transition: color 0.3s;
    }

    .footer-description a:hover {
      color: #93c5fd;
    }
  </style>
</head>
<body>
  <img src='https://i.imgur.com/o8w0NyN.png' alt='Kyanite Logo' class='logo'>
  <h1>Hello from Kyanite</h1>
  <p>Kyanite is a toy programming language made for fun, experimentation, and learning.</p>
  <div class='tagline'>print('Hello, World!')</div>
  <p class='footer-description'>This website was built with Kyanite. The source code is available <a href='https://github.com/lsouoliveira/kyanite/blob/main/examples/http/http_server.k'>here</a>.</p>
</body>
</html>
"

class Request
    def constructor(method, path, headers)
        self.method = method
        self.path = path
        self.headers = headers
    end
end

class Response
    def constructor()
        self.status_code = 200
        self.body = ""
    end

    def set_status_code(code)
        self.status_code = code
    end

    def set_body(body)
        self.body = body
    end

    def build
        response = "HTTP/1.1 ".concat(self.status_code.__repr__()).concat(" OK\r\n")
        response = response.concat("Content-Type: text/html; charset=UTF-8\r\n")
        response = response.concat("Cache-Control: no-cache\r\n")
        response = response.concat("Content-Length: ").concat(self.body.length().__repr__()).concat("\r\n")
        response = response.concat("Connection: close\r\n")
        response = response.concat("\r\n")
        response = response.concat(self.body)

        return response
    end
end

class RequestLine
    def constructor(method, path, version)
        self.method = method
        self.path = path
        self.version = version
    end

    def __repr__
        return "RequestLine(method: '".concat(self.method).concat("', path: '").concat(self.path).concat("', version: '").concat(self.version).concat("')")
    end
end

def build_response(request)
    if request.path == "/"
        response = Response()
        response.set_status_code(200)
        response.set_body(INDEX_PAGE)
        response = response.build()

        return response
    end

    response = Response()
    response.set_status_code(404)
    response = response.build()

    return response
end

class RequestHandler
    def constructor(connection)
        self.connection = connection
    end

    def handle
        request = self.read_request()

        if request == None
            print("No request received. Closing connection.")
            self.connection.close()
            return
        end

        response = build_response(request)

        self.connection.send(response.encode())
        self.connection.close()

        print("Handled request: ".concat(request.method).concat(" ").concat(request.path))
    end

    def read_request
        msg = self.connection.recv(1024).decode()

        if msg == ""
            return None
        end

        lines = msg.split("\r\n")

        request_line = self.parse_request_line(lines.at(0))
        headers = self.parse_headers(lines.slice(1, lines.length() - 2))
        request = Request(request_line.method, request_line.path, headers)

        return request
    end

    def parse_request_line(line)
        parts = line.split(" ")

        if parts.length() != 3
            raise Exception("Invalid request line: ".concat(line))
        end

        method = parts.at(0)
        path = parts.at(1)
        version = parts.at(2)

        return RequestLine(method, path, version)
    end

    def parse_headers(lines)
        headers = Hash()

        i = 0

        while i < lines.length()
            line = lines.at(i)

            if line.length() != 0
                parts = line.split(": ")

                if parts.length() != 2
                    raise Exception("Invalid header line: ".concat(line))
                end

                key = parts.at(0).strip()
                value = parts.at(1).strip()

                headers.insert(key, value)
            end

            i = i + 1
        end

        return headers
    end
end

class HttpServer
    def constructor(host, port)
        self.host = host
        self.port = port
        self.socket = socket()
        self.connected = 0
    end

    def start
        self.socket.bind(self.host, self.port)

        print("Server started on http://".concat(self.host).concat(":").concat(self.port.__repr__()))

        while true
            conn = self.socket.accept()
            handler = RequestHandler(conn)

            thread = Thread(handler.handle)
            thread.start()
        end
    end
end

server = HttpServer("localhost", 8080)
server.start()
