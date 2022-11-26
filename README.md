# actix-web-and-yew-test
First start backend with:
```
$ cd backend
$ cargo run
```
Backend will be available at http://localhost:8000

Then start frontend with:
```
$ cd frontend
$ trunk serve --proxy-backend=http://localhost:8000/students
```
Frontend will be available at http://localhost:8080

Go to http://localhost:8080. The list of displayed students is coming from a dummy database in the backend.