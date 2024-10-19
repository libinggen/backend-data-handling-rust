# backend-data-handling-rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update


CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    quantity INT
);


cargo run

curl -X POST http://localhost:8000/items -H "Content-Type: application/json" -d '{"name": "item1", "quantity": 10}'

curl http://localhost:8000/items

curl -X PUT http://localhost:8000/items/1 -H "Content-Type: application/json" -d '{"name": "item2", "quantity": 15}'

curl -X DELETE http://localhost:8000/items/1
```