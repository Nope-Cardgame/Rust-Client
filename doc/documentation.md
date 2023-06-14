# Genereller Aufbau
Der Code wurde in drei Module unterteilt:
* [connect](../src/connect): Authentifizierung und SocketIO Verbindung
* [logic](../src/logic): Spiellogik
* [menu](../src/menu): Konsolenmenü

Da zur Kommunikation zwischen Client und Server JSON benutzt wird, kommt in dieser Implementierung das Crate "serde_json"
benutzt. Dieses stellt eine API zur Verarbeitung von JSON-Strings bereit.

Die [hier](https://github.com/Nope-Cardgame/Doku/blob/main/Schnittstellen/Schnittstellen.md) definierten JSON-Objekte
wurden in der Datei [game_objects.rs](../src/logic/game_objects.rs) als structs angelegt und per serde_json eingehende JSON-Strings zu diesen structs umgewandelt, bzw. umgekehrt
für ausgehende Kommunikation.

Jegliche Zuglogik befindet sich in der Datei [turn.rs](../src/logic/turn.rs).

Logik für die Authentifizierung per HTTP-Requests in der Datei [authenticate.rs](../src/connect/authenticate.rs).
Verbindungsaufbau per SocketIO in der Datei [connect.rs](../src/connect/connect.rs).
Alle Callbacks für die SocketIO events in der Datei [events.rs](../src/connect/events.rs)

In den Dateien [.env](../.env) und [.alt.env](../.alt.env) werden nötige Verbindungsdaten als globale Variablen gehalten.
Diese werden im Client durch das crate dotenvy eingelesen.

---

# Authentifizierung
Die initiale Authentifizierung wird über einen HTTP-Request geregelt.
Hierzu gibt es zwei Möglichkeiten:
* [sign_up()](../src/connect/authenticate.rs): Falls der Spieler noch nicht auf dem Server bekannt ist, kann sich dieser registrieren und erhält sofort einen gültigen JSON-Webtoken
* [sign_in()](../src/connect/authenticate.rs): Der Spieler ist auf dem Server schon bekannt und kann sich mit seinen Logindaten der Registrierung anmelden. Man erhält sofort einen gültigen JSON-Webtoken

Für die weitere Spielabwicklung wird eine Verbindung per SocketIO benötigt.
Hierzu wird der erhaltene JSON-Webtoken der Funktion [upgrade_socket(token)](../src/connect/connect.rs) mitgegeben.
Von der Funktion erhält man den nutzbaren Socket.
Da es per crate rust_socketio keine Möglichkeit gibt, dem Socket nachträglich Callbacks für events zu definieren, müssen
alle Callbacks in der upgrade_socket Funktion mitgegeben werden.

---

# Spielablauf
Der Client wird durch ein simples Konsolenmenü gesteuert und alle Züge werden automatisch ausgeführt.
Initial kann der Benutzer entscheiden ob der normale, oder ein alternativer Login benutzt werden soll.
Dadurch können Spiele zwischen zwei Instanzen dieses Clients gestartet werden.

Darauffolgend wird der Nutzer durch die verschiedenen Möglichkeiten der Spiele geführt.
Dabei kann entschieden werden ob:
* ein Einzelspiel gespielt werden soll
  * auf eine Einladung gewartet werden soll
  * ein neues Spiel erstellt werden soll
* ein Turnierspiel gespielt werden soll

Bei der Einladung zu einem Einzelspiel wird der Benutzer über eine eingehende Einladung und dessen Sender informiert.
Darauf kann der Benutzer entscheiden, ob die Einladung angenommen werden soll

Beim Erstellen eines Einzelspiels bekommt der Nutzer eine Liste aller verbundener Spieler ausgegeben.
Der Benutzer kann entscheiden, wem eine Einladung gesandt wird.

Wenn auf eine Turniereinladung gewartet wird, wird diese sofort, ohne Frage angenommen.

Sobald ein Turnier, oder Einzelspiel, angenommen wurde, läuft die weitere Spieleabwicklung automatisch und
erst nachdem das Spiel, oder Turnier, beendet wurde kann der Benutzer wieder Eingaben tätigen.

---

# Logik
Die Logikkaskade wird durch den Eventcallback [game_state_callback](../src/connect/events.rs) ausgelöst, sobald ein
neuer Spielstand empfangen wird und der Benutzer der aktive Spieler ist.

Danach werden in der Funktion [ai_turn()](../src/logic/turn.rs) die gespielte Karte und die regelkonformen Möglichkeiten
durchgegangen.

Generell wird der Client, wenn möglich, eine Aktionskarte spielen, bevor Zahlenkarten abgeworfen werden.
Wenn keine spielbaren Aktionskarten auf der Hand sind, werden die spielbaren Zahlenkarten so sortiert, dass zuerst Karten mit mehreren Farben gespielt werden.
Nur allerletzt, werden Zahlenkarten mit einer Farbe gespielt. Wenn mehrere abgeworfen werden müssen, werden je nach Anzahl
der Karten auf der gegnerischen Hand entweder hohe, oder niedrige Werte oben aufgelegt.
Somit soll vermieden werden, dass man den Gegner 3 Karten abwerfen lassen möchte, obwohl dieser nur noch 2 Karten auf der Hand hat.