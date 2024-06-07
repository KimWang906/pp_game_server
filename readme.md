# BSSM Ping Pong Server

## Client Test

### Register

```bash
grpcurl -plaintext -import-path ./proto -proto auth.proto -d \
'{
    "username": "KimWang906",
    "password": "test1234",
    "student_id": "2022028"
}' \
'[::1]:50051' auth.Auth/Register
```

### Login

```bash
grpcurl -plaintext -import-path ./proto -proto auth.proto -d \
'{
    "username": "KimWang906",
    "password": "test1234"
}' \
'[::1]:50051' auth.Auth/Login
```

### Create Room

```bash
grpcurl -plaintext -import-path ./proto -proto pp_api.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "room_name": "Testing Room!",
    "max_size": "TWO_PLAYERS"
    "owner_name": "KimWang906",
}' \
'[::1]:50051' pp_api.RoomService/Create
```

### Destroy Room

```bash
grpcurl -plaintext -import-path ./proto -proto pp_api.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "room_id": 1,
    "token": {
        "access_token": <room_access_token>
    }
}' \
'[::1]:50051' pp_api.RoomService/Destroy
```

## Room List

```bash
grpcurl -plaintext -import-path ./proto -proto pp_api.proto \
-H 'x-authorization: <access_token>' '[::1]:50051' pp_api.RoomService/List
```
