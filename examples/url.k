url = Url.parse("https://example.com/path?query=123#fragment")

print("scheme: " + url.scheme())
print("host: " + url.host())
print("port: ", url.port())
print("path: " + url.path())
print("query: " + url.query())
