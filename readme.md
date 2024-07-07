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
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "room_name": "Testing Room!",
    "max_size": "DOUBLE_PLAYERS",
    "owner_name": "KimWang906"
}' \
'[::1]:50051' room.RoomManager/CreateRoom
```

### Destroy Room

```bash
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "id": 1
}' \
'[::1]:50051' room.RoomManager/DeleteRoom
```

## Room List

```bash
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' '[::1]:50051' room.RoomManager/ListRooms
```

## Watching Room

```bash
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "id": 1
}' \
'[::1]:50051' room.RoomManager/WatchRoomInfo
```

## Join Room

```bash
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' -d \
'{
    "id": 1
}' \
'[::1]:50051' room.RoomUserManager/JoinRoom
```

## Leave Room

```bash
grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: <access_token>' -d \
'{}' '[::1]:50051' room.RoomUserManager/LeaveRoom
```



grpcurl -plaintext -import-path ./proto -proto room.proto \
-H 'x-authorization: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOiIzNjAwIiwiaWF0IjoiMTcxODA4NzU0MiIsInN1YiI6IjIwMjIwMjgifQ.obsh6jsoGc79wQSamO7n8GygEjzvYTUt5efV3sKazbw' -d \
'{
    "room_name": "Testing Room!!!!!",
    "max_size": "DOUBLE_PLAYERS",
    "owner_name": "KimWang906"
}' \
'[::1]:50051' room.RoomManager/CreateRoom