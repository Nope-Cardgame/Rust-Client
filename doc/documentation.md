Der Client wird durch ein simples Konsolenmenü gesteuert und alle Züge werden automatisch ausgeführt.

Jegliche Zuglogik befindet sich in der [turn.rs](../src/logic/turn.rs).

Logik für die Authentifizierung per HTTP-Requests in der [authenticate.rs](../src/connect/authenticate.rs).
Verbindungsaufbau per SocketIO in der [connect.rs](../src/connect/connect.rs).
Alle Callbacks für die SocketIO events in der [events.rs](../src/connect/events.rs)
