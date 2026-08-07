from http.server import HTTPServer, BaseHTTPRequestHandler

class C2Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        print(f"[C2 RECEIVED POST] {self.path} data: {post_data.decode('utf-8')}")
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"OK")
        
    def do_GET(self):
        print(f"[C2 RECEIVED GET] {self.path}")
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"OK")

if __name__ == '__main__':
    server = HTTPServer(('0.0.0.0', 8000), C2Handler)
    print("C2 Server listening on port 8000...")
    server.serve_forever()
